mod config;
use config::DBConfig;

fn main() {
    
    let chemin = String::from("chemin/de/la/base_de_donnee_chaine_de_caractere");
    let instance_1 = DBConfig::new(chemin);
    println!("{}", instance_1.get_dbpath());
    
    let chemin_json = String::from("/home/shrek/Documents/ProjetBDDARust/MiniSGBDR/PROJET_BDDA/src/fichier.json");
    let instance_2 = DBConfig::load_db_config(chemin_json);
    println!("{}", instance_2.get_dbpath());
    
}
