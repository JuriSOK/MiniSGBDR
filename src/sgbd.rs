use crate::DBConfig;
use std::collections::HashMap;
use std::option::Option;
use crate::col_info::ColInfo;
use crate::data_base::Database;
use crate::relation::Relation;
use std::rc::Rc;
use std::cell:: RefCell;
use crate::buffer_manager::BufferManager;
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};
use std::fs::File;
use std::io::Read;
use crate::PageId;
use crate::buffer_manager; 
use crate::db_manager::DBManager; 
use crate::disk_manager::DiskManager; 
pub struct Sgbd<'a> {
    dbconfig : &'a DBConfig,
    buffer_manager : Rc<RefCell<BufferManager<'a>>>,
    db_manager : DBManager<'a>,

}

impl <'a>Sgbd<'a> {
    pub fn new(self, db : DBConfig) -> Self {
        /*
        let mut tmp = DiskManager::new(&db);
        tmp.load_state();

        let tmp_buffer_m = BufferManager::new(&db,tmp,"LRU".to_string());
        Sgbd {
            dbconfig: &db,
            buffer_manager : tmp_buffer_m,
            db_manager : DBManager::new(&db, Rc::new(RefCell::new(tmp_buffer_m))),
        }
        */

        let mut dk = DiskManager::new(&db);
        dk.load_state();
        let rc_bfm = Rc::new(RefCell::new(BufferManager::new(&db, dk, "LRU".to_string())));
        let mut dbm = DBManager::new(&db, Rc::clone(&rc_bfm));
        dbm.load_state();

        Sgbd{
            dbconfig: &db,
            buffer_manager: Rc::clone(&rc_bfm),
            db_manager: DBManager::new(&db, rc_bfm), //DBManager::new(&db, Rc::clone(&rc_bfm))
        }

    }
    pub fn run(&self) {
        let mut saisie: String = String::from("johnmatrix");
        while(saisie !="q"){
            //code complètement emprunté sur le forum rust, ne pas me demander comment ça fonctionne
            let _ = stdout().flush();
            stdin().read_line(&mut saisie).expect("Did not enter a correct string");
            if let Some('\n')=saisie.chars().next_back() {
                saisie.pop();
            }
            if let Some('\r')=saisie.chars().next_back() {
                saisie.pop();
            }
            match saisie {
                String::from("QUIT") => self.process_quit_command(),
                String::from("CREATE DATABASE") => self.process_create_data_base_command(),
                String::from("SET DATABASE") => self.process_set_data_base_command(),
                String::from("DROP DATABASES") => self.process_drop_data_bases_command(),
                String::from("LIST DATABASES") => self.process_list_data_bases_command(),
                String::from("CREATE TABLE") => self.process_create_table_command(),
                String::from("DROP TABLE") => self.process_drop_table_command(),
                String::from("LIST TABLES") => self.process_list_tables_command(),
                _ => println!("{} n'est pas une commande", saisie),
            }
        }
    }
    pub fn process_quit_command(&self, commande: String) {}
    pub fn process_create_data_base_command(&self, commande: String) {}
    pub fn process_set_data_base_command(&self, commande: String) {}
    pub fn process_list_data_bases_command(&self, commande: String) {}
    pub fn process_drop_data_bases_command(&self, commande: String) {}
    pub fn process_create_table_command(&self, commande: String) {}
    pub fn process_drop_table_command(&self, commande: String) {}
    pub fn process_list_tables_command(&self, commande: String) {}



}

