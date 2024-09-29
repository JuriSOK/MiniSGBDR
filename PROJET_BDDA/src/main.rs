mod config;
mod page;
mod DiskManager;
use config::DBConfig;
fn main() {
    
    let chemin = String::from("./src/dbpath/BinData");
    let ps: u32 = 32 ;
    let dm_max : u32 = 64;
    let instance_1 = DBConfig::new(chemin,ps,dm_max);
    println!("{}", instance_1.get_dbpath());
    
    println!("\n");
    
    let chemin_json = String::from("../PROJET_BDDA/res/fichier.json");
    let instance_2 = DBConfig::load_db_config(chemin_json);
    println!("{}", instance_2.get_dbpath());
    
}
