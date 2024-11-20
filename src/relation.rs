use bytebuffer::ByteBuffer;
use crate::buffer::Buffer;
use crate::col_info::ColInfo;
use crate::page:: PageId;
use crate::record::Record;
use std::cell:: RefCell;
use crate::buffer_manager::BufferManager;
use crate::record_id::RecordId;
use std::rc::Rc;

pub struct Relation<'a> { //PERSONNE(NOM,PRENOM?,AGE)
    name:String,
    columns: Vec<ColInfo>,
    nb_columns: usize,
    //TP5
    buffer_manager: Rc<RefCell<BufferManager<'a>>>,
    header_page_id : PageId  //id de la header page
    
}

impl<'a> Relation<'a> {

    pub fn new(name: String, columns: Vec<ColInfo>, bfm: Rc<RefCell<BufferManager<'a>>>) -> Self {
        let tmp = columns.len();

        //On appelle 'alloc_page' avant de déplacer 'buffer_manager' sinon ça fais des chinoiseries
        let header_page_id = bfm.borrow_mut().get_disk_manager_mut().alloc_page();

        //bon là c'est expérimental on va dire, j'ai mis un scope pour pas géner le constructeur mais ça se trouve ça fonctionne pas du tout
        {
        let mut bfmr = bfm.borrow_mut();
        let _ = bfmr.get_page(&header_page_id).write_int(0, 0);
        bfmr.free_page(&header_page_id, true);
        bfmr.flush_buffers();
        }

        //ça c'est une autre version de ce qu'il y a plus haut, parce que pourquoi pas
        /*
        bfm.borrow_mut().get_page(&header_page_id).write_int(0, 0);
       
        bfm.borrow_mut().free_page(&header_page_id, true);
      
        bfm.borrow_mut().flush_buffers();
       */

        Relation {
            name: String::from(name),
            columns,
            nb_columns: tmp,
            buffer_manager: bfm,
            header_page_id,
        }
    }

    pub fn get_name(&self)->&String {
        &self.name
    }

    // Getter pour les informations sur les colonnes
    pub fn get_columns(&self) -> Vec<ColInfo> {
        self.columns.clone()
    }

    fn get_header_page_id (&self) -> &PageId {
        return &self.header_page_id
    }

    pub fn write_record_to_buffer(& self, record:Record, buffer:&mut Buffer, pos:usize)->usize{
        // Copie du tuple (pas obligatoire)
        let tuple = record.get_tuple().clone();
        //Pour avoir le nom des colonnes, le type etc...
        let mut compteur:usize=0;
        
        let mut indice:usize = pos; //pour la position

        // Initialisation de la taille d'un BUFFER
        

        // Pour savoir si une colonne type  varchar a été trouvé dans le record
        let mut varchar_trouve:bool=false;

        // Recherche d'un ou plusieurs VARCHAR dans le tuple.
        for i in 0..self.columns.len(){
            if self.columns[i].get_column_type().starts_with("VARCHAR"){
                //println!("{:?}", self.columns[i].get_name());
                varchar_trouve=true;
                break;
            }      
        }
        //println!("{:?}", varchar_trouve);

        let mut taille_objets:Vec<usize> =Vec::new();
        
        
        if varchar_trouve{
            // Stockage des longueurs en octet de chaque attribut
            //pour faire les offset après
            for i in 0..tuple.len(){
                match self.columns[i].get_column_type().as_str() {
                    "INT" => {
                        taille_objets.push(4);
                        compteur+=4;
                        continue;
                    }
                    "REAL" => {
                        taille_objets.push(4);
                        compteur+=4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        
                        let substring: &str = &self.columns[i].get_column_type()[5..index.unwrap()];
                        //println!("{:?}", index);
                        let nbytes=" ".repeat(substring.parse::<usize>().unwrap()).as_bytes().len();
                        taille_objets.push(nbytes);
                        compteur+=4; // COMPTEUR + LA TAILLE DE LA CHAINE
                        continue;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let index:Option<usize> = s2.find(')') ;
                        let substring: &str = &self.columns[i].get_column_type()[8..index.unwrap()];
                    
                        let len_s=substring.parse::<usize>().unwrap();
                        let nbytes=if len_s>=tuple[i].len(){
                            tuple[i].as_bytes().len()
                        }else{
                            // A REVOIR
                            " ".repeat(len_s).as_bytes().len()
                        };
                        taille_objets.push(nbytes);
                        compteur+=4;
                        continue;
                    }
                    _ => {} //default du match 
                }
            }
            compteur+=4;
            
            //Ecriture des longueurs des attributs
            //ça met les offset dans le buffer
            let mut compteur2=compteur;
            for i in 0..taille_objets.len(){
                match self.columns[i].get_column_type().as_str() {
                    "INT" => {
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, (compteur2 + pos) as i32).unwrap();
                        compteur2+=4;
                        indice+=4;
                        continue;
                    }
                    "REAL" => {
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, (compteur2 + pos) as i32).unwrap();
                        compteur2+=4;
                        indice+=4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        let taille=taille_objets[i];
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, (compteur2 + pos) as i32).unwrap();

                        compteur2+=taille;
                        indice+=4;
                        continue;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let taille=taille_objets[i];
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, (compteur2 + pos) as i32).unwrap();

                        compteur2+=taille;
                        indice+=4;
                        continue;
                    }
                    _ => {} //default du match
                }
            }
            //let bytes=((compteur2) as u32).to_be_bytes();
            //buffer[indice..indice + 4].copy_from_slice(&bytes);
            buffer.write_int(indice, (compteur2 + pos) as i32).unwrap();
            indice=pos+compteur;

            // Ecriture des valeurs des attributs
            for i in 0..taille_objets.len(){
                match self.columns[i].get_column_type().as_str() {
                    "INT" => {
                        //on transforme la valeur dans le tuple en octets
                        //let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);

                        let value: i32 = tuple[i].parse().unwrap();
                        let _ = buffer.write_int(indice, value);

                        compteur+=4;// Pour les 4 octets de l'entier
                        indice += 4;
                        continue;
                    }
                    "REAL" => {
                        //let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        let value: f32 = tuple[i].parse().unwrap();
                        let _ = buffer.write_float(indice, value);
                        compteur+=4;// Pour les 4 octets du reel
                        indice += 4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        // 3 ligne en dessous : taille entre parentheses : (12) par exemple
                        /*
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s: usize=substring.parse::<usize>().unwrap();
                        */
    
                        let bytes = tuple[i].as_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        let _ = buffer.write_string(indice, tuple[i].as_str(), bytes.len());
                        compteur+= bytes.len();
                        indice += bytes.len();
                        continue;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        /*
                        let index:Option<usize> = s2.find(')') ;
                        let substring: &str = &tuple[i][8..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        */
                        let nbytes=tuple[i].as_bytes().len();
                        let bytes = tuple[i].as_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        let _ = buffer.write_string(indice, tuple[i].as_str(), bytes.len());
                        compteur += nbytes;
                        indice += nbytes;
                        continue;
                    }
                    _ => {}
                }
            }

        }else{// Si pas de VARCHAR dans le tuple 
            
            // FACILE à comprendre, le code est transparent :)
            for i in 0..self.nb_columns{
               //println!("{:?}", self.nb_columns);
                //let tmp=tuple[i].clone();
                
                match self.columns[i].get_column_type().as_str() {
                    "INT" => {
                         //on transforme la valeur dans le tuple en octets
                        //let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);

                        let value: i32 = tuple[i].parse().unwrap();
                        let _ = buffer.write_int(indice, value);


                        compteur+=4;// Pour les 4 octets de l'entier
                        indice += 4;
                        continue;
                        
                    }
                    "REAL" => {
                        //let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);

                        let value: f32 = tuple[i].parse().unwrap();
                        let _ = buffer.write_float(indice, value);

                        compteur += 4;// Pour les 4 octets du reel
                        indice += 4;
                        continue;
                    } // CHAR(20) --> 20 CARACTERES = 20 OCTETS
                    s if s.starts_with("CHAR")  => {
                        /*
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        */
                    
                        let bytes = tuple[i].as_bytes();
                        //println!("{:?}", buffer.len());
                        let _ = buffer.write_string(indice, tuple[i].as_str(), bytes.len());
                        compteur += bytes.len();
                        indice += bytes.len();
                        
                        continue;
                        
                    }
                    
                    _ => {} //default du match
                }

            }
        }
        return compteur;
        
        
    }
    
    pub fn read_from_buffer(& self,un_record: &mut Record, buff: &Buffer,  pos: usize) -> usize {
    
        let mut tuple:Vec<String> = Vec::new();
        let mut varchar = false;
        let mut nb_octets_lus = 0;
        let mut pos_local = pos;
        
        //on regarde si on a un varchar :
        for i in 0.. self.nb_columns{
            if self.columns[i].get_column_type().as_str().starts_with("VARCHAR"){
                varchar = true;
                break;
            }
        }
        
        
        //cas ou on a un varchar, du coup on aura des offsets
        if varchar {
            //la taille de la valeur est donnée par le offset impair et le offset qui le suit. (debut de la valeur dans le buffer et la fin de celle-ci)
            /* 
            let offset_debut: usize = 0;
            let offset_fin:usize = 0;
            let mut nb_octets_lu: usize = 0;
            let mut verif = 0;*/
            //on doit mettre dans le tuple les valeurs qui commencent après les offsets
            for i in 0..self.nb_columns{

               

                //let offset_debut: u32 = u32::from_be_bytes(buff[pos_local..pos_local + 4].try_into().unwrap());
                let offset_debut: usize = buff.read_int(pos_local).unwrap().try_into().unwrap();
                //println!("offset debut :{}",offset_debut);
                 //on convertit la valeur en entier (je sais pas si ça fonctionne ça, à méditer)
                //let offset_fin: u32 = u32::from_be_bytes(buff[pos_local+4..pos_local + 8].try_into().unwrap());
                let offset_fin: usize = buff.read_int(pos_local + 4).unwrap().try_into().unwrap();
                //println!("offset fin :{}",offset_fin);
                //on met dans le tuple le sous_vecteur correspondant à la valeur, en chaine de caractere

                nb_octets_lus +=4;

                if self.columns[i].get_column_type().eq("INT")  {

                    //let value = u32::from_be_bytes(buff[(offset_debut) as usize..(offset_fin) as usize].try_into().unwrap());
                    let value = buff.read_int(offset_debut).unwrap();
                    tuple.push(value.to_string());
                    nb_octets_lus+=4;
                   

                }
                else if self.columns[i].get_column_type().eq("REAL") {
                    //let value = f32::from_be_bytes(buff[(offset_debut) as usize..(offset_fin) as usize].try_into().unwrap());
                    let value = buff.read_float(offset_debut).unwrap();
                    tuple.push(value.to_string());
                    nb_octets_lus+=4;
                }
                else {
                    let string_value = buff.read_string(offset_debut, (offset_fin - offset_debut) as usize).unwrap();
                    tuple.push(string_value);
                    nb_octets_lus += (offset_fin - offset_debut) as usize;
                    
                }

                    

               

                //println!("NB OCTET LU READ inside boucle : {}",nb_octets_lus);

                //pour recup le nb d'octets lus, pas sur de ce que je fais là 
                pos_local +=4 ;
                //println!("ZIZI : {}",pos_local);
            }
            nb_octets_lus +=4;
        }
        else{
            let mut compteur_pos = pos;
            for i in 0..self.nb_columns{
                match self.columns[i].get_column_type().as_str() {
                    "INT" => {
                        // from_be_bytes et pas from_ne_bytes car en little indian ca renverse le bit de poids fort et faible
                        //let value = u32::from_be_bytes(buff[compteur_pos..compteur_pos + 4].try_into().unwrap());
                        //let value2 = String::from_utf8(buff[compteur_pos..compteur_pos+4].to_vec()).unwrap(); // CELA NE MARCHE PAS CAR IMPOSSIBLE DE CONVERTIR 4 OCTET EN UTF 8
                        //println!("VALEUR VALUE:{}",value);
                        let value = buff.read_int(compteur_pos).unwrap();
                        tuple.push(value.to_string());
                        compteur_pos += 4;
                        nb_octets_lus += 4;
                        continue;
                    }
                    "REAL" => {
                        //let value = f32::from_be_bytes(buff[compteur_pos..compteur_pos + 4].try_into().unwrap());
                        let value = buff.read_float(compteur_pos).unwrap();
                        tuple.push(value.to_string());
                        compteur_pos += 4;
                        nb_octets_lus += 4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        let indice_parenthese_ouvrante = s.find("(");
                        let indice_parenthese_fermante = s.find(")"); //on prend les deux parenthèses comme on connait pas le chiffre on connait pas la taille de son string correspondant
                        let taille_char = s[(indice_parenthese_ouvrante.unwrap()+1)..indice_parenthese_fermante.unwrap()].
                        parse::<i32>().unwrap();

                        let string_value = buff.read_string(compteur_pos, taille_char as usize).unwrap();

                        //tuple.push(String::from_utf8(buff[compteur_pos..compteur_pos + taille_char as usize].to_vec()).unwrap());

                        tuple.push(string_value);
                        compteur_pos += taille_char as usize ;
                        nb_octets_lus += taille_char as usize;
                        continue;
                    }
                     
                    _ => {} //default du match
                }
            }
        }
        un_record.set_tuple(tuple);
        //println!("NB OCTET LU READ : {}",nb_octets_lus);
        return nb_octets_lus as usize;
    }



    pub fn add_data_page(&mut self) -> () {
        // Emprunt mutable de buffer_manager pour effectuer toutes les opérations
        let mut buffer_manager = self.buffer_manager.borrow_mut();
        let nb_octets_restant = buffer_manager.get_disk_manager().get_dbconfig().get_page_size() as u32 ;
    

        // Allocation de la nouvelle page
        let nouvelle_page = buffer_manager.get_disk_manager_mut().alloc_page();

        // Accès et manipulation de la page d'en-tête
        let mut header_page = buffer_manager.get_page(&self.header_page_id); // Emprunt mutable de la page d'en-tête
        let mut nb_pages = header_page.read_int(0).unwrap();
        nb_pages += 1; // Incrémentation du nombre de pages
        let _ = header_page.write_int(0, nb_pages);
        

        let next_offset = 4 + (nb_pages - 1) * 12; // Calcul de l'offset pour l'écriture des données

        // Écriture des informations sur la nouvelle page
        let _ = header_page.write_int(next_offset as usize, nouvelle_page.get_file_idx() as i32);
        let _ = header_page.write_int((next_offset + 4) as usize, nouvelle_page.get_page_idx() as i32);

        // Calcul de la taille restante de la page
        let _ = header_page.write_int((next_offset + 8) as usize, (nb_octets_restant - 8 ) as i32);
    
        buffer_manager.free_page(&self.header_page_id, true); // Libération de la page d'en-tête

        //Écriture des trucs à la fin dans la datapage.
        let mut data_page = buffer_manager.get_page(&nouvelle_page);
        let _ = data_page.write_int((nb_octets_restant-4) as usize, 0);
        let _ = data_page.write_int((nb_octets_restant-8) as usize, 0);
        buffer_manager.free_page(&nouvelle_page, true);

        
        buffer_manager.flush_buffers();

}


    //Je retourne un Option car je veux que si je trouve rien, je retourne genre "null"
    pub fn get_free_data_page_id(&self, size_record: usize) -> Option<PageId>{

        let mut buffer_manager = self.buffer_manager.borrow_mut();
        //let page_id:PageId;


        for i in 0..buffer_manager.get_page(&self.header_page_id).read_int(0).unwrap(){

            let offset = 4 + i * 12;

           
            
            if size_record + 8  <=  buffer_manager.get_page(&self.header_page_id).read_int((offset + 8) as usize).unwrap() as usize  {

                let page = Some(PageId::new(buffer_manager.get_page(&self.header_page_id).read_int(offset as usize).unwrap() as u32, buffer_manager.get_page(&self.header_page_id).read_int((offset + 4) as usize).unwrap() as u32));
              

                buffer_manager.free_page(&self.header_page_id, false);
                buffer_manager.free_page(&self.header_page_id, false);

                return page
            }
        }

        buffer_manager.free_page(&self.header_page_id, false);

        return None;

    }

    pub fn write_record_to_data_page(&mut self, record: Record, page_id: PageId) -> RecordId {
        // Emprunt immuable temporaire pour obtenir des informations nécessaires
        let mut buffer_manager: std::cell::RefMut<'_, BufferManager<'a>> = self.buffer_manager.borrow_mut();

        let page_size = buffer_manager.get_disk_manager().get_dbconfig().get_page_size();

        // Emprunter la page une seule fois
        let mut page = buffer_manager.get_page(&page_id);

        // Lecture des données nécessaires une seule fois
        let position_libre = page.read_int((page_size - 4) as usize).unwrap() as usize;

        let taille_record: usize = self.write_record_to_buffer(record, &mut page, position_libre);
        println!("TAILLE RECORD {}",taille_record);

        let m_nb_slot: usize = page.read_int((page_size - 8) as usize).unwrap() as usize;

        // Mise à jour des données de la page
        let _ = page.write_int((page_size - 8) as usize, (m_nb_slot + 1) as i32); // Mise à jour du nombre de records
        let _ = page.write_int((page_size - 4) as usize, (position_libre + taille_record) as i32); // Mise à jour de la position libre

        let taille_pos: usize = m_nb_slot * 8; // Taille totale des couples (position, taille) déjà présents

        
        // Écriture du couple (position, taille) pour le record actuel
        let _ = page.write_int((page_size as usize) - 8 - taille_pos - 8, position_libre as i32);
        let _ = page.write_int((page_size as usize) - 8 - taille_pos - 4, taille_record as i32);

        let taille_totale: usize = taille_record + 8;

        // Mise à jour dans la page d'en-tête
        let mut header_page = buffer_manager.get_page(&self.header_page_id);
        for i in 0..header_page.read_int(0).unwrap() {
            let offset = 4 + i * 12;
            
            if header_page.read_int(offset as usize).unwrap() == (page_id.get_file_idx() as i32)
                && header_page.read_int((offset + 4) as usize).unwrap() == (page_id.get_page_idx() as i32)
            {
                let tmp = header_page.read_int((offset + 8) as usize).unwrap();
                let _ = header_page.write_int((offset + 8) as usize, tmp - taille_totale as i32)  ;
                break;
            }
        }

        buffer_manager.free_page(&page_id, true);
        buffer_manager.free_page(&self.header_page_id, true);
        buffer_manager.flush_buffers();

        // Retourner l'identifiant du record
        RecordId::new(page_id.clone(), (page_size as usize) - 8 - taille_pos - 8)
}


    pub fn get_records_in_data_page(&self, page_id: &PageId)-> Vec<Record> {

	    let mut buffer_manager: std::cell::RefMut<'_, BufferManager<'a>> = self.buffer_manager.borrow_mut();
	    
	    let mut liste_de_records = Vec::new();
	    let page_size = buffer_manager.get_disk_manager().get_dbconfig().get_page_size() as usize;
	    
	    
	    let buffer_data = buffer_manager.get_page(&page_id);
	    let nb_record = buffer_data.read_int(page_size - 8).unwrap() as usize;  
	    
	    let mut pos = 0;

        //let mut vec: Vec<String> = Vec::new();

	    for _i in 0..nb_record{
	        let vec: Vec<String> = Vec::new();

            let mut record = Record::new(vec);

         
            pos = pos + self.read_from_buffer(&mut record, &buffer_data, pos);


            liste_de_records.push(record);
    
	    }
	    
	    buffer_manager.free_page(&page_id, false);
	    return liste_de_records;
    }

    pub fn get_data_pages(&self) -> Vec<PageId> {
    
        let mut liste_pages = Vec::new();
        let mut buffer_manager  = self.buffer_manager.borrow_mut();
        //let page_size = buffer_manager.get_disk_manager().get_dbconfig().get_page_size() as usize;
        
        let buffer_header = buffer_manager.get_page(&self.header_page_id); 
        let nb_pages = buffer_header.read_int(0).unwrap();
        
        for i in 0..nb_pages{
            let file_idx = buffer_header.read_int((4 + i * 12) as usize).unwrap();
            let page_idx = buffer_header.read_int((4 + i * 12 + 4) as usize).unwrap();
            
            liste_pages.push(PageId::new(file_idx as u32, page_idx as u32));
        }
        
        buffer_manager.free_page(&self.header_page_id, false);
        return liste_pages;
    }

    
    pub fn insert_record(&mut self, record: Record) -> RecordId {
        let page_size = self.buffer_manager.borrow_mut().get_disk_manager().get_dbconfig().get_page_size();
        //tout ça c'est pour recup la taille du coup
        let mut byte_record = ByteBuffer::new();
        byte_record.resize(page_size as usize); // Je resize le buffer en fonction d'une page de donnée, on ne peut écrire dans
        // un buffer vide.
        let refcell_record = RefCell::new(byte_record);
        let mut buffer_record = Buffer::new(&Rc::new(refcell_record));
        
        //on récupère la taille du record de cette manière, pas sûr que ce soit la bonne méthode
        let taille_record = self.write_record_to_buffer(record.clone(), &mut buffer_record, 0);
        
        //on récupère une page avec assez de place pour écrire
        let data_page = self.get_free_data_page_id(taille_record);
        
        //Incroyable, Optimisation niveau Master, si question demander à Aymeric
        if data_page.is_none() {
            self.add_data_page();
            let data_page = (self.get_free_data_page_id(taille_record)).unwrap();
            return self.write_record_to_data_page(record, data_page);
        }
        else{
            return self.write_record_to_data_page(record, data_page.unwrap());
        }
        
    } 
    
    pub fn get_all_records(&mut self) -> Vec<Record> {
    
        let mut liste_records = Vec::new();
        let liste_data_pages = self.get_data_pages();
        
        for page in liste_data_pages.iter() {
            let mut liste_record_tmp = self.get_records_in_data_page(page);
            print!("PAGE : {:?}",page);
            liste_records.append(&mut liste_record_tmp);
            println!("Liste record tmp : {:?}",liste_record_tmp);
        }
        
        return liste_records;
    
    }

}

/*
#[cfg(test)]
mod tests{

    use crate::DBConfig;
    use crate::disk_manager::DiskManager;

   
    use super::*;
    use std::rc::Rc;
    
    #[test]
    fn test_write_varchar(){

        let chemin = String::from("res/dbpath/BinData");
        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);




        let record = Record::new(vec!["SOK".to_string(),"ARNAUD".to_string(),"20".to_string()]);
        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(3)".to_string()),
            ColInfo::new("AGE".to_string(), "VARCHAR(6)".to_string()),
            ColInfo::new("PRENOM".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);
        let pos=0; 

        let mut buffer =  ByteBuffer::new();
        buffer.resize(32);
        let refcbuffer = RefCell::new(buffer);
        let mut Buffer = Buffer::new(&Rc::new(refcbuffer));
        
       
        //let mut buffer = Vec::with_capacity(40);
        
        relation.write_record_to_buffer(record, &mut Buffer, pos);
        println!("{:?}", Buffer.get_mut_buffer().as_bytes());
        //A lancer avec "cargo test test_write_varchar -- --nocapture" pour voir le println
    }

    #[test]
    
    fn test_read_from_buffer() {

        let chemin = String::from("res/dbpath/BinData");
        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);


        let record = Record::new(vec!["SOK".to_string(),"20".to_string(),"ARNAUD".to_string()]);
        let record2 = record.clone();
        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(3)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(6)".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);
        let pos=0; 

        let mut buffer =  ByteBuffer::new();
        buffer.resize(32);
        let refcbuffer = RefCell::new(buffer);
        let mut Buffer = Buffer::new(&Rc::new(refcbuffer));
        
       
        
        relation.write_record_to_buffer(record, &mut Buffer, pos);
        //println!("{:?}", buffer);
        //println!("NB OCTET {}",relation.write_record_to_buffer(record2, &mut Buffer, pos));


        let string_tuple = vec!["".to_string(), "".to_string(), "".to_string()];

        let mut record_test: Record = Record::new(string_tuple);

        println!("NB octet lu {}",relation.read_from_buffer(&mut record_test, &Buffer, pos));
        

        println!("Contenu du record_test après lecture du buffer :");
        for field in record_test.get_tuple() {
            println!("{}", field);
        }

    }


    #[test]

    fn test_addDataPage() {


        let chemin = String::from("res/dbpath/BinData");
        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);

        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(3)".to_string()),
            ColInfo::new("AGE".to_string(), "VARCHAR(6)".to_string()),
            ColInfo::new("PRENOM".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);
        relation.addDataPage();
        relation.addDataPage();
        relation.addDataPage();
        relation.addDataPage();
        

    }    


    #[test]
    fn test_get_free_dataPage () {

        let chemin = String::from("res/dbpath/BinData");
        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);

        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(3)".to_string()),
            ColInfo::new("AGE".to_string(), "VARCHAR(6)".to_string()),
            ColInfo::new("PRENOM".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);
        relation.addDataPage();
        relation.addDataPage();

        let freepage = relation.get_free_data_page_id(10).unwrap();
        println!("Page ID : {},{}",freepage.get_FileIdx(),freepage.get_PageIdx());


    }

    #[test]


    fn test_writeRecordToDataPage() {

        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);

        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);

        let record1 = Record::new(vec!["SOK".to_string(),"ARNAUD".to_string(),"20".to_string()]);
        let record2 = Record::new(vec!["MEUNIER".to_string(),"YOHANN".to_string(),"20".to_string()]);
        let record3 = Record::new(vec!["LETACONNOUX".to_string(),"AYMERIC".to_string(),"20".to_string()]);

        let page_id = PageId::new(0, 1);
        relation.addDataPage();
        let rid1 = relation.writeRecordToDataPage(record1, page_id);
        let rid2 = relation.writeRecordToDataPage(record2, page_id);
        //relation.writeRecordToDataPage(record3, page_id);
        println!("RID tuple 1 : File idx {}, Page idx {}, Slot idx : {}",rid1.get_page_id().get_FileIdx(),rid1.get_page_id().get_PageIdx(),rid1.get_slot_idx());

        println!("RID tuple 2 : File idx {}, Page idx {}, Slot idx : {}",rid2.get_page_id().get_FileIdx(),rid2.get_page_id().get_PageIdx(),rid2.get_slot_idx());
        
    }

    #[test] 

    fn test_getRecordsInDataPage() {

        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);

        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);


        let record1 = Record::new(vec!["SOK".to_string(),"ARNAUD".to_string(),"20".to_string()]);
        let record2 = Record::new(vec!["MEUNIER".to_string(),"YOHANN".to_string(),"20".to_string()]);
        let record3 = Record::new(vec!["MOUE".to_string(),"MAT".to_string(),"20".to_string()]);

        let page_id = PageId::new(0, 1);
        relation.addDataPage();
        relation.writeRecordToDataPage(record1, page_id);
        relation.writeRecordToDataPage(record2, page_id);
        relation.writeRecordToDataPage(record3, page_id);

        let vecRecord = relation.get_records_in_data_page(&page_id);

        println!("{:?}",vecRecord);

        /*for field in vecRecord[0].get_tuple() {
            println!("{}", field);
        }*/


    }

    #[test]

    fn test_get_data_pages () {

        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);

        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);

        relation.addDataPage();
        relation.addDataPage();
        relation.addDataPage();
        relation.addDataPage();
        relation.addDataPage();

        let vecPage = relation.get_data_pages();

        println!("{:?}",vecPage);
    }

    #[test]

    fn test_insert_record() {

        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);

        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);

        let record1 = Record::new(vec!["SOK".to_string(),"ARNAUD".to_string(),"20".to_string()]);
        let record2 = Record::new(vec!["MEUNIER".to_string(),"YOHANN".to_string(),"20".to_string()]);
        let record3 = Record::new(vec!["MEUNIER".to_string(),"YOHANN".to_string(),"20".to_string()]);
        let record4 = Record::new(vec!["MEUNIER".to_string(),"YOHANN".to_string(),"20".to_string()]);


        let rid1= relation.insert_record(record1);
        let rid2 = relation.insert_record(record2);
        let rid3 = relation.insert_record(record3);
        let rid4 = relation.insert_record(record4);

        println!("RID tuple 1 : File idx {}, Page idx {}, Slot idx : {}",rid1.get_page_id().get_FileIdx(),rid1.get_page_id().get_PageIdx(),rid1.get_slot_idx());

        println!("RID tuple 2 : File idx {}, Page idx {}, Slot idx : {}",rid2.get_page_id().get_FileIdx(),rid2.get_page_id().get_PageIdx(),rid2.get_slot_idx());


    }

    #[test]
    fn test_get_all_records() {

        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");
        
        let mut buffer_manager = BufferManager::new(&config, dm, &algo_lru);

        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(20)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),buffer_manager);


        let record1 = Record::new(vec!["SOK".to_string(),"ARNAUD".to_string(),"20".to_string()]);
        let record2 = Record::new(vec!["MEUNIER".to_string(),"YOHANN".to_string(),"20".to_string()]);
        let record3 = Record::new(vec!["MOUST".to_string(),"MATH".to_string(),"20".to_string()]);
        let record4 = Record::new(vec!["LETACONNOUX".to_string(),"AYMERIC".to_string(),"20".to_string()]);
        let record5 = Record::new(vec!["CHIBANNI".to_string(),"RAMZY".to_string(),"20".to_string()]);
        let record6 = Record::new(vec!["BOTKZ".to_string(),"LEFOU".to_string(),"89".to_string()]);
        let record7 = Record::new(vec!["GNAHO".to_string(),"CHRISTOPHE".to_string(),"50".to_string()]);


        let rid1= relation.insert_record(record1);
        let rid2 = relation.insert_record(record2);
        let rid3 = relation.insert_record(record3);
        let rid4 = relation.insert_record(record4);
        let rid5 = relation.insert_record(record5);
        let rid6 = relation.insert_record(record6);
        let rid7 = relation.insert_record(record7); 


        let listRecord = relation.get_all_records();

        println!("Liste record : {:?}",listRecord);

        /*println!("RID tuple 1 : File idx {}, Page idx {}, Slot idx : {}",rid1.get_page_id().get_FileIdx(),rid1.get_page_id().get_PageIdx(),rid1.get_slot_idx());

        println!("RID tuple 2 : File idx {}, Page idx {}, Slot idx : {}",rid2.get_page_id().get_FileIdx(),rid2.get_page_id().get_PageIdx(),rid2.get_slot_idx());

        println!("RID tuple 3 : File idx {}, Page idx {}, Slot idx : {}",rid3.get_page_id().get_FileIdx(),rid3.get_page_id().get_PageIdx(),rid3.get_slot_idx());

        println!("RID tuple 4 : File idx {}, Page idx {}, Slot idx : {}",rid4.get_page_id().get_FileIdx(),rid4.get_page_id().get_PageIdx(),rid4.get_slot_idx());*/

    }


    
     /* 
    #[test]
    fn test_ecriture_dans_un_fichier() {
        //println!("{}",(4 as u32).to_be_bytes().len().to_string());
        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "VARCHAR(10)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(10)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new(String::from("PERSONNE"),colinfo.clone());

    // Ajout de colonnes à la relation en utilisant la méthode new de ColInfo
        

        // Exemple de création d'un Record
        let record = Record::new(vec![
        String::from("Dupozt"),
        String::from("Jean"),
        String::from("30"),
        ]);

        let mut buffer =  ByteBuffer::new();
        buffer.resize(32);
        let refcbuffer = RefCell::new(buffer);
        let mut Buffer = Buffer::new(&refcbuffer);
        

       relation.write_record_to_buffer(record, &mut Buffer, 0);

        let s: String = String::from("res/fichier_test_write_relation");
        let mut fichier1 = OpenOptions::new().write(true).open(s).expect("tkt");

        fichier1.write_all(&refcbuffer.borrow().as_bytes());
       
      

    }
    */
    

    
    
    
}
*/