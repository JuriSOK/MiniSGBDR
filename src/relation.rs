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

    pub fn write_record_to_buffer(&mut self, record:Record, buffer:&mut Vec<u8>, pos:usize)->usize{
        // Copie du tuple (pas obligatoire)
        let tuple = record.get_tuple().clone();
        //Pour avoir le nom des colonnes, le type etc...
        let columns_local = self.columns.clone();
        let mut compteur:usize=0;
        
        let mut indice:usize = pos; //pour la position

        // Initialisation de la taille d'un BUFFER
        

        // Pour savoir si une colonne type  varchar a été trouvé dans le record
        let mut varchar_trouve:bool=false;

        // Recherche d'un ou plusieurs VARCHAR dans le tuple.
        for i in 0..columns_local.len(){
            if columns_local[i].get_name().starts_with("VARCHAR"){
                varchar_trouve=true;
                break;
            }      
        }

        let mut taille_objets:Vec<usize> =Vec::new();
        
        
        if varchar_trouve{
            // Stockage des longueurs en octet de chaque attribut
            //pour faire les offset après
            for i in 0..tuple.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        taille_objets.push(4);
                        compteur+=4;
                        break;
                    }
                    "REAL" => {
                        taille_objets.push(4);
                        compteur+=4;
                        break;
                    }
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let nbytes=" ".repeat(substring.parse::<usize>().unwrap()).as_bytes().len();
                        taille_objets.push(nbytes);
                        compteur+=4; // COMPTEUR + LA TAILLE DE LA CHAINE
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
                        taille_objets.push(nbytes);
                        compteur+=4;
                        break;
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
                        break;
                    }
                    "REAL" => {
                        let bytes=(compteur2 as u32).to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur2+=4;
                        indice+=4;
                        break;
                    }
                    s if s.starts_with("CHAR")  => {
                        let taille=taille_objets[i];
                        let bytes=(taille as u32).to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur2+=taille;
                        indice+=taille;
                        break;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        let taille=taille_objets[i];
                        let bytes=(taille as u32).to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur2+=taille;
                        indice+=taille;
                        break;
                    }
                    _ => {} //default du match
                }
            }
            buffer.extend_from_slice(&(compteur2 as u32).to_be_bytes());
            
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
                        break;
                    }
                    "REAL" => {
                        let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur+=4;// Pour les 4 octets du reel
                        indice += 4;
                        break;
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
                        break;
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
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur+=4;// Pour les 4 octets de l'entier
                        indice += 4;
                        break;
                        
                    }
                    "REAL" => {
                        let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur += 4;// Pour les 4 octets du reel
                        indice += 4;
                        break;
                    } // CHAR(20) --> 20 CARACTERES = 20 OCTETS
                    s if s.starts_with("CHAR")  => {
                        /*
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        */
                    
                        let mut bytes = tuple[i].as_bytes();
                        buffer[indice..indice + bytes.len()].copy_from_slice(&bytes);
                        compteur += bytes.len();
                        indice += bytes.len();
                        
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

