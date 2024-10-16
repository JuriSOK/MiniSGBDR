use crate::config::DBConfig;
use bytebuffer::ByteBuffer;
use serde::Deserialize;
use std::fs::File;
use crate::page::PageId; 
use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::OpenOptions;
use bincode;
use std::io::BufWriter;
use std::error::Error;


pub struct DiskManager<'a>{
    config:&'a DBConfig,

    free_pages: Vec<PageId>, //Vecteur de pages libres courante

}

impl<'a> DiskManager<'a>{
    pub fn new(config: &'a DBConfig) -> Self {

        let mut dm = Self {
            config,  // Assignation de la référence de configuration
            free_pages: Vec::new(), // Initialisation du vecteur de pages libres
            
        };
        
        if let Err(e) = dm.load_state() {
            eprintln!("Erreur lors du chargement de l'état : {}", e);
            // Ici, tu peux soit retourner une valeur par défaut, soit panique.
            // Par exemple, on peut panique si c'est critique :
            panic!("Échec de l'allocation de la page : {}", e);
        } // Charger l'état initial des pages libres

        dm // Retourner l'instance de DiskManager
    }
    
    
    
    pub fn get_free_pages(&self) -> &Vec<PageId> {
        &self.free_pages
    }


    pub fn alloc_page(&mut self) -> PageId {


        self.free_pages.clear();
        if let Err(e) = self.load_state() {
            eprintln!("Erreur lors du chargement de l'état : {}", e);
            // Ici, tu peux soit retourner une valeur par défaut, soit panique.
            // Par exemple, on peut panique si c'est critique :
            panic!("Échec de l'allocation de la page : {}", e);
        }

        if let Some(page_id) = self.free_pages.pop() {


            
            if let Err(e) = self.save_state() {
                eprintln!("Erreur lors de la sauvegarde de l'état : {}", e);
                // Tu peux choisir de panique ici aussi, ou de gérer l'erreur d'une autre manière.
                panic!("Échec de l'allocation de la page : {}", e);
            }
            
            return page_id;
        }

        let mut file_idx = 0;

        loop {
            
            let file_path = format!("{}/F{}.rsdb", self.config.get_dbpath(), file_idx);

            let mut file = OpenOptions::new().write(true).create(true).append(true).open(&file_path).unwrap();

            let current_size = file.metadata().unwrap().len() as u32;
            let page_size = self.config.get_page_size();
            let max_file_size = self.config.get_dm_maxfilesize();

            if current_size < max_file_size {
                let new_page_id = PageId::new(file_idx, (current_size / page_size));

                let forbidden_value = 0xFF; // On écrit dans la page une valeur interdite pour marquer la présence.
                let mut write_buffer = Vec::<u8>::new();
                let byte_array = [forbidden_value;32]; 
                write_buffer.extend_from_slice(byte_array.as_ref());

                   // Écriture du contenu du tampon dans le fichier
                if let Err(e) = file.write_all(&write_buffer) {
                    eprintln!("Erreur lors de l'écriture dans le fichier : {}", e);
                    panic!("Échec de l'écriture de la page : {}", e); // Ou gère l'erreur d'une autre manière
                }

    
            
                return new_page_id;
            }

            file_idx+= 1;

        }
        
       
    }

    //SI PAS COMPRIS, IL FAUT DEMANDER À MATHIEU
    pub fn read_page(&self, page_id: &PageId, buff: &mut Vec<u8>) -> Result<(), std::io::Error> { 
        //vérifier si page existe
        let num_fichier = page_id.get_FileIdx();
        let num_page = page_id.get_PageIdx();
        //println!("num_fichier: {}, num_page: {}", num_fichier, num_page);

        //Ouverture du fichier
        let mut fichier: File = OpenOptions::new()
        .read(true)
        .open(format!("res/dbpath/BinData/F{}.rsdb", num_fichier))?;

        //Placement du pointeur dans le fichier
        fichier.seek(SeekFrom::Start((num_page * self.config.get_page_size()) as u64))?; 
        
        //Creation d'un buffer temporaire pour stocker les données
        let mut temp_buffer:Vec<u8> = vec![0; self.config.get_page_size() as usize];

        //Lecture des données
        fichier.read_exact(&mut temp_buffer)?;

        //Ecriture des données dans le buffer
        buff.extend_from_slice(&temp_buffer);

        //Affichage du buffer
        //println!("buffer: {:?}", buff);
        Ok(())
    }

    //SI PAS COMPRIS, IL FAUT DEMANDER À MATHIEU
    pub fn write_page(&self, page_id: &PageId, buff: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        //faudrait vérifier que la page est libre je pense 
        let num_fichier = page_id.get_FileIdx();
        let num_page = page_id.get_PageIdx();

        //Ouverture du fichier avec les droits d'écriture et d'ajout
        let mut fichier: File =OpenOptions::new()
        .write(true)
        .append(false)
        .open(format!("res/dbpath/BinData/F{}.rsdb", num_fichier))?;

        //placement du pointeur dans le fichier
        fichier.seek(SeekFrom::Start((num_page * self.config.get_page_size()) as u64))?; //a faire aorès pour le ?

        //Ecriture des données dans le fichier
        //URGENT A DEMANDER A MATHIEU
        fichier.write_all(&buff[..self.config.get_page_size() as usize])?; // Use the write_all method //transformer le truc en vecteur avant de faire le write_all


        Ok(())
        //Utiliser le crate buffer avec un reader.

    }

    pub fn save_state(&self) -> std::io::Result<()> {
        let dm_save_path = format!("res/dbpath/dm.save");
    
        // Supprimer le fichier s'il existe, a priori ca marche sans maintenant mais dans le doute.
        let _ = std::fs::remove_file(&dm_save_path);
    
        // Ouvrir le fichier avec l'option de création s'il n'existe pas
        let fichier = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dm_save_path)?;
    
        // Utiliser BufWriter pour améliorer les performances lors de multiples écritures
        let mut writer = BufWriter::new(fichier);
    

        //On doit itérer 1 par 1 car si on sauvegarde tout le vecteur d'un coups, le premier élément sérialisé est la taille
        //Donc ca fausse notre manière de faire.
        // Itérer sur chaque élément du vecteur self.free_pages
        for page in &self.free_pages {
            // Sérialiser chaque page individuellement.
            //LIGNE OBSOLÈTE À CHANGER, MAIS JE VOUS CACHE PAS JE SUIS FATIGUÉ + FLEMME
            let contenu_sérialisé = bincode::serialize(&page)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.description()))?;
    
            // Écrire l'élément sérialisé dans le fichier
            writer.write_all(&contenu_sérialisé)?;
        }
    
        // Tout s'est bien passé
        Ok(())
    }


    pub fn load_state(&mut self) -> std::io::Result<()>{



        let dm_save_path = format!("res/dbpath/dm.save");
        let mut fichier = File::open(dm_save_path).expect("tkt");

        let mut contenu = Vec::new();
        fichier.read_to_end(&mut contenu).expect("tkt");

        //self.free_pages.clear();


        let mut pos = 0;


        while pos < contenu.len() {

            match bincode::deserialize::<PageId>(&contenu[pos..]) {
                Ok(instance) => {
                    self.free_pages.push(instance);

                    pos += bincode::serialized_size(&self.free_pages.last().unwrap()).unwrap() as usize;
                }
                Err(_) => break,
            }

        }
        Ok(())

    }


    pub fn dealloc_page(&mut self, page_id: PageId){

        if !self.free_pages.contains(&page_id) {
            self.free_pages.push(page_id);
            if let Err(e) = self.save_state() {
                eprintln!("Erreur lors de la sauvegarde de l'état : {}", e);
                // Tu peux choisir de panique ici aussi, ou de gérer l'erreur d'une autre manière.
                panic!("Échec de l'allocation de la page : {}", e);
            }
        }
        
    }


}




#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_constructeur() {
        let chemin = String::from("res/dbpath/BinData");
        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        assert_eq!(dm.config.get_dbpath(), "res/dbpath/BinData" );
        
    }

    #[test]
    fn test_write_page_and_read_page_and_alloc_page() {

        let config= DBConfig::load_db_config("res/fichier.json".to_string());
        let mut dm= DiskManager::new(&config);
        let page_id = dm.alloc_page(); //PageId::new(999,0);
        //TEST ÉCRITURE
        let mut write_buffer = Vec::<u8>::new();
        let byte_array = [3;32]; 
        write_buffer.extend_from_slice(byte_array.as_ref());
        
        dm.write_page(&page_id, &mut write_buffer).expect("write_page failed");


        //TEST LECTURE
        let mut read_buff = Vec::<u8>::new();
        dm.read_page(&page_id, &mut read_buff).expect("read_page failed");


        let expected_data = [3;32]; //PASSER LES BITS À 1 POUR FAIRE ÉCHOUER LE TEST
        let read_data = read_buff.clone();

        //TEST QUE LES DONNÉES ÉCRITE ET LUE SONT PAREILS
        assert_eq!(&read_data[..], &expected_data[..]);

    }

    #[test]
    fn test_alloc_page() {
        let config= DBConfig::load_db_config("res/fichier.json".to_string());
        let mut dm= DiskManager::new(&config);
        let page_id = dm.alloc_page();

    }



    #[test]
    fn test_dealloc_page() {
        let config= DBConfig::load_db_config("res/fichier.json".to_string());
        let mut dm= DiskManager::new(&config);
        let page_id = PageId::new(2, 1);
        dm.dealloc_page(page_id);
        let expected_page_id = PageId::new(2, 1);
        assert!(dm.free_pages.contains(&expected_page_id));


    }


    #[test]
    fn test_save_state() {


        //POUR TESTER SAVE_STATE() IL FAUT RETIRER LE SAVE_STATE DE DEALLOC !!!
        let config = DBConfig::load_db_config("res/fichier.json".to_string());
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

        //TEST À REFAIRE

        let config = DBConfig::load_db_config("res/fichier.json".to_string());
        let mut dm = DiskManager::new(&config);

    
    }
    
}
