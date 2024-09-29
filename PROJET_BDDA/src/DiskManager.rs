use crate::config::DBConfig;
use bytebuffer::ByteBuffer;
use serde::Deserialize;
use std::fs::File;
use crate::page::PageId; 
use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::OpenOptions;
use bincode;


struct DiskManager<'a>{
    config:&'a DBConfig,

    free_pages: Vec<PageId>, //Vecteur de pages libres courante

}

impl<'a> DiskManager<'a>{
    pub fn new(config: &'a DBConfig) -> Self {

        let mut dm = Self {
            config,  // Assignation de la référence de configuration
            free_pages: Vec::new(), // Initialisation du vecteur de pages libres
            
        };
        
        dm.load_state(); // Charger l'état initial des pages libres
        dm // Retourner l'instance de DiskManager
    }

    pub fn alloc_page(&mut self) -> PageId {

        if let Some(page_id) = self.free_pages.pop() {
            self.save_state();
            return page_id;
        }

        let mut file_idx = 0;



        loop {
            

            let file_path = format!("{}/F{}.bin", self.config.get_dbpath(), file_idx);

            let mut file = OpenOptions::new().write(true).create(true).open(&file_path).unwrap();

            let current_size = file.metadata().unwrap().len() as u32;
            let page_size = self.config.get_page_size();
            let max_file_size = self.config.get_dm_maxfilesize();

            if current_size < max_file_size {
                let new_page_id = PageId::new(file_idx, (current_size / page_size));

                return new_page_id;
            }

            file_idx+= 1;

        }
        
       
    }

    //SI PAS COMPRIS, IL FAUT DEMANDER À MATHIEU
    pub fn read_page(&self, page_id: &PageId, buff: &mut ByteBuffer) -> Result<(), std::io::Error> { 
        let num_fichier = page_id.get_FileIdx();
        let num_page = page_id.get_PageIdx();
        //println!("num_fichier: {}, num_page: {}", num_fichier, num_page);

        //Ouverture du fichier
        let mut fichier: File = OpenOptions::new()
        .read(true)
        .open(format!("./src/dbpath/BinData/F{}.bin", num_fichier))?;

        //Placement du pointeur dans le fichier
        fichier.seek(SeekFrom::Start((num_page * self.config.get_page_size()) as u64))?; 
        
        //Creation d'un buffer temporaire pour stocker les données
        let mut temp_buffer = vec![0; self.config.get_page_size() as usize];

        //Lecture des données
        fichier.read_exact(&mut temp_buffer)?;

        //Ecriture des données dans le buffer
        buff.write_bytes(&temp_buffer);

        //Affichage du buffer
        //println!("buffer: {:?}", buff);
        Ok(())
    }

    //SI PAS COMPRIS, IL FAUT DEMANDER À MATHIEU
    pub fn write_page(&self, page_id: &PageId, buff: &mut ByteBuffer) -> Result<(), std::io::Error> {
        let num_fichier = page_id.get_FileIdx();
        let num_page = page_id.get_PageIdx();

        //Ouverture du fichier avec les droits d'écriture et d'ajout
        let mut fichier: File =OpenOptions::new()
        .write(true)
        .append(false)
        .open(format!("./src/dbpath/BinData/F{}.bin", num_fichier))?;

        //placement du pointeur dans le fichier
        fichier.seek(SeekFrom::Start((num_page * self.config.get_page_size()) as u64))?; //a faire aorès pour le ?

        //Ecriture des données dans le fichier
        //URGENT A DEMANDER A MATHIEU
        fichier.write_all(&buff.read_bytes(self.config.get_page_size() as usize)?)?; // Use the write_all method


        Ok(())
        //Utiliser le crate buffer avec un reader.

    }

    pub fn save_state(&self){

        let dm_save_path = format!("./src/dbpath/dm.save");

        let mut file = OpenOptions::new().write(true).truncate(true).open(&dm_save_path).unwrap();

        let binaire : Vec<u8> = bincode::serialize(&self.free_pages).unwrap();
        file.write_all(&binaire);

    }

    pub fn load_state(&mut self){


        let dm_save_path = format!("./src/dbpath/dm.save");

        let mut file = OpenOptions::new().read(true).open(&dm_save_path).unwrap();

       if file.metadata().unwrap().len() == 0 {
        return;
       }

        let mut resultats: Vec<u8> = Vec::new();
        file.read_to_end(&mut resultats);

        self.free_pages = bincode::deserialize(&resultats).expect("Failed to deserialize free pages");
 
    }


    pub fn dealloc_page(&mut self, page_id: PageId){

        if !self.free_pages.contains(&page_id) {
            self.free_pages.push(page_id);
            self.save_state();
        }
        
    }


}






#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_constructeur() {
        let chemin = String::from("./src/dbpath/BinData");
        let s: String = String::from("../PROJET_BDDA/res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        assert_eq!(dm.config.get_dbpath(), "./src/dbpath/BinData" );
        
    }

    #[test]
    fn test_write_page_and_read_page_and_alloc_page() {

        let config= DBConfig::load_db_config("../PROJET_BDDA/res/fichier.json".to_string());
        let mut dm= DiskManager::new(&config);
        let page_id = dm.alloc_page(); //PageId::new(999,0);
        //TEST ÉCRITURE
        let mut write_buffer = ByteBuffer::new();
        let byte_array = [0;32]; 
        write_buffer.write_bytes(byte_array.as_ref());
        
        dm.write_page(&page_id, &mut write_buffer).expect("write_page failed");


        //TEST LECTURE
        let mut read_buff = ByteBuffer::new();
        dm.read_page(&page_id, &mut read_buff).expect("read_page failed");

        let expected_data = [0;32]; //PASSER LES BITS À 1 POUR FAIRE ÉCHOUER LE TEST
        let read_data = read_buff.read_bytes(byte_array.len()).expect("Failed to read bytes from buffer");

        //TEST QUE LES DONNÉES ÉCRITE ET LUE SONT PAREILS
        assert_eq!(&read_data[..], &expected_data[..]);

    }

    #[test]
    fn test_dealloc_page() {
        let config= DBConfig::load_db_config("../PROJET_BDDA/res/fichier.json".to_string());
        let mut dm= DiskManager::new(&config);
        let page_id = PageId::new(995, 0);
        dm.dealloc_page(page_id);
        let expected_page_id = PageId::new(995, 0);
        assert!(dm.free_pages.contains(&expected_page_id));


    }


    #[test]
    fn test_save_state() {

        //POUR TESTER SAVE_STATE() IL FAUT RETIRER LE SAVE_STATE DE DEALLOC !!!
        let config = DBConfig::load_db_config("../PROJET_BDDA/res/fichier.json".to_string());
        let mut dm = DiskManager::new(&config);

        let page_id = PageId::new(999, 0);
        dm.dealloc_page(page_id);
        dm.save_state();

        let mut dm2 = DiskManager::new(&config);
        let expected_page_id = PageId::new(999, 0);
        assert!(dm2.free_pages.contains(&expected_page_id));

    }

    #[test]
    fn test_load_state() {

        let config = DBConfig::load_db_config("../PROJET_BDDA/res/fichier.json".to_string());
        let mut dm = DiskManager::new(&config);

        let expected_page_id = PageId::new(999, 0);
        assert!(dm.free_pages.contains(&expected_page_id));

    }
    

    

    

}