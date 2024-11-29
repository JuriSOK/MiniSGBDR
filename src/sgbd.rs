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
    db_manager : &'a mut DBManager<'a>,
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
            db_manager: &mut DBManager::new(&db, rc_bfm), //DBManager::new(&db, Rc::clone(&rc_bfm))
        }

    }
    pub fn run(&self) {
        let mut saisie: String = String::from("johnmatrix");
        while(saisie != "q".to_string()){
            //code complètement emprunté sur le forum rust, ne pas me demander comment ça fonctionne
            let _ = stdout().flush();
            stdin().read_line(&mut saisie).expect("Did not enter a correct string");
            if let Some('\n')=saisie.chars().next_back() {
                saisie.pop();
            }
            if let Some('\r')=saisie.chars().next_back() {
                saisie.pop();
            }
            match saisie.as_str() {
                s if s.starts_with("QUIT") => {self.process_quit_command(&saisie); saisie = "q".to_string()},
                s if s.starts_with("CREATE DATABASE") => self.process_create_data_base_command(&saisie.split_whitespace().next_back().unwrap().to_string()), //la on prend la chaine de caractere on la transforme en iterateur et on prend le dernier element, en esperant que ca soit le nom de la BDD
                s if s.starts_with("SET DATABASE") => self.process_set_data_base_command(&saisie.split_whitespace().next_back().unwrap().to_string()),
                s if s.starts_with("DROP DATABASES") => self.process_drop_data_bases_command(&saisie.split_whitespace().next_back().unwrap().to_string()),
                s if s.starts_with("LIST DATABASES") => self.process_list_data_bases_command(&saisie.split_whitespace().next_back().unwrap().to_string()),
                s if s.starts_with("CREATE TABLE") => self.process_create_table_command(&saisie.split_whitespace().next_back().unwrap().to_string()),
                s if s.starts_with("DROP TABLE") => self.process_drop_table_command(&saisie.split_whitespace().next_back().unwrap().to_string()),
                s if s.starts_with("DROP TABLES") => self.process_drop_tables_command(&saisie.split_whitespace().next_back().unwrap().to_string()),
                s if s.starts_with("LIST TABLES") => self.process_list_tables_command(&saisie.split_whitespace().next_back().unwrap().to_string()),
                _ => println!("{} n'est pas une commande", saisie),
            }
        }
    }
    pub fn process_quit_command(&self, commande: &String) {
        self.db_manager.save_state();
        let mut dm = DiskManager::new(&self.dbconfig);
        dm.save_state();
        self.buffer_manager.borrow_mut().flush_buffers();
    }
    pub fn process_create_data_base_command(&self, commande: &String) {
        self.db_manager.create_data_base(commande);
    }
    pub fn process_set_data_base_command(&self, commande: &String) {
        self.db_manager.set_current_data_base(commande);
    }
    pub fn process_list_data_bases_command(&self, commande: &String) {
        self.db_manager.list_databases()
    }
    pub fn process_create_table_command(&self, commande: &String) {
        let infos = commande.split_whitespace().collect::<Vec<&str>>();
        let name = infos[0].to_string();
        let table = infos[1];
        let _ = table.next(); //on eneleve la premiere parenthese
        let _ = table.next_back(); //on eneleve la derniere parenthese fermante
        let table_infos = table.split([',']).collect::<Vec<&str>>();
        let mut vec_cols = Vec::new();

        for chaine in table_infos {
            /*
            let tmp = ColInfo::new(chaine.split(':').collect::<Vec<&str>>()[0], chaine.split(':').collect::<Vec<&str>>()[1]);
            vec_cols.push(tmp);
            */
            vec_cols.push(ColInfo::new(chaine.split(':').collect::<Vec<&str>>()[0], chaine.split(':').collect::<Vec<&str>>()[1])); //pour faire un vec de col info
        }

        self.db_manager.add_table_to_current_data_base(Relation::new(name, vec_cols, Rc::clone(&self.buffer_manager)));
    }
    pub fn process_drop_table_command(&self, commande: &String) {
        //desallouer toutes les pages de la table, header page + data page j'imagine
        let table = self.db_manager.get_table_from_current_data_base(commande).unwrap();
        let hp_id = table.get_header_page_id();
        let page_ids = table.get_data_pages();
        let dm = self.buffer_manager.borrow().get_disk_manager();
        dm.borrow().dealloc_page(hp_id.clone());
        for page_id in page_ids {
            dm.borrow().dealloc_page(page_id);
        }
        self.db_manager.remove_table_from_current_data_base(commande);
    }
    pub fn process_drop_tables_command(&self, commande: &String) {
        let mut tables = self.db_manager.get_bdd_courante().unwrap().get_relations();
        let mut page_ids = Vec::new();
        let dm = self.buffer_manager.borrow().get_disk_manager();
        for rel in tables {
            page_ids.push(rel.get_header_page_id().clone());
            page_ids.append(&mut rel.get_data_pages());
        }
        for page in page_ids {
            dm.borrow().dealloc_page(page);
        }
        self.db_manager.remove_tables_from_current_data_base();
    }
    pub fn process_drop_data_bases_command(&self, commande: &String) {
        let bdds = self.db_manager.get_basededonnees().keys().collect::<Vec<&String>>();
        for bdd in bdds {
            self.db_manager.set_current_data_base(bdd);
            self.process_drop_tables_command(commande); //normalement ça supprime toutes les bdd en les passant d'abord en bdd courante pour ensuite pouvoir utiliser DROP TABLES
        }
        self.db_manager.remove_data_bases();
    }
    pub fn process_list_tables_command(&self, commande: &String) {
        self.db_manager.list_tables_in_current_data_base();
    }
}