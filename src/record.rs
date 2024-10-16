pub struct record<'a> {
    //le tuple de la ligne, représenté par un vecteur de chaines de caractères
    record_tuple: Vec<String>,
}

impl<'a> record<'a>{
    //constructeur
    pub fn new(record_tuple: Vec<String>)->Self{
        Self{
            record_tuple,
        }
    }
    
    //get, indispensable si on veut récupérer le tuple dans un module externe
    pub fn get_tuple(&self) -> Vec<Sting>{
        return self.record_tuple.clone();
    }
}
