use bytebuffer::ByteBuffer;

use string_builder::Builder;
use crate::col_info::ColInfo;
use crate::record::Record;


pub struct Relation { //PERSONNE(NOM,PRENOM?,AGE)
    name:String,
    columns: Vec<ColInfo>,
    nb_columns: usize,
}

impl Relation {

    fn new (name : String, nb_columns:usize) -> Self{

        Relation {
            name: String::from(name),
            columns: Vec::new(),
            nb_columns,

        }

    }

    fn get_name(&self)->&String {
        &self.name
    }

    // Getter pour les informations sur les colonnes
    fn get_columns(&self) -> Vec<ColInfo> {
        self.columns.clone()
    }
 
    }

    pub fn write_record_to_buffer(&mut self, record:Record, buffer:&mut Vec<u8>, pos:usize)->usize{
        // Copie du tuple (pas obligatoire)
        let tuple = record.get_tuple().clone();
        //Pour avoir le nom des colonnes, le type etc...
        let columns_local = self.columns.clone();
        let mut compteur:usize=0;
        
        let mut index = pos; //pour la position

        // Initialisation de la taille d'un BUFFER
        

        // Pour savoir si une colonne type  varchar a été trouvé dans le record
        let mut varchar_trouve:bool=false;

        

        // Pour stocker les longueurs des VARCHAR dans une liste
        let mut liste_len_varchars:Vec<usize> = Vec::new();

        // Recherche d'un ou plusieurs VARCHAR dans le tuple.
        for i in 0..columns_local.len(){
            if columns_local[i].get_name().starts_with("VARCHAR"){
                varchar_trouve=true;
                break;
            }      
        }

        let mut index_objets:Vec<usize> =Vec::new();
        
        
        if varchar_trouve{
            // Stockage des longueurs en octet de chaque attribut
            //pour faire les offset après
            for i in 0..tuple.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        index_objets.push(4);
                        compteur+=4;
                        break;
                    }
                    "REAL" => {
                        index_objets.push(4);
                        compteur+=4;
                        break;
                    }
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let nbytes=" ".repeat(substring.parse::<usize>().unwrap()).as_bytes().len();
                        index_objets.push(nbytes);
                        compteur+=nbytes; // COMPTEUR + LA TAILLE DE LA CHAINE
                        break;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let index:Option<usize> = s2.find(')') ;
                        let substring: &str = &tuple[i][8..index.unwrap()];
                    
                        let len_s=substring.parse::<usize>().unwrap();
                        let nbytes=if len_s>=tuple[i].len(){
                            tuple[i].as_bytes().len()
                        }else{
                            // A REVOIR
                            " ".repeat(len_s).as_bytes().len()
                        };
                        index_objets.push(nbytes);
                        compteur+=nbytes;
                        break;
                    }
                    _ => {} //default du match
                    
                    
                }
            }
            //Ecriture des longueurs des attributs
            //ça met les offset dans le buffer
            for i in 0..index_objets.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        buffer.extend_from_slice(&(compteur as u32).to_be_bytes());
                        compteur+=4;
                        break;
                    }
                    "REAL" => {
                        buffer.extend_from_slice(&(compteur as u32).to_be_bytes());
                        compteur+=4;
                        break;
                    }
                    s if s.starts_with("CHAR")  => {
                        buffer.extend_from_slice(&(compteur as u32).to_be_bytes());
                        let taille=index_objets[i];
                        compteur+=taille;
                        break;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        buffer.extend_from_slice(&(compteur as u32).to_be_bytes());
                        let taille=index_objets[i];
                        compteur+=taille;
                        break;
                    }
                    _ => {} //default du match
                }
            }
            
            // Ecriture des valeurs des attributs
            for i in 0..index_objets.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        //on transforme la valeur dans le tuple en octets
                        let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        buffer[index..index + byte.len()].copy_from_slice(&Vec::from(&bytes));
                        compteur+=4;// Pour les 4 octets de l'entier
                        index += 4;
                        break;
                    }
                    "REAL" => {
                        let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        buffer[index..index + byte.len()].copy_from_slice(&Vec::from(&bytes));
                        compteur+=4;// Pour les 4 octets du reel
                        index += 4;
                        break;
                    }
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                    
                        let mut bytes = tuple[i].to_be_bytes();
                        buffer[index..index + byte.len()].copy_from_slice(&Vec::from(&bytes));
                        compteur+= taille_s;
                        index += taille_s;
                        break;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let index:Option<usize> = s2.find(')') ;
                        let substring: &str = &tuple[i][8..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        
                        
                        let bytes=tuple[i].as_bytes();
                        let nbytes=if taille_s<tuple[i].len(){
                            tuple[i][0..taille_s].as_bytes().len()
                        }else{
                            // A REVOIR CAR "🚀" par exemple prend plus de place que " "
                            " ".repeat(taille_s).as_bytes().len()
                        };
                        let mut bytes = tuple[i].to_be_bytes();
                        buffer[index..index + byte.len()].copy_from_slice(&Vec::from(&bytes));
                        compteur += nbytes;
                        index += nbytes;
                        break;
                    }
                    _ => {}
                }
            }

        }else{// Si pas de VARCHAR dans le tuple 
            
            // FACILE à comprendre, le code est transparent :)
            for i in 0..self.nb_columns{
                let tmp=tuple[i].clone();
                
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                         //on transforme la valeur dans le tuple en octets
                        let mut bytes = tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        //on rentre cette valeur dans le buffer à la bonne position
                        buffer[index..index + byte.len()].copy_from_slice(&Vec::from(&bytes));
                        compteur+=4;// Pour les 4 octets de l'entier
                        index += 4;
                        break;
                        
                    }
                    "REAL" => {
                        let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        buffer[index..index + byte.len()].copy_from_slice(&Vec::from(&bytes));
                        compteur+=4;// Pour les 4 octets du reel
                        index += 4;
                        break;
                    } // CHAR(20) --> 20 CARACTERES = 20 OCTETS
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                    
                        let mut bytes = tuple[i].to_be_bytes();
                        buffer[index..index + byte.len()].copy_from_slice(&Vec::from(&bytes));
                        compteur+= taille_s;
                        index += taille_s;
                        
                        break;
                        
                    }
                    
                    _ => {} //default du match
                }

            }
        }
        return compteur;
        
        
    }

    

    




}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_apagnan() {
        println!("{}",(4 as u32).to_be_bytes().len().to_string());
    }
}

