use std::fs::File;
use serde_json::{Result, Value};

pub struct DBConfig {
    dbpath: String,
    pagesize: u32,
    dm_maxfilesize: u32,
}

impl DBConfig { //permet d'implémenter la structure (en gros c'est la classe en elle même et struct c'est juste pour mettre les valeurs je pense)
    pub fn new(chemin: String, pagesize : u32, dm_maxfilesize: u32 ) -> Self{ //Constructeur de la classe
        Self{ //dans ce scope on met les attributs de la classe
            dbpath: chemin, 
            pagesize : pagesize,
            dm_maxfilesize : dm_maxfilesize,
        }
    }

    pub fn set_dbpath(&mut self, chemin: String){
        self.dbpath = chemin;
    }
    
    pub fn get_dbpath(&self) -> &String{
        &self.dbpath
    }

    pub fn get_page_size(&self) -> u32{
        self.pagesize
    }

    pub fn get_dm_maxfilesize(&self) -> u32 {
        self.dm_maxfilesize
    }


    pub fn load_db_config(fichier_config: String) -> DBConfig {
        let file = File::open(fichier_config).expect("file should open read only test"); //OUVRE LE FICHIER
        let valeur: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON"); // RECUPERE LE CONTENUE
        
        let dbpath: String = valeur["dbpath"].as_str().unwrap().to_string();
        let pagesize:u32 =  valeur["pagesize"].as_str().unwrap().to_string().parse().expect("Not a number");
        let dm_maxfilesize: u32 =  valeur["dm_maxfilesize"].as_str().unwrap().to_string().parse().expect("Not a number");
        return DBConfig::new(dbpath, pagesize,dm_maxfilesize); // Sans to_string ça renvoie un truc bizarre
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_constructeur() {
        let s = String::from("./src/dbpath/BinData");
        let ps_test: u32 = 32 ;
        let dm_max_test : u32 = 64;

        let classe = DBConfig::new(s,ps_test,dm_max_test);
        assert_eq!(classe.dbpath, "./src/dbpath/BinData" );
        assert_eq!(classe.pagesize, 32 );
        assert_eq!(classe.dm_maxfilesize, 64);
        
        
        
    }

    #[test]
    fn test_load_db_config() {
        let chemin_json = String::from("../PROJET_BDDA/res/fichier.json");
        let classe = DBConfig::load_db_config(chemin_json);
        assert_eq!(classe.dbpath, "./src/dbpath/BinData" );
        assert_eq!(classe.pagesize, 32 );
        assert_eq!(classe.dm_maxfilesize, 64 );
        
    }
}

