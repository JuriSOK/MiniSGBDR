mod config;
mod page;
mod disk_manager;
mod buffer_manager;
mod page_info;
mod col_info;
mod record;
mod relation;
mod buffer;
mod record_id;
mod data_base;
mod db_manager;
mod sgbd;

use serde_json::Value::String;
use config::DBConfig;
//use std::io::Read;
use crate::page::PageId;
use crate::sgbd::SGBD;

fn main() {

    let chemin_json = ("res/fichier.json").to_string();
    let dbc = DBConfig::load_db_config(chemin_json);
    let mut sgbd = SGBD::new(&dbc);
    println!("---SGBD----");
    sgbd.run();
    /* 


    /*let chemin = String::from("res/dbpath/BinData");
    let ps: u32 = 32 ;
    let dm_max : u32 = 64;
    let instance_1 = DBConfig::new(chemin,ps,dm_max);
    println!("{}", instance_1.get_dbpath());
    
    println!("\n");
    
    let chemin_json = String::from("res/fichier.json");
    let instance_2 = DBConfig::load_db_config(chemin_json);
    println!("{}", instance_2.get_dbpath()); */


    let config= DBConfig::load_db_config("res/fichier.json".to_string());
    let mut dm= disk_manager::DiskManager::new(&config);

    let page_id_1 = dm.alloc_page();
    let mut write_buffer1 = Vec::new();
    let byte_array1 = [1;32]; 
    write_buffer1.extend_from_slice(byte_array1.as_ref());
    dm.write_page(&page_id_1, &mut write_buffer1).expect("write_page failed");

    println!("Pour les 1");
    println!("FileIdx : {}",page_id_1.get_FileIdx());
    println!("PageIdx : {}",page_id_1.get_PageIdx());
    ////
    let page_id_2 = dm.alloc_page();
    let mut write_buffer2 = Vec::new();
    let byte_array2 = [2;32]; 
    write_buffer2.extend_from_slice(byte_array2.as_ref());
    dm.write_page(&page_id_2, &mut write_buffer2).expect("write_page failed");

    println!("Pour les 2");
    println!("FileIdx : {}",page_id_2.get_FileIdx());
    println!("PageIdx : {}",page_id_2.get_PageIdx());
    ////
    let page_id_3 = dm.alloc_page();
    let mut write_buffer3 = Vec::new();
    let byte_array3 = [3;32]; 
    write_buffer3.extend_from_slice(byte_array3.as_ref());
    dm.write_page(&page_id_3, &mut write_buffer3).expect("write_page failed");

    println!("Pour les 3");
    println!("FileIdx : {}",page_id_3.get_FileIdx());
    println!("PageIdx : {}",page_id_3.get_PageIdx());
    ////
    ////
    let page_id_4 = dm.alloc_page();
    let mut write_buffer4 =  Vec::new();
    let byte_array4 = [4;32]; 
    write_buffer4.extend_from_slice(byte_array4.as_ref());
    dm.write_page(&page_id_4, &mut write_buffer4).expect("write_page failed");

    println!("Pour les 4");
    println!("FileIdx : {}",page_id_4.get_FileIdx());
    println!("PageIdx : {}",page_id_4.get_PageIdx());

    ///
    /// 
    let page_id_5 = dm.alloc_page();
    let mut write_buffer5 = Vec::new();
    let byte_array5 = [5;32]; 
    write_buffer5.extend_from_slice(byte_array5.as_ref());
    dm.write_page(&page_id_5, &mut write_buffer5).expect("write_page failed");

    println!("Pour les 5");
    println!("FileIdx : {}",page_id_5.get_FileIdx());
    println!("PageIdx : {}",page_id_5.get_PageIdx());

    ///
    /// 
    println!("Pour les 6");
    /// 
    let page_id_6 = dm.alloc_page();
    let mut write_buffer6 = Vec::new();
    let byte_array6 = [6;32]; 
    write_buffer6.extend_from_slice(byte_array6.as_ref());
    dm.write_page(&page_id_6, &mut write_buffer6).expect("write_page failed");

    println!("FileIdx : {}",page_id_6.get_FileIdx());
    println!("PageIdx : {}",page_id_6.get_PageIdx());

    ///
    /// 
    let page_id_7 = dm.alloc_page();
    let mut write_buffer7 = Vec::new();
    let byte_array7 = [5;32]; 
    write_buffer7.extend_from_slice(byte_array7.as_ref());
    dm.write_page(&page_id_7, &mut write_buffer7).expect("write_page failed");

    println!("Pour les 5 encore");
    println!("FileIdx : {}",page_id_7.get_FileIdx());
    println!("PageIdx : {}",page_id_7.get_PageIdx());


    dm.dealloc_page(page_id_1);
    dm.dealloc_page(page_id_2);
    dm.dealloc_page(page_id_3);
    //dm.dealloc_page(page_id_4);
    //dm.dealloc_page(page_id_5);
    //dm.dealloc_page(page_id_6);
    //dm.dealloc_page(page_id_7);

    

    println!("Page : ");
    for pageid in dm.get_free_pages() {
        println!("{:?}", pageid);
    }
    println!("AAAAAAAAAA");

    println!("Contenu de dm.save :");

    // Lire et afficher le contenu de dm.save
    let dm_save_path = "res/dbpath/dm.save";

    // Lire le fichier
    let mut fichier = File::open(dm_save_path).expect("Erreur lors de l'ouverture du fichier");
    let mut contenu = Vec::new();
    fichier.read_to_end(&mut contenu).expect("Erreur lors de la lecture du fichier");

    // Désérialiser les données
    let mut free_pages: Vec<PageId> = Vec::new();
    let mut pos = 0;

    while pos < contenu.len() {
        match bincode::deserialize::<PageId>(&contenu[pos..]) {
            Ok(instance) => {
                free_pages.push(instance);
                pos += bincode::serialized_size(&free_pages.last().unwrap()).unwrap() as usize;
            }
            Err(_) => {
                eprintln!("Erreur lors de la désérialisation à la position {}", pos);
                break;
            }
        }
    }

    // Afficher les PageId
    for page_id in free_pages {
        println!("PageId: {:?}", page_id);
    }

    */
   
}

    
