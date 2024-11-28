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
use std::io::Write;
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
    pub fn new(db : DBConfig) -> Self {

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

        let rc_bfm = Rc::new(RefCell::new(BufferManager::new(&db, DiskManager::new(&db), "LRU".to_string())));
        Sgbd{
            dbconfig: &db,
            buffer_manager: Rc::clone(&rc_bfm),
            db_manager: DBManager::new(&db, rc_bfm), //DBManager::new(&db, Rc::clone(&rc_bfm))
        }
    }


}

