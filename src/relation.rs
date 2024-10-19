use bytebuffer::ByteBuffer;

use string_builder::Builder;
use crate::col_info::ColInfo;
use crate::record::Record;
use std::fs::OpenOptions;
use std::io::{self, Write};


pub struct Relation { //PERSONNE(NOM,PRENOM?,AGE)
    name:String,
    columns: Vec<ColInfo>,
    nb_columns: usize,
}

impl Relation {

    pub fn new (name : String,columns:Vec<ColInfo>) -> Self{

        Relation {
            name: String::from(name),
            columns,
            nb_columns:columns.len(),

        }

    }

    fn get_name(&self)->&String {
        &self.name
    }

    // Getter pour les informations sur les colonnes
    fn get_columns(&self) -> Vec<ColInfo> {
        self.columns.clone()
    }

    pub fn write_record_to_buffer(&mut self, record:Record, buffer:&mut Vec<u8>, pos:usize)->usize{
        // Copie du tuple (pas obligatoire)
        let tuple = record.get_tuple().clone();
        //Pour avoir le nom des colonnes, le type etc...
        let mut compteur:usize=0;
        
        let mut indice:usize = pos; //pour la position

        // Initialisation de la taille d'un BUFFER
        

        // Pour savoir si une colonne type  varchar a été trouvé dans le record
        let mut varchar_trouve:bool=false;

        // Recherche d'un ou plusieurs VARCHAR dans le tuple.
        for i in 0..self.columns.len(){
            if self.columns[i].get_column_type().starts_with("VARCHAR"){
                println!("{:?}", self.columns[i].get_name());
                varchar_trouve=true;
                break;
            }      
        }
        println!("{:?}", varchar_trouve);

        let mut taille_objets:Vec<usize> =Vec::new();
        
        
        if varchar_trouve{
            // Stockage des longueurs en octet de chaque attribut
            //pour faire les offset après
            for i in 0..tuple.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        taille_objets.push(4);
                        compteur+=4;
                        continue;
                    }
                    "REAL" => {
                        taille_objets.push(4);
                        compteur+=4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        
                        let substring: &str = &self.columns[i].get_column_type()[5..index.unwrap()];
                        println!("{:?}", index);
                        let nbytes=" ".repeat(substring.parse::<usize>().unwrap()).as_bytes().len();
                        taille_objets.push(nbytes);
                        compteur+=4; // COMPTEUR + LA TAILLE DE LA CHAINE
                        continue;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let index:Option<usize> = s2.find(')') ;
                        let substring: &str = &self.columns[i].get_column_type()[8..index.unwrap()];
                    
                        let len_s=substring.parse::<usize>().unwrap();
                        let nbytes=if len_s>=tuple[i].len(){
                            tuple[i].as_bytes().len()
                        }else{
                            // A REVOIR
                            " ".repeat(len_s).as_bytes().len()
                        };
                        taille_objets.push(nbytes);
                        compteur+=4;
                        continue;
                    }
                    _ => {} //default du match 
                }
            }
            compteur+=4;
            
            //Ecriture des longueurs des attributs
            //ça met les offset dans le buffer
            let mut compteur2=compteur;
            for i in 0..taille_objets.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        let bytes=(compteur2 as u32).to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur2+=4;
                        indice+=4;
                        continue;
                    }
                    "REAL" => {
                        let bytes=(compteur2 as u32).to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur2+=4;
                        indice+=4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        let taille=taille_objets[i];
                        let bytes=(compteur2 as u32).to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur2+=taille;
                        indice+=4;
                        continue;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let taille=taille_objets[i];
                        let bytes=(compteur2 as u32).to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur2+=taille;
                        indice+=4;
                        continue;
                    }
                    _ => {} //default du match
                }
            }
            let bytes=(compteur2 as u32).to_be_bytes();
            buffer[indice..indice + 4].copy_from_slice(&bytes);
            indice=pos+compteur;
            // Ecriture des valeurs des attributs
            for i in 0..taille_objets.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        //on transforme la valeur dans le tuple en octets
                        let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur+=4;// Pour les 4 octets de l'entier
                        indice += 4;
                        continue;
                    }
                    "REAL" => {
                        let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur+=4;// Pour les 4 octets du reel
                        indice += 4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        // 3 ligne en dessous : taille entre parentheses : (12) par exemple
                        /*
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s: usize=substring.parse::<usize>().unwrap();
                        */
                    
                        let mut bytes = tuple[i].as_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur+= bytes.len();
                        indice += bytes.len();
                        continue;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        /*
                        let index:Option<usize> = s2.find(')') ;
                        let substring: &str = &tuple[i][8..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        */
                        let nbytes=tuple[i].as_bytes().len();
                        let mut bytes = tuple[i].as_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur += nbytes;
                        indice += nbytes;
                        continue;
                    }
                    _ => {}
                }
            }

        }else{// Si pas de VARCHAR dans le tuple 
            
            // FACILE à comprendre, le code est transparent :)
            for i in 0..self.nb_columns{
                println!("{:?}", self.nb_columns);
                let tmp=tuple[i].clone();
                
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                         //on transforme la valeur dans le tuple en octets
                        let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur+=4;// Pour les 4 octets de l'entier
                        indice += 4;
                        continue;
                        
                    }
                    "REAL" => {
                        let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur += 4;// Pour les 4 octets du reel
                        indice += 4;
                        continue;
                    } // CHAR(20) --> 20 CARACTERES = 20 OCTETS
                    s if s.starts_with("CHAR")  => {
                        /*
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        */
                    
                        let mut bytes = tuple[i].as_bytes();
                        println!("{:?}", buffer.len());
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur += bytes.len();
                        indice += bytes.len();
                        
                        continue;
                        
                    }
                    
                    _ => {} //default du match
                }

            }
        }
        return compteur;
        
        
    }
    
    pub fn read_from_buffer(un_record: &mut record, buff: Vec<u8>, int pos) -> int {
    
        let tuple = Vec::new();
        let varchar = false;
        let nb_octets_lus = 0;
        
        //on regarde si on a un varchar :
        for i in 0..self.nb_columns{
            if columns[i].get_column_type().as_str().starts_with("VARCHAR"){
                varchar = true;
            }
        }
        
        //cas ou on a un varchar, du coup on aura des offsets
        if varchar {
            //la taille de la valeur est donnée par le offset impair et le offset qui le suit. (debut de la valeur dans le buffer et la fin de celle-ci)
            let offset_debut = 0;
            let offset_fin = 0;
            //on doit mettre dans le tuple les valeurs qui commencent après les offsets
            for i in 0..self.nb_columns{
                offset_debut = buff[pos+i] as u32; //on convertit la valeur en entier (je sais pas si ça fonctionne ça, à méditer)
                offset_fin = buff[pos+i+1] as u32;
                //on met dans le tuple le sous_vecteur correspondant à la valeur, en chaine de caractere
                tuple.push(&buff[offset_debut..offset_fin].to_string());
                nb_octets_lu += offset_fin - offset_debut; //pour recup le nb d'octets lus, pas sur de ce que je fais là 
            }
        }
        else{
            let compteur_pos = pos;
            for i in 0..self.nb_columns{
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        tuple.push(&buff[compteur_pos..compteur_pos+4]);
                        compteur_pos += 4;
                        nb_octets_lus += 4;
                        continue;
                    }
                    "REAL" => {
                        tuple.push(&buff[compteur_pos..compteur_pos+4]);
                        compteur_pos += 4;
                        nb_octets_lus += 4;
                        continue;
                    }
                    s if s.starts_with("CHAR")  => {
                        let indice_parenthese_ouvrante = s.find("(");
                        let indice_parenthse_fermante = s.find(")"); //on prend les deux parenthèses comme on connait pas le chiffre on connait pas la taille de son string correspondant
                        let taille_char = s[indice_parenthese_ouvrante+1..indice_parenthese_fermante].parse::<i32>().unwrap();
                        tuple.push(&buff[compteur_pos..compteur_pos+taille_char]);
                        compteur_pos += taille_char;
                        nb_octets_lus += taille_char;
                        continue;
                    }
                    
                    _ => {} //default du match
                }
            }
        }
        record.set_tuple(tuple);
        return nb_octets_lus;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_write_varchar(){
        let record = Record::new(vec!["APAGNANNAA".to_string(),"QUOICOUBEH".to_string(),"20".to_string()]);
        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(10)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(10)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new("PERSONNE".to_string(),colinfo.clone(),3);
        let pos=0; 
        
        let mut buffer:Vec<u8> =vec![0;40];
        
        relation.write_record_to_buffer(record, &mut buffer, pos);
        println!("{:?}", buffer);
        //A lancer avec "cargo test test_write_varchar -- --nocapture" pour voir le println
    }
    #[test]
    fn test_apagnan() {
        //println!("{}",(4 as u32).to_be_bytes().len().to_string());
        let colinfo: Vec<ColInfo> = vec![
            ColInfo::new("NOM".to_string(), "CHAR(10)".to_string()),
            ColInfo::new("PRENOM".to_string(), "VARCHAR(10)".to_string()),
            ColInfo::new("AGE".to_string(), "INT".to_string()),
        ];
        let mut relation = Relation::new(String::from("PERSONNE"),colinfo.clone(), 3);

    // Ajout de colonnes à la relation en utilisant la méthode new de ColInfo
        

        // Exemple de création d'un Record
        let record = Record::new(vec![
        String::from("Dupont"),
        String::from("Jean"),
        String::from("30"),
        ]);

        let mut buffer: Vec<u8> = vec![0; 5000];
        

       relation.write_record_to_buffer(record, &mut buffer, 1);

        let s: String = String::from("res/fichier_test_write_relation");
        let mut fichier1 = OpenOptions::new().write(true).open(s).expect("tkt");

        fichier1.write_all(&buffer);
       
      

            
 

  


    }
}

