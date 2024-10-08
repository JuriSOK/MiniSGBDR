use std::ptr::null;
use std::string::ToString;
use bytebuffer::ByteBuffer;
use file::get;
use crate::{config::DBConfig,
            disk_manager::{self, DiskManager},
            page_info::PageInfo};

pub struct BufferManager<'a>{
    db_config:&'a DBConfig,
    disk_manager:&'a DiskManager<'a>,
    liste_pages:&'a mut Vec<PageInfo>,
    liste_buffer:&'a Vec<ByteBuffer>,
    compteur:u64,
    algo_remplacement:&'a String,

    
     //Vecteur de pages libres courante

}
fn get_lru()->String{
    String::from("LRU")
}
fn get_mru()->String{
    String::from("MRU")
}
impl<'a> BufferManager<'a>{
    pub const LRU:&'a str="LRU";
    pub const MRU:&'a str="MRU";


    pub fn new(db_config:&'a DBConfig,
               disk_manager:&'a DiskManager,
               liste_pages:&'a mut Vec<PageInfo>,
               liste_buffer:&'a Vec<ByteBuffer>,
               algo_remplacement:&'a String
    )->Self
    {
        let compteur:u64=0;
        Self { db_config,
            disk_manager,
            liste_pages,
            liste_buffer,
            compteur,
            algo_remplacement
        }
    }

    pub fn set_current_replacement_policy(&mut self,policy:&'a String){
        self.algo_remplacement=policy;
    }

    pub fn Lru(&mut self){
        let mut pi:Option<&mut PageInfo>=None;
        let indice:usize;
        for i in 0..self.liste_pages.len(){
            let c=self.liste_pages[i].get_pin_count();
            if(c==0){
                if(pi.is_none()){
                    pi=Some(&mut self.liste_pages[i]);
                }
                if(pi.is_some()){

                }
            }
        }
    }

    /*
    pub fn flush_buffers(&self){
        for i  in self.liste_pages.iter(){
            if i.get_dirty()==0{
                let tmp:PageId=PageId::new("")
                self.disk_manager.write_page(tmp, )
            }
        }
        for i in 0..
    }*/
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_constructeur() {
        let s = String::from("res/dbpath/BinData");
        let ps_test: u32 = 32 ;
        let dm_max_test : u32 = 64;
        let bm_buffer_count : u32 = 4; 
        let bm_policy : String = String ::from("LRU"); 
        let config = DBConfig::new(s,ps_test,dm_max_test, bm_buffer_count,bm_policy);
        let dm= DiskManager::new(&config);
        
        
        let classe = BufferManager::new(&config, &dm);
        assert_eq!(classe.db_config.get_dbpath(), "res/dbpath/BinData" );
        assert_eq!(classe.disk_manager.get_free_pages(), dm.get_free_pages());
       
        
    }
}
