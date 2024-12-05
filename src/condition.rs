use crate::types::{Operande, Number, Chars};
use crate::record::Record;


enum Operateur {
    EQUAL,
    LESSTHAN,
    GREATERTHAN,
    LESSEQUAL,
    GREATEREQUAL,
    NOTEQUAL,
}

pub struct Condition {
    oper_gauche: Box<dyn Operande>,
    operateur: Operateur,
    oper_droite: Box<dyn Operande>,
}

impl Condition {
    pub fn new(gauche: Box<dyn Operande>, operateur: Operateur, droite: Box<dyn Operande>) -> Self {
        Condition {
            oper_gauche: gauche,
            operateur,
            oper_droite: droite,
        }
    }


    pub fn evaluate(&self, record: &Record) -> bool {
        // Récupérer les opérandes gauche et droite et leur type
        let gauche_type = self.oper_gauche.get_type();
        let droite_type = self.oper_droite.get_type();

        // Comparer les types des opérandes avant de comparer les valeurs
        if gauche_type != droite_type {
            return false; // Les types ne correspondent pas
        }

        match self.operateur {
            Operateur::EQUAL => {
                self.oper_gauche.equals(&*self.oper_droite)
            }
            Operateur::LESSTHAN => {
                self.compare_less_than(&*self.oper_gauche, &*self.oper_droite)
            }
            Operateur::GREATERTHAN => {
                self.compare_greater_than(&*self.oper_gauche, &*self.oper_droite)
            }
            Operateur::LESSEQUAL => {
                self.compare_less_than_or_equal(&*self.oper_gauche, &*self.oper_droite)
            }
            Operateur::GREATEREQUAL => {
                self.compare_greater_than_or_equal(&*self.oper_gauche, &*self.oper_droite)
            }
            Operateur::NOTEQUAL => {
                !self.oper_gauche.equals(&*self.oper_droite)
            }
        }
    }

    fn compare_less_than(&self, gauche: &dyn Operande, droite: &dyn Operande) -> bool {
        if gauche.get_type() == "NUMBER" {
            if let (Some(left), Some(right)) = (
                gauche.as_any().downcast_ref::<Number>(),
                droite.as_any().downcast_ref::<Number>(),
            ) {
                return left.valeur < right.valeur;
            }
        }
        false
    }

    fn compare_greater_than(&self, gauche: &dyn Operande, droite: &dyn Operande) -> bool {
        if gauche.get_type() == "NUMBER" {
            if let (Some(left), Some(right)) = (
                gauche.as_any().downcast_ref::<Number>(),
                droite.as_any().downcast_ref::<Number>(),
            ) {
                return left.valeur > right.valeur;
            }
        }
        false
    }

    fn compare_less_than_or_equal(&self, gauche: &dyn Operande, droite: &dyn Operande) -> bool {
        if gauche.get_type() == "NUMBER" {
            if let (Some(left), Some(right)) = (
                gauche.as_any().downcast_ref::<Number>(),
                droite.as_any().downcast_ref::<Number>(),
            ) {
                return left.valeur <= right.valeur;
            }
        }
        false
    }

    fn compare_greater_than_or_equal(&self, gauche: &dyn Operande, droite: &dyn Operande) -> bool {
        if gauche.get_type() == "NUMBER" {
            if let (Some(left), Some(right)) = (
                gauche.as_any().downcast_ref::<Number>(),
                droite.as_any().downcast_ref::<Number>(),
            ) {
                return left.valeur >= right.valeur;
            }
        }
        false
    }
}