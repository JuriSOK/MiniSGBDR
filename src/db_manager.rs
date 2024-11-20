use std::borrow::Borrow;
use crate::DBConfig;
use std::env;
use std::collections::HashMap;
use serde::de::Unexpected::Option;
use crate::data_base::Database;
use crate::relation::Relation;

pub struct DBManager<'a> {
    basededonnees : HashMap<String, Database<'a>>,
    dbconfig : &'a DBConfig,
    bdd_courante : Option<&'a Database<'a>>
}

impl<'a> DBManager<'a> {
    pub fn new(db : &'a DBConfig) -> Self {
        DBManager{
            basededonnees : HashMap::new(),
            dbconfig : db,
            bdd_courante: None

        }
    }
    pub fn create_data_base(&mut self, db: &str){
        self.basededonnees.insert(db.to_string(), Database::new(db.to_string()));
    }
    pub fn set_current_data_base(&mut self, nom : &str)-> Self{
        match self.basededonnees.get(nom){
            Some(db) => self.bdd_courante = self.bdd_courante.get(nom),
            None => self.create_data_base(nom)
        }
    }
    pub fn add_table_to_current_data_base(&mut self, tab: Relation){
        self.bdd_courante.unwrap().add_relation(tab);
    }
    pub fn get_table_from_current_data_base(&mut self, nom_tab:&str)-> Option<Relation>{
        let rel_result: Option<Relation> = None;
        for rel in self.bdd_courante.unwrap().iter(){
            if rel.get_name() == nom_tab{
               rel_result = Some(rel);
            }
        }
        return rel_result;
    }
    pub fn remove_table_from_current_data_base(&mut self, nom_tab:&str){
        self.bdd_courante.unwrap().remove_relation(nom_tab);
    }
    pub fn remove_data_base(&mut self, nom_bdd:&str){

    }
}
