use crate::col_info::ColInfo;


pub struct Relation {

    name:String,
    columns: Vec<ColInfo>,
    nb_columns: usize,


}
impl Relation {

    fn new (name : &str, nb_columns:usize) -> Self{

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
    fn get_columns(&self) -> &Vec<ColInfo> {
        &self.columns
    }



}