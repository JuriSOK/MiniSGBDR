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
mod condition;  
mod types;
mod operator;
mod select;


use config::DBConfig;
use crate::page::PageId;
use crate::sgbd::SGBD;

fn main() {

    let chemin_json = ("config.json").to_string();
    let dbc = DBConfig::load_db_config(chemin_json);
    let mut sgbd = SGBD::new(&dbc);
    println!(" .----------------.  .----------------.  .----------------.  .----------------. ");
    println!("| .--------------. || .--------------. || .--------------. || .--------------. |");
    println!("| |    _______   | || |    ______    | || |   ______     | || |  ________    | |");
    println!("| |   /  ___  |  | || |  .' ___  |   | || |  |_   _ \\    | || | |_   ___ `.  | |");
    println!("| |  |  (__ \\_|  | || | / .'   \\_|   | || |    | |_) |   | || |   | |   `. \\ | |");
    println!("| |   '.___`-.   | || | | |    ____  | || |    |  __'.   | || |   | |    | | | |");
    println!("| |  |`\\____) |  | || | \\ `.___]  _| | || |   _| |__) |  | || |  _| |___.' / | |");
    println!("| |  |_______.'  | || |  `._____.'   | || |  |_______/   | || | |________.'  | |");
    println!("| |              | || |              | || |              | || |              | |");
    println!("| '--------------' || '--------------' || '--------------' || '--------------' |");
    println!(" '----------------'  '----------------'  '----------------'  '----------------' ");

    sgbd.run();
    
   
}

    
