//use std::borrow::Borrow;
use crate::DBConfig;
use std::collections::HashMap;
use std::option::Option;
use crate::col_info::ColInfo;
use crate::data_base::Database;
use crate::relation::Relation;
use std::rc::Rc;
use std::cell:: RefCell;
use std::fs::{File, OpenOptions};
use crate::buffer_manager::BufferManager;
use std::io::{Read, Write, Seek, SeekFrom};
use std::error::Error;
pub struct DBManager<'a> {
    basededonnees : HashMap<String, Database<'a>>,
    dbconfig : &'a DBConfig,
    bdd_courante : Option<String>,
    buffer_manager : Rc<RefCell<BufferManager<'a>>>,
}

impl<'a> DBManager<'a> {
    pub fn new(db : &'a DBConfig, buffer_m : Rc<RefCell<BufferManager<'a>>>) -> Self {
        DBManager{
            basededonnees : HashMap::new(),
            dbconfig : db,
            bdd_courante: None::<String>,
            buffer_manager : buffer_m
        }
    }
    pub fn get_bdd_courante(&mut self) -> Option<&mut Database<'a>> {
        if self.bdd_courante.is_some() {
            return self.basededonnees.get_mut(self.bdd_courante.as_ref().unwrap());
        }else {
            return None;
        }
    }
    pub fn get_basededonnees(&self) -> &HashMap<String, Database<'a>> {
        return &self.basededonnees;
    }

    pub fn get_dbconfig(&self) -> &'a DBConfig {
        return self.dbconfig;
    }

    pub fn create_data_base(&mut self, db: &str){
        self.basededonnees.insert(db.to_string(), Database::new(db.to_string()));
    }
    pub fn set_current_data_base(&mut self, nom : &str){
        if self.basededonnees.contains_key(nom) {
            self.bdd_courante = Some(nom.to_string());
        }
        else {
            self.create_data_base(nom);
            self.bdd_courante = Some(nom.to_string());
        }
    }
    pub fn add_table_to_current_data_base(&mut self, tab: Relation<'a>){
        if self.bdd_courante.is_some() {
            self.get_bdd_courante().unwrap().add_relation(tab);
        }
    }
    pub fn get_table_from_current_data_base(&mut self, nom_tab:&str)-> Option<&Relation<'a>>{
        let bdd = self.get_bdd_courante().unwrap();
        let rel_bdd = bdd.get_relations();
        let mut rel_result = None;
        for rel in rel_bdd.iter(){
            if rel.get_name() == nom_tab{
               rel_result = Some(rel);
            }
        }
        return rel_result;
    }
    pub fn remove_table_from_current_data_base(&mut self, nom_tab:&str){
        self.get_bdd_courante().unwrap().remove_relation(nom_tab);
    }
    pub fn remove_data_base(&mut self, nom_bdd:&str){
        if let Some(_db) = self.basededonnees.get(nom_bdd){
            self.basededonnees.remove(nom_bdd);
        }
        if self.get_bdd_courante().unwrap().get_nom() == nom_bdd{
            self.bdd_courante = None;
        }
    }
    pub fn remove_tables_from_current_data_base(&mut self){
        self.get_bdd_courante().unwrap().set_relations(Vec::new());
    }
    pub fn remove_data_bases(&mut self){
        self.bdd_courante = None;
        self.basededonnees.clear();
    }
    pub fn list_databases(&mut self){
        println!("Affichage des bases de données : ");
        println!("Base de données courante : {}", self.get_bdd_courante().unwrap().get_nom());
        for bdd in self.basededonnees.keys(){
            println!("Base de données : {}", bdd)
        }
    }
    pub fn list_tables_in_current_data_base(&mut self){
        let relations:&Vec<Relation> = self.get_bdd_courante().unwrap().get_relations();
        for rel in relations {
            println!("Table : {}, colonnes : ", rel.get_name());
            let cols:Vec<ColInfo> = rel.get_columns();
            for col in cols {
                println!("nom : {}, type : {}", col.get_name(), col.get_column_type());
            }
        }
    }
    pub fn save_state(&mut self)->Result<(),Box<dyn Error>>{


        for bdd in self.basededonnees.values(){
            for relation in bdd.get_relations(){
                let buffer = self.buffer_manager.borrow_mut().get_page(relation.get_header_page_id()) ;
                let mut fichier: File =OpenOptions::new()
                    .write(true)
                    .append(false)
                    .open(format!("res/dbpath/LoadBDD/File.rsdb"))?;
                let num_page = relation.get_header_page_id().get_page_idx();
                fichier.seek(SeekFrom::Start((num_page * self.dbconfig.get_page_size()) as u64)); //a faire aorès pour le ?
                let data_to_write = buffer.get_mut_buffer();
                fichier.write_all(&data_to_write.as_bytes())?;
            }

        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use std::cell::RefCell;
    use crate::DBConfig;
    use super::*;
    use std::rc::Rc;
    use crate::buffer_manager::BufferManager;
    use crate::disk_manager::DiskManager;

    #[test]
    fn test_bdd_courante(){
        let chemin = String::from("res/dbpath/BinData");
        let s: String = String::from("res/fichier.json");
        let mut config= DBConfig::load_db_config(s);
        let mut dm= DiskManager::new(&config);
        let algo_lru = String::from("LRU");

        let mut buffer_manager1 = BufferManager::new(&config, dm, &algo_lru);
        let rc_a = Rc::new(RefCell::new(buffer_manager1));

        let colinfo1: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(10)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(6)".to_string()),
        ];
        let mut relation1 = Relation::new("PERSONNE".to_string(),colinfo1.clone(),Rc::clone(&rc_a));

        let colinfo2: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(20)".to_string()),
            ColInfo::new("ID".to_string(), "INT".to_string()),
            ColInfo::new("SALAIRE".to_string(), "FLOAT".to_string()),
        ];
        let mut relation2 = Relation::new("EMPLOI".to_string(),colinfo2.clone(),Rc::clone(&rc_a));

        let colinfo3: Vec<ColInfo> = vec![
            ColInfo::new("MARQUE".to_string(), "CHAR(20)".to_string()),
            ColInfo::new("MODELE".to_string(), "VARCHAR(10)".to_string()),
            ColInfo::new("ID".to_string(), "INT".to_string()),
            ColInfo::new("PUISSANCE".to_string(), "INT".to_string()),
            ColInfo::new("PRIX".to_string(), "FLOAT".to_string()),
        ];
        let mut relation3 = Relation::new("VOITURE".to_string(),colinfo3.clone(),Rc::clone(&rc_a));

        let colinfo4: Vec<ColInfo> = vec![
            ColInfo::new("MARQUE".to_string(), "CHAR(20)".to_string()),
            ColInfo::new("MODELE".to_string(), "VARCHAR(10)".to_string()),
            ColInfo::new("ID".to_string(), "INT".to_string()),
            ColInfo::new("PUISSANCE".to_string(), "INT".to_string()),
            ColInfo::new("CARBURANT".to_string(), "CHAR(10)".to_string()),
            ColInfo::new("PRIX".to_string(), "FLOAT".to_string()),
        ];
        let mut relation4 = Relation::new("TRACTEUR".to_string(),colinfo4.clone(),Rc::clone(&rc_a));

        let mut db_manager = DBManager::new(&config, Rc::clone(&rc_a));
        db_manager.create_data_base("carrefour");
        db_manager.set_current_data_base("carrefour");
        db_manager.add_table_to_current_data_base(relation1);
        db_manager.add_table_to_current_data_base(relation2);

        db_manager.list_databases();
        db_manager.list_tables_in_current_data_base();

        db_manager.create_data_base("concession");
        db_manager.set_current_data_base("concession");
        db_manager.add_table_to_current_data_base(relation3);
        db_manager.add_table_to_current_data_base(relation4);

        db_manager.list_databases();
        db_manager.list_tables_in_current_data_base();
    }
}