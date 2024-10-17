use bytebuffer::ByteBuffer;

use string_builder::Builder;
use crate::col_info::ColInfo;
use crate::record::Record;


pub struct Relation {
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

    pub fn write_record_to_buffer(&mut self, record:Record, liste_buffer:&mut Vec<Vec<u8>>, pos:usize)->usize{
        // Copie du tuple (pas obligatoire)
        let tuple = record.get_tuple().clone();
        let mut compteur:usize=0;

        // Initialisation de la taille d'un BUFFER
        

        // Pour savoir si un varchar a été trouvé dans le record
        let mut varchar_trouve:bool=false;

        

        // Pour stocker les longueurs des VARCHAR dans une liste
        let mut liste_len_varchars:Vec<usize> = Vec::new();
        // Recherche d'un ou plusieurs VARCHAR dans le tuple.
        for i in 0..tuple.len(){
            if tuple[i].starts_with("VARCHAR"){
                varchar_trouve=true;
                break;
            }      
        }

        let mut index_objets:Vec<usize> =Vec::new();
        
        
        if varchar_trouve{
            // Stockage des longueurs en octet de chaque attribut
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
                        compteur+=4;
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
                        compteur+=4;
                        break;
                    }
                    _ => {}
                }
            }
            // Ecriture des longueurs des attributs
            for i in 0..index_objets.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        liste_buffer[pos].extend_from_slice(&(compteur as u32).to_be_bytes());
                        compteur+=4;
                        break;
                    }
                    "REAL" => {
                        liste_buffer[pos].extend_from_slice(&(compteur as u32).to_be_bytes());
                        compteur+=4;
                        break;
                    }
                    s if s.starts_with("CHAR")  => {
                        liste_buffer[pos].extend_from_slice(&(compteur as u32).to_be_bytes());
                        let taille=index_objets[i];
                        compteur+=taille;
                        break;
                    }
                    s2 if s2.starts_with("VARCHAR") => {
                        liste_buffer[pos].extend_from_slice(&(compteur as u32).to_be_bytes());
                        let taille=index_objets[i];
                        compteur+=taille;
                        break;
                    }
                    _ => {}
                }
            }
            // Ecriture des valeurs des attributs
            for i in 0..index_objets.len(){
                match self.columns[i].get_column_type().as_str().clone() {
                    "INT" => {
                        liste_buffer[pos].extend_from_slice(&(4 as i32).to_be_bytes());
                        let mut bytes= tuple[i].parse::<i32>().unwrap().to_be_bytes();
                        for i in 0..bytes.len(){
                            liste_buffer[pos][compteur]=bytes[i];
                            compteur+=1;
                        }
                        //compteur+=4;// Pour les 4 octets de l'entier
                        break;
                    }
                    "REAL" => {
                        liste_buffer[pos].extend_from_slice(&(4 as f32).to_be_bytes());
                        let mut bytes= tuple[i].parse::<f32>().unwrap().to_be_bytes();
                        for i in 0..bytes.len(){
                            liste_buffer[pos][compteur]=bytes[i];
                            compteur+=1;
                        }
                        //compteur+=4;// Pour les 4 octets de l'entier
                        break;
                    }
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        
                        // Remplissage des caractères du string
                        let mut sb: Builder=Builder::default();
                        sb.append(tuple[i].clone());
                        for j in tuple[i].len()..taille_s{
                            //A REVOIR
                            sb.append(" ");// Caractères pour remplir
                        }
                        let s=sb.string().unwrap();

                        // Ecriture
                        let bytes=s.as_bytes();
                        liste_buffer[pos].extend_from_slice(&(bytes.len() as f32).to_be_bytes());
                        for j in 0..bytes.len(){
                            liste_buffer[pos][compteur]=bytes[j];
                            compteur+=1;
                        }
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
                        for j in 0..nbytes{
                            if(j>=taille_s){
                                break;
                            }
                            liste_buffer[pos][compteur]=bytes[j];
                            compteur+=1;
                        }
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
                        let bytes_result: Result<i32, std::num::ParseIntError>=tmp.parse::<i32>();
                        let bytes =bytes_result.unwrap().to_be_bytes();
                        for i in 0..4 {
                            liste_buffer[pos][i]=bytes[i];
                            compteur+=1;
                        }
                    }
                    "REAL" => {
                        let bytes_result: Result<f32, std::num::ParseFloatError> =tmp.parse::<f32>();
                        let bytes: [u8; 4]=bytes_result.unwrap().to_be_bytes();
                        for i in 0..4 {
                            liste_buffer[pos][i]=bytes[i];
                            compteur+=1;
                        }
                    }
                    s if s.starts_with("CHAR")  => {
                        let index:Option<usize> = s.find(')') ;
                        let substring: &str = &tuple[i][5..index.unwrap()];
                        let taille_s=substring.parse::<usize>().unwrap();
                        
                        // Remplissage des caractères du string
                        let mut sb: Builder=Builder::default();
                        sb.append(tuple[i].clone());
                        for j in tuple[i].len()..taille_s{
                            sb.append(" ");// Caractères pour remplir
                        }
                        let s=sb.string().unwrap();

                        // Ecriture
                        let bytes=s.as_bytes();
                        for j in 0..bytes.len(){
                            liste_buffer[pos][compteur]=bytes[j];
                            compteur+=1;
                        }
                        break;
                        
                    }
                    
                    _ => {}
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