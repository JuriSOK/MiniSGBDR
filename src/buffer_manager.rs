use core::str;

use bytebuffer::ByteBuffer;
use crate::{config::DBConfig, disk_manager::{self, DiskManager}, page::{self, PageId}, page_info::{self, PageInfo}};

pub struct BufferManager<'a>{

    //Quand on passe une référence e, attributs, life time obligatoire
    db_config:&'a DBConfig,
    disk_manager:&'a DiskManager<'a>,
    liste_pages:Vec<PageInfo>, //quand c'est mutable aussi

    //Concrètement, c'est le buffer pool, Ex si 4 Buffers, alors on a un vecteur de 4 ByteBuffer
    liste_buffer:Vec<ByteBuffer>,
    //Pour pouvoir tracker les pages à enlever, ex à chaque getPage on incrémente le temps, et si on doit freePage, 
    //on sait que c'est à ce temps là. Ex : GetPage --> compteur_temps == 1 et pin_count  == 1, freePage --> compteur_temps = 0 et pin_count = 0 donc bye bye la page
    compteur_temps:u64,
    //LRU ou MRU
    algo_remplacement:&'a String,
    // Compteur général du nombre de page dans le buffer
    nb_pages_vecteur : u32, 


}

impl<'a> BufferManager<'a>{

    pub fn new(db_config:&'a DBConfig, disk_manager:&'a DiskManager, algo_remplacement:&'a String)->Self
    {
        let compteur_temps:u64=0;

        //On crée un Vecteur de ByteBuffer de la taille qu'on a dans le fichier.json
        let mut tmp: Vec<ByteBuffer> =Vec::<ByteBuffer>::with_capacity(db_config.get_bm_buffer_count() as usize);
        //On doit rédéfinir la taille de chaque ByteBuffer, nous on veut que chaque ByteBuffer fait la taille d'une page.
        for i in tmp.iter_mut(){
            i.resize(db_config.get_page_size() as usize);
        }

        let mut liste_pages: Vec<PageInfo> = Vec::<PageInfo>::with_capacity(db_config.get_bm_buffer_count() as usize);

        Self { db_config,
            disk_manager,
            liste_pages,
            liste_buffer: tmp,
            compteur_temps,
            algo_remplacement, 
            nb_pages_vecteur:0,
        }
    }

    pub fn lru(&mut self)->usize{ //Renvoie l'indice de la page à bouger dans liste_buffer (je pense mais à vérifier c pas qui a fais)



        let mut indice:u32=0;

        //On définit arbitrairement la première page comme référence pour la comparaison.
        let mut oldest_page:&PageInfo=&self.liste_pages[0];

        
        let mut premierelemtrouve:bool=false;

        //On parcourt le buffer pool (vecteur de ByteBuffer)
        for i in 0..self.liste_pages.len(){

            //Si la page dans un ByteBuffer a le pincount à 0
            if self.liste_pages[i].get_pin_count() == 0 {


                //On cherche maintenant si il existe une page avec le pin_count() à 0 avec un temps plus petit,
                //Si c'est le cas alors on prend lui car on est dans LRU (Least recently use).
                if premierelemtrouve{
                    if oldest_page.get_time() > self.liste_pages[i].get_time() {

                        oldest_page=&self.liste_pages[i];
                        indice = i as u32;
                    }
                    
                //On a trouvé une première page, donc on active premierelemtrouve pour commencé à comparer.
                }else{
                    oldest_page=&self.liste_pages[i];
                    premierelemtrouve=true;
                }
            }
        }

        //On doit vérifier car si on en trouve pas, cela va renvoyer la pageInfo qu'on a défini arbitrairement.
        if oldest_page.get_pin_count()==0 {
            return indice as usize;
        }
        else {
            return self.db_config.get_bm_buffer_count() as usize; //ON a besoin d'une valeur de retour, ici valeur interdit à priori?
        }

    }


    //Même idée que LRU sauf qu'au lieu de prendre celle avec le temps le plus bas, on va prendre celui avec le temps le plus haut.
    pub fn mru(&mut self)->usize{ //Renvoie l'indice de la page à bouger dans liste_buffer (je pense mais à vérifier c pas qui a fais)


        let mut indice:u32=0;

        let mut oldest_page:&PageInfo=&self.liste_pages[0];

        let mut premierelemtrouve:bool=false;

        for i in 0..self.liste_pages.len(){

            if self.liste_pages[i].get_pin_count() == 0 {

                if premierelemtrouve{
                    if oldest_page.get_time() < self.liste_pages[i].get_time() {

                        oldest_page=&self.liste_pages[i];
                        indice = i as u32;
                    }
                    
                }else{
                    oldest_page=&self.liste_pages[i];
                    premierelemtrouve=true;
                }
            }
        }

        if oldest_page.get_pin_count()==0 {
            return indice as usize;
        }
        else {
            return self.db_config.get_bm_buffer_count() as usize; //ON a besoin d'une valeur de retour, ici valeur interdit à priori?
        }

    }


    //Pour changer l'algo
    pub fn set_current_replacement_policy(&mut self, algo:&'a String){
        self.algo_remplacement=algo;
    }


    // ATTTENTION À VÉRIFIER ABSOLUMENT JE SUIS PAS CONFIANT DU TOUT POUR CA ,
    pub fn get_page(&mut self,page_id:&PageId)->&mut ByteBuffer{
        if self.nb_pages_vecteur < 4 {
            let pageinfo  : PageInfo = PageInfo::new( page_id.clone(), 1  ,  false , -1 ); //normalement c'est 0 mais à revoir 
            let ind : u32 = self.nb_pages_vecteur;
            //let mut list : ByteBuffer = self.liste_buffer[ind as usize]; 
            self.liste_pages[ind as usize] = pageinfo; 
            self.disk_manager.read_page(&page_id,&mut self.liste_buffer[ind as usize] ); 
            self.nb_pages_vecteur+=1; 
            self.compteur_temps+=1; //A REVOIR ON LE SET JAMAIS DANS LA PAGE

            return &mut self.liste_buffer[ind as usize];
        } 
        else{
             //1ere vérif pour le cas où une place dans le buffer n'est pas encore allouée
            for i in 0..self.liste_pages.len(){
                
                if page_id.get_FileIdx()==self.liste_pages[i].get_page_id().get_FileIdx() && page_id.get_PageIdx()==self.liste_pages[i].get_page_id().get_PageIdx(){
                    // pin count ++ quand on est sûr que la page est bien allouée
                    let setpin=self.liste_pages[i].get_pin_count()+1;
                    self.liste_pages[i].set_pin_count(setpin);
                    self.liste_pages[i].set_time(self.compteur_temps as i32); // à voir ça, il faut vérifier si on met le compteur au bon moment
                    return &mut self.liste_buffer[i];
                }

            }

            //Pour le cas où un une page est à remplacer --> indice de la page à changer 
            let mut page_a_changer :usize;

            if self.algo_remplacement.eq("LRU"){
                page_a_changer=self.lru();
            }else{
                page_a_changer=self.mru();
            }
            if  self.liste_pages[page_a_changer].get_pin_count()==0{
                if self.liste_pages[page_a_changer].get_dirty()==true{
                    self.disk_manager.write_page(&page_id, &mut self.liste_buffer[page_a_changer]);
                }
                self.disk_manager.read_page(&page_id, &mut self.liste_buffer[page_a_changer]);
                let pageinfo  : PageInfo = PageInfo::new( page_id.clone(), 1  ,  false , self.compteur_temps as i32 ); 
                self.liste_pages[page_a_changer] = pageinfo;  //il faut mettre le page info correspondant dans la liste des pages
            }
            &mut self.liste_buffer[page_a_changer]
        }
    }

     // ATTTENTION À VÉRIFIER ABSOLUMENT JE SUIS PAS CONFIANT DU TOUT POUR CA  
     
     pub fn free_page(&mut self,mut page_id:PageId,bit_dirty:bool)->(){
        //self.compteur_temps+=1; on incrémente pas le temps quand on fait un free
        let mut page_info:&mut PageInfo=&mut PageInfo::new(page_id,0,false,0); //CETTE LIGNE GROS GROS PROBLEME, AU NIVEAU LIFE TIME C'EST UNE DINGUERIE
        let mut trouve:bool=false;
        for i in self.liste_pages.iter_mut(){
            if page_id.get_FileIdx()==i.get_page_id().get_FileIdx() && page_id.get_PageIdx()==i.get_page_id().get_PageIdx(){
                page_info=i;
                trouve=true;
                break;
            }
        }
        if !trouve{
            return;
        }
        let index=page_info.get_pin_count()-1;
        page_info.set_pin_count(index);
        if(page_info.get_pin_count()==0){
              page_info.set_time(self.compteur_temps as i32);
        }
    }

    pub fn flush_buffers(&mut self){
        for i in 0..self.nb_pages_vecteur{
            if self.liste_pages[i as usize].get_dirty()==true{
                self.disk_manager.write_page(self.liste_pages[i as usize].get_page_id(),&mut self.liste_buffer[i as usize]);
                self.liste_pages[i as usize].set_pin_count(0);
            }
        }
        self.nb_pages_vecteur=0;
    }


}   