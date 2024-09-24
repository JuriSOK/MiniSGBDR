use std::fs::File;
use serde_json::{Result, Value};

pub struct DBConfig {
    dbpath: String,
}

impl DBConfig { //permet d'implémenter la structure (en gros c'est la classe en elle même et struct c'est juste pour mettre les valeurs je pense)
    pub fn new(chemin: String) -> Self{ //Constructeur de la classe
        Self{ //dans ce scope on met les attributs de la classe
            dbpath: chemin, 
        }
    }

    pub fn set_dbpath(&mut self, chemin: String){
        self.dbpath = chemin;
    }
    
    pub fn get_dbpath(&self) -> &String{
        &self.dbpath
    }
    
    pub fn load_db_config(fichier_config: String) -> DBConfig {
        let file = File::open(fichier_config).expect("file should open read only test"); //OUVRE LE FICHIER
        let valeur: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON"); // RECUPERE LE CONTENUE
        return DBConfig::new(valeur["dbpath"].as_str().unwrap().to_string()); // Sans to_string ça renvoie un truc bizarre
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_constructeur() {
        let s = String::from("chemin/de/la/base_de_donnee_chaine_de_caractere");
        let classe = DBConfig::new(s);
        assert_eq!(classe.dbpath, "chemin/de/la/base_de_donnee_chaine_de_caractere" );
    }
    
    #[test]
    fn test_methode_load_db_config() {
        let chemin_json = String::from("../PROJET_BDDA/res/fichier.json");
        let classe = DBConfig::load_db_config(chemin_json);
        assert_eq!(classe.dbpath, "chemin/de/la/base_de_donnee_json" );
    }
}

