use bytebuffer::ByteBuffer;

use string_builder::Builder;
use crate::buffer::{self, Buffer};
use crate::col_info::ColInfo;
use crate::page::{self, PageId};
use crate::record::Record;
use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::cell::{Ref, RefCell};
use crate::buffer_manager::BufferManager;
use crate::record_id::RecordId;
use crate::DBConfig;
use crate::disk_manager::DiskManager;
use std::env;

pub struct Relation<'a> { //PERSONNE(NOM,PRENOM?,AGE)
    name:String,
    columns: Vec<ColInfo>,
    nb_columns: usize,
    //TP5
    buffer_manager: RefCell<BufferManager<'a>>, 
    header_page_id : PageId  //id de la header page
    
}

impl<'a> Relation<'a> {

    pub fn new(name: String, columns: Vec<ColInfo>, mut bfm: BufferManager<'a>) -> Self {
        let tmp = columns.len();

        //On appelle 'alloc_page' avant de déplacer 'buffer_manager' sinon ça fais des chinoiseries
        let header_page_id = bfm.get_disk_manager_mut().alloc_page();
        //println!("file idx {}",header_page_id.get_FileIdx());
        //println!("page idx {}",header_page_id.get_PageIdx());

        //let mut x = bfm.get_page(&header_page_id);
        //println!("{:?}",x.get_mut_buffer().as_bytes());
        bfm.get_page(&header_page_id).write_int(0, 0);
        //println!("{:?}",x.get_mut_buffer().as_bytes());
        bfm.free_page(&header_page_id, true);
        //println!("first dirty constru {:?}",bfm.get_liste_pages()[0].get_dirty());
        //println!("Page id du construc");
        //println!("{}",bfm.get_liste_pages()[0].get_page_id().get_FileIdx());
        //println!("{}",bfm.get_liste_pages()[0].get_page_id().get_PageIdx());

        /* 
        println!("contenu buffer1 : {:?}",bfm.get_liste_buffer()[0].borrow());
        println!("contenu buffer2 : {:?}",bfm.get_liste_buffer()[1].borrow());
        println!("contenu buffer3 : {:?}",bfm.get_liste_buffer()[2].borrow());
        println!("contenu buffer4 : {:?}",bfm.get_liste_buffer()[3].borrow());
       
        
        //println!("second dirty constru {}",bfm.get_liste_pages()[0].get_dirty());
        println!("Taille vec page info : {}",bfm.get_liste_pages().len());
        println!("Taille vec buff : {}",bfm.get_liste_buffer().len());
        println!("contenu buffer1 : {:?}",bfm.get_liste_buffer()[0].borrow());
        println!("contenu buffer2 : {:?}",bfm.get_liste_buffer()[1].borrow());
        println!("contenu buffer3 : {:?}",bfm.get_liste_buffer()[2].borrow());
        println!("contenu buffer4 : {:?}",bfm.get_liste_buffer()[3].borrow());*/

        bfm.flush_buffers();
       
        Relation {
            name: String::from(name),
            columns,
            nb_columns: tmp,
            buffer_manager: RefCell::new(bfm), 
            header_page_id,
        }
    }

    fn get_name(&self)->&String {
        &self.name
    }

    // Getter pour les informations sur les colonnes
    fn get_columns(&self) -> Vec<ColInfo> {
        self.columns.clone()
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
                match self.columns[i].get_column_type().as_str().clone() {
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
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, compteur2 as i32).unwrap();
                        compteur2+=4;
                        indice+=4;
                        continue;
                    }
                    "REAL" => {
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, compteur2 as i32).unwrap();
                        compteur2+=4;
                        indice+=4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        let taille=taille_objets[i];
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, compteur2 as i32);

                        compteur2+=taille;
                        indice+=4;
                        continue;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let taille=taille_objets[i];
                        //let bytes=(compteur2 as u32).to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_int(indice, compteur2 as i32);

                        compteur2+=taille;
                        indice+=4;
                        continue;
                    }
                    _ => {} //default du match
                }
            }
            //let bytes=((compteur2) as u32).to_be_bytes();
            //buffer[indice..indice + 4].copy_from_slice(&bytes);
            buffer.write_int(indice, compteur2 as i32);
            indice=pos+compteur;

            // Ecriture des valeurs des attributs
            for i in 0..taille_objets.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        //on transforme la valeur dans le tuple en octets
                        //let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);

                        let value: i32 = tuple[i].parse().unwrap();
                        buffer.write_int(indice, value);

                        compteur+=4;// Pour les 4 octets de l'entier
                        indice += 4;
                        continue;
                    }
                    "REAL" => {
                        //let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        let value: f32 = tuple[i].parse().unwrap();
                        buffer.write_float(indice, value);
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
    
                        let mut bytes = tuple[i].as_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_string(indice, tuple[i].as_str(), bytes.len());
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
                        let mut bytes = tuple[i].as_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        buffer.write_string(indice, tuple[i].as_str(), bytes.len());
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
                let tmp=tuple[i].clone();
                
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                         //on transforme la valeur dans le tuple en octets
                        //let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);

                        let value: i32 = tuple[i].parse().unwrap();
                        buffer.write_int(indice, value);


                        compteur+=4;// Pour les 4 octets de l'entier
                        indice += 4;
                        continue;
                        
                    }
                    "REAL" => {
                        //let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        //buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);

                        let value: f32 = tuple[i].parse().unwrap();
                        buffer.write_float(indice, value);

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
                    
                        let mut bytes = tuple[i].as_bytes();
                        //println!("{:?}", buffer.len());
                        buffer.write_string(indice, tuple[i].as_str(), bytes.len());
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
            let offset_debut: usize = 0;
            let offset_fin:usize = 0;
            let mut nb_octets_lu: usize = 0;
            let mut verif = 0;
            //on doit mettre dans le tuple les valeurs qui commencent après les offsets
            for i in 0..self.nb_columns{

               

                //let offset_debut: u32 = u32::from_be_bytes(buff[pos_local..pos_local + 4].try_into().unwrap());
                let offset_debut: usize = buff.read_int(pos_local).unwrap().try_into().unwrap();
                println!("offset debut :{}",offset_debut);
                 //on convertit la valeur en entier (je sais pas si ça fonctionne ça, à méditer)
                //let offset_fin: u32 = u32::from_be_bytes(buff[pos_local+4..pos_local + 8].try_into().unwrap());
                let offset_fin: usize = buff.read_int(pos_local + 4).unwrap().try_into().unwrap();
                println!("offset fin :{}",offset_fin);
                //on met dans le tuple le sous_vecteur correspondant à la valeur, en chaine de caractere


                if self.columns[i].get_column_type().eq("INT")  {

                    //let value = u32::from_be_bytes(buff[(offset_debut) as usize..(offset_fin) as usize].try_into().unwrap());
                    let value = buff.read_int(offset_debut).unwrap();
                    tuple.push(value.to_string());
                   

                }
                else if self.columns[i].get_column_type().eq("REAL") {
                    //let value = f32::from_be_bytes(buff[(offset_debut) as usize..(offset_fin) as usize].try_into().unwrap());
                    let value = buff.read_float(offset_debut).unwrap();
                    tuple.push(value.to_string());
                }
                else {
                    let string_value = buff.read_string(offset_debut, (offset_fin - offset_debut) as usize).unwrap();
                    tuple.push(string_value);
                }

                    

                nb_octets_lu += (offset_fin - offset_debut) as usize; //pour recup le nb d'octets lus, pas sur de ce que je fais là 
                pos_local +=4 ;
                //println!("ZIZI : {}",pos_local);
            }
        }
        else{
            let mut compteur_pos = pos;
            for i in 0..self.nb_columns{
                match self.columns[i].get_column_type().as_str().clone() {
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
                        nb_octets_lus += taille_char;
                        continue;
                    }
                    
                    _ => {} //default du match
                }
            }
        }
        un_record.set_tuple(tuple);
        return nb_octets_lus as usize;
    }


    /*pub fn addDataPage(&mut self) -> () {

        let mut buffer_manager = self.buffer_manager.borrow_mut();
        let nouvelle_page = buffer_manager.get_disk_manager_mut().alloc_page();
    
        let mut nb_pages = buffer_manager.get_page(&self.header_page_id).read_int(0).unwrap();
        //print!(" nb pages 1 {} ",nb_pages);


        nb_pages = nb_pages+1;
        buffer_manager.get_page(&self.header_page_id).write_int(0, nb_pages);
        //print!(" nb pages 2 {} ",nb_pages);

        
    
        let next_offset = 4 + (nb_pages - 1) * 12; //Position pour écrire les couples page idx file idx
        buffer_manager.get_page(&self.header_page_id).write_int(next_offset as usize, nouvelle_page.get_FileIdx() as i32);
        buffer_manager.get_page(&self.header_page_id).write_int((next_offset+4) as usize, nouvelle_page.get_PageIdx() as i32);
    
        //On doit maintenant écrire la place restante dans la page de donnée.
        //let nb_octets_pris = buffer_manager.get_page(&nouvelle_page).get_mut_buffer().len();
        let nb_octets_restant = buffer_manager.get_disk_manager().get_dbconfig().get_page_size()  as u32;
        buffer_manager.get_page(&self.header_page_id).write_int((next_offset+8) as usize,nb_octets_restant as i32);

       

        //On free les pages, quand elles vont être bougé du bufferpool, ca va les écrire imo.
        //buffer_manager.free_page(&self.header_page_id, true);
        buffer_manager.free_page(&self.header_page_id, true);
        buffer_manager.free_page(&self.header_page_id, true);
        buffer_manager.free_page(&self.header_page_id, true);
        buffer_manager.free_page(&self.header_page_id, true);
        buffer_manager.free_page(&nouvelle_page, false);
        
        buffer_manager.flush_buffers();

        //let mut byte_buffer = buffer_manager.get_page(&self.header_page_id).get_mut_buffer(); 
        //buffer_manager.get_disk_manager().write_page(&self.header_page_id, &mut byte_buffer);
     
    }*/

    pub fn addDataPage(&mut self) -> () {
    // Emprunt mutable de buffer_manager pour effectuer toutes les opérations
    let mut buffer_manager = self.buffer_manager.borrow_mut();
    let nb_octets_restant = buffer_manager.get_disk_manager().get_dbconfig().get_page_size() as u32;
    

    //let x = buffer_manager.get_liste_pages()[0].get_dirty();

    // Allocation de la nouvelle page
    let nouvelle_page = buffer_manager.get_disk_manager_mut().alloc_page();

    // Accès et manipulation de la page d'en-tête
    let mut header_page = buffer_manager.get_page(&self.header_page_id); // Emprunt mutable de la page d'en-tête

    //println!("{}",x);
  

    let mut nb_pages = header_page.read_int(0).unwrap();
    nb_pages += 1; // Incrémentation du nombre de pages
    header_page.write_int(0, nb_pages);

    let next_offset = 4 + (nb_pages - 1) * 12; // Calcul de l'offset pour l'écriture des données

    // Écriture des informations sur la nouvelle page
    header_page.write_int(next_offset as usize, nouvelle_page.get_FileIdx() as i32);
    header_page.write_int((next_offset + 4) as usize, nouvelle_page.get_PageIdx() as i32);

    // Calcul de la taille restante de la page
   
    header_page.write_int((next_offset + 8) as usize, nb_octets_restant as i32);
    println!("{:?}",header_page.get_mut_buffer().as_bytes());

    //drop(header_page);

    /* 
    let x = buffer_manager.get_liste_pages()[0].get_dirty();
    //let x2 = buffer_manager.get_liste_pages()[1].get_dirty();
    let x3 = buffer_manager.get_liste_pages()[0].get_pin_count();
    //let x4 = buffer_manager.get_liste_pages()[1].get_pin_count();
    let x5 = buffer_manager.get_liste_pages()[0].get_page_id().get_FileIdx();
    let x6 = buffer_manager.get_liste_pages()[0].get_page_id().get_PageIdx();
    //let x7 = buffer_manager.get_liste_pages()[1].get_page_id().get_FileIdx();
    //let x8 = buffer_manager.get_liste_pages()[1].get_page_id().get_PageIdx();

    
    println!("{}",x);
    //println!("{}",x2);
    println!("{}",x3);
    //println!("{}",x4);
    println!("Page idx  dans buffer 1: {},{}",x5,x6);
    //println!("Page idx  dans buffer 2: {},{}",x7,x8);

    */
    // Libération des pages en fin de traitement


    let x15 = buffer_manager.get_liste_pages()[0].get_dirty();
    println!("first dirty {}",x15);

    buffer_manager.free_page(&self.header_page_id, true); // Libération de la page d'en-tête
    let x13 = buffer_manager.get_liste_pages()[0].get_dirty();
    println!("second dirty {}",x13);

    println!("contenu buffer1 : {:?}",buffer_manager.get_liste_buffer()[0].borrow());
    println!("contenu buffer2 : {:?}",buffer_manager.get_liste_buffer()[1].borrow());
    println!("contenu buffer3 : {:?}",buffer_manager.get_liste_buffer()[2].borrow());
    println!("contenu buffer4 : {:?}",buffer_manager.get_liste_buffer()[3].borrow());


    buffer_manager.flush_buffers();

    println!("Taille vec page info : {}",buffer_manager.get_liste_pages().len());
    println!("Taille vec buff : {}",buffer_manager.get_liste_buffer().len());
    println!("contenu buffer1 : {:?}",buffer_manager.get_liste_buffer()[0].borrow());
    println!("contenu buffer2 : {:?}",buffer_manager.get_liste_buffer()[1].borrow());
    println!("contenu buffer3 : {:?}",buffer_manager.get_liste_buffer()[2].borrow());
    println!("contenu buffer4 : {:?}",buffer_manager.get_liste_buffer()[3].borrow());
    //let x5 = buffer_manager.get_liste_pages()[0].get_page_id().get_FileIdx();
    //let x6 = buffer_manager.get_liste_pages()[0].get_page_id().get_PageIdx();
    //let x10 = buffer_manager.get_liste_pages()[0].get_dirty();
    //println!("Page idx  dans le premier buffer : {},{}",x5,x6);
   //println!("second dirty {}",x10);
    //buffer_manager.free_page(&nouvelle_page, false);     // Libération de la nouvelle page
    // Finalisation de l'écriture des buffers




        /* 
       
       
        
        //println!("second dirty constru {}",bfm.get_liste_pages()[0].get_dirty());
        println!("Taille vec page info : {}",bfm.get_liste_pages().len());
        println!("Taille vec buff : {}",bfm.get_liste_buffer().len());
        println!("contenu buffer1 : {:?}",bfm.get_liste_buffer()[0].borrow());
        println!("contenu buffer2 : {:?}",bfm.get_liste_buffer()[1].borrow());
        println!("contenu buffer3 : {:?}",bfm.get_liste_buffer()[2].borrow());
        println!("contenu buffer4 : {:?}",bfm.get_liste_buffer()[3].borrow());*/
    
   
}

    




    //Je retourne un Option car je veux que si je trouve rien, je retourne genre "null"
    pub fn get_free_data_page_id(&self, size_record: usize) -> Option<PageId>{

        let mut buffer_manager = self.buffer_manager.borrow_mut();
        let page_id:PageId;


        for i in 0..buffer_manager.get_page(&self.header_page_id).read_int(0).unwrap(){

            let offset = 4 + i * 12;

            if (size_record <=  buffer_manager.get_page(&self.header_page_id).read_int((offset + 8) as usize).unwrap() as usize) {

                let page = Some(PageId::new(buffer_manager.get_page(&self.header_page_id).read_int(offset as usize).unwrap() as u32, buffer_manager.get_page(&self.header_page_id).read_int((offset + 4) as usize).unwrap() as u32));

                buffer_manager.free_page(&self.header_page_id, false);
                buffer_manager.free_page(&self.header_page_id, false);

                return page
            }
        }

        buffer_manager.free_page(&self.header_page_id, false);

        return None;

    }

    pub fn writeRecordToDataPage(&mut self, record: Record, page_id: PageId) -> RecordId {
        // Emprunt immuable temporaire pour obtenir des informations nécessaires
        let mut buffer_manager: std::cell::RefMut<'_, BufferManager<'a>> = self.buffer_manager.borrow_mut();

        let page_size = buffer_manager.get_disk_manager().get_dbconfig().get_page_size();

        // Emprunter la page une seule fois
        let mut page = buffer_manager.get_page(&page_id);

        // Lecture des données nécessaires une seule fois
        let position_libre = page.read_int((page_size - 4) as usize).unwrap() as usize;
        let taille_record: usize = self.write_record_to_buffer(record, &mut page, position_libre);

        let m_nb_slot: usize = page.read_int((page_size - 8) as usize).unwrap() as usize;

        // Mise à jour des données de la page
        page.write_int((page_size - 8) as usize, (m_nb_slot + 1) as i32); // Mise à jour du nombre de records
        page.write_int((page_size - 4) as usize, (position_libre + taille_record) as i32); // Mise à jour de la position libre

        let taille_pos: usize = m_nb_slot * 8; // Taille totale des couples (position, taille) déjà présents

        // Écriture du couple (position, taille) pour le record actuel
        page.write_int((page_size as usize) - 8 - taille_pos - 8, position_libre as i32);
        page.write_int((page_size as usize) - 8 - taille_pos - 4, taille_record as i32);

        let taille_totale: usize = taille_record + 8;

        // Mise à jour dans la page d'en-tête
        let mut header_page = buffer_manager.get_page(&self.header_page_id);
        for i in 0..header_page.read_int(0).unwrap() {
            let offset = 4 + i * 12;

            if header_page.read_int(offset as usize).unwrap() == (page_id.get_FileIdx() as i32)
                && header_page.read_int((offset + 4) as usize).unwrap() == (page_id.get_PageIdx() as i32)
            {
                let tmp = header_page.read_int((offset + 8) as usize).unwrap();
                header_page.write_int((offset + 8) as usize, tmp - taille_totale as i32);
            }
        }

        // Retourner l'identifiant du record
        RecordId::new(page_id.clone(), (page_size as usize) - 8 - taille_pos - 8)
}

    pub fn get_records_in_data_page(&self, pageId: &PageId)-> Vec<Record> {

	    let mut buffer_manager: std::cell::RefMut<'_, BufferManager<'a>> = self.buffer_manager.borrow_mut();
	    
	    let mut listeDeRecords = Vec::new();
	    let page_size = buffer_manager.get_disk_manager().get_dbconfig().get_page_size() as usize;
	    
	    
	    let buffer_data = buffer_manager.get_page(pageId); 
	    let nb_record = buffer_data.read_int(page_size - 8).unwrap() as usize;  
	    
	    let mut pos = 0;
	    for i in 0..nb_record{
	        let vec: Vec<String> = Vec::new();
            let mut record = Record::new(vec);
            pos = self.read_from_buffer(&mut record, &buffer_data, pos);
		    listeDeRecords.push(record);
	    }
	    
	    buffer_manager.free_page(pageId, false);
	    return listeDeRecords;
    }

    pub fn get_data_pages(&self) -> Vec<PageId> {
    
        let mut listePages = Vec::new();
        let mut buffer_manager: std::cell::RefMut<'_, BufferManager<'a>> = self.buffer_manager.borrow_mut();
        let page_size = buffer_manager.get_disk_manager().get_dbconfig().get_page_size() as usize;
        
        let buffer_header = buffer_manager.get_page(&self.header_page_id); 
        let nb_pages = buffer_header.read_int(0).unwrap();
        
        for i in 0..nb_pages{
            let file_idx = buffer_header.read_int((4 + i * 12) as usize).unwrap();
            let page_idx = buffer_header.read_int((4 + i * 12 + 4) as usize).unwrap();
            
            listePages.push(PageId::new(file_idx as u32, page_idx as u32));
        }
        
        buffer_manager.free_page(&self.header_page_id, false);
        return listePages;
    }

    pub fn insert_record(&mut self, record: Record) -> RecordId {
        //tout ça c'est pour recup la taille du coup
        let mut byte_record = ByteBuffer::new();
        let refcell_record = RefCell::new(byte_record);
        let mut buffer_record = Buffer::new(&refcell_record);
        
        //on récupère la taille du record de cette manière, pas sûr que ce soit la bonne méthode
        let taille_record = self.write_record_to_buffer(record.clone(), &mut buffer_record, 0);
        
        //on récupère une page avec assez de place pour écrire
        let data_page = self.get_free_data_page_id(taille_record);
        
        if(data_page.is_none()){
            self.addDataPage();
            let data_page = (self.get_free_data_page_id(taille_record)).unwrap();
            return self.writeRecordToDataPage(record, data_page);
        }
        else{
            return self.writeRecordToDataPage(record, data_page.unwrap());
        }
        
    }
    
    pub fn get_all_records(&mut self) -> Vec<Record> {
    
        let mut liste_records = Vec::new();
        let mut liste_data_pages = self.get_data_pages();
        
        for page in liste_data_pages.iter() {
            let mut liste_record_tmp = self.get_records_in_data_page(page);
            liste_records.append(&mut liste_record_tmp);
        }
        
        return liste_records;
    
    }



}


#[cfg(test)]
mod tests{

    
    use std::borrow::Borrow;
    use super::*;
    
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
        let mut Buffer = Buffer::new(&refcbuffer);
        
       
        //let mut buffer = Vec::with_capacity(40);
        
        relation.write_record_to_buffer(record, &mut Buffer, pos);
        println!("{:?}", refcbuffer.borrow());
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
        let mut Buffer = Buffer::new(&refcbuffer);
        
       
        
        relation.write_record_to_buffer(record, &mut Buffer, pos);
        //println!("{:?}", buffer);
        //println!("NB OCTET {}",relation.write_record_to_buffer(record2, &mut buffer, pos));


        let string_tuple = vec!["".to_string(), "".to_string(), "".to_string()];

        let mut record_test: Record = Record::new(string_tuple);

        relation.read_from_buffer(&mut record_test, &Buffer, pos);
        

        println!("Contenu du record_test après lecture du buffer :");
        for field in record_test.get_tuple() {
            println!("{}", field);
        }

    }


    #[test]

    fn test_addDataPage() {

        env::set_var("RUST_BACKTRACE", "1");

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
        relation.addDataPage();
        relation.addDataPage();
        



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
