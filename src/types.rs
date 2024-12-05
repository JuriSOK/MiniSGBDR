use std::any::Any;
pub struct Number{
    pub valeur: f64,
}

pub struct Chars {
    pub valeur: String,
}


pub trait Operande {
    fn equals(&self, other: &dyn Operande) -> bool;
    fn get_type(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}



impl Operande for Number {
    fn equals(&self, other: &dyn Operande) -> bool {
        if other.get_type() == "NUMBER" {
            // Downcast to a Number to compare
            if let Some(other_number) = other.as_any().downcast_ref::<Number>() {
                return self.valeur == other_number.valeur;
            }
        }
        false
    }

    fn get_type(&self) -> String {
        String::from("NUMBER")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Operande for Chars {
    fn equals(&self, other: &dyn Operande) -> bool {
        if other.get_type() == "CHARS" {
            // Downcast to a Chars to compare
            if let Some(other_chars) = other.as_any().downcast_ref::<Chars>() {
                return self.valeur == other_chars.valeur;
            }
        }
        false
    }


    fn get_type(&self) -> String {
        String::from("CHARS")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}