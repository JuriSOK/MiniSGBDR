use crate::record::Record;
use std::any::Any;
use crate::col_info::ColInfo;

pub enum ConditionType {
    ColumnColumn(String, String, String), // Colonne1, opérateur, Colonne2
    ColumnConstant(String, String, String), // Colonne, opérateur, Valeur
   
}
pub struct Condition {
    condition_type: ConditionType,
    // Type de la colonne pour la comparaison
}
impl Condition {

    // Crée une condition de type colonne-colonne
    pub fn new_column_column(col1: String, operator: String, col2: String) -> Self {
        Condition {
            condition_type: ConditionType::ColumnColumn(col1, operator, col2),
           
        }
    }
    // Crée une condition de type colonne-valeur
    pub fn new_column_constant(col: String, operator: String, value: String) -> Self {
        Condition {
            condition_type: ConditionType::ColumnConstant(col, operator, value),
        }
    }



    // Fonction pour évaluer la condition sur un enregistrement (Record)
    pub fn evaluate(&self, record: &Record, col_info: Vec<ColInfo>) -> bool {
        match &self.condition_type {
            ConditionType::ColumnColumn(col1, operator, col2) => {
                // Comparaison entre 2 colonnes
                let value1 = self.get_value_for_column(col1, record, col_info.clone());
                let value2 = self.get_value_for_column(col2, record, col_info.clone());
                self.compare_values(value1, value2, operator)
            }
            ConditionType::ColumnConstant(col, operator, value) => {
                // Comparaison entre une colonne et une constante
                let record_value = self.get_value_for_column(col, record, col_info);
                // Convertir la constante en fonction du type de la colonne
                let converted_value = self.convert_constant(value.clone(), &record_value);
                self.compare_values(record_value, converted_value, operator)
            }
        }
    }

    fn convert_constant(&self, constant: String, record_value: &Box<dyn Any>) -> Box<dyn Any> {
        // Convertir la constante en fonction du type de la valeur de la colonne
        if let Some(_) = record_value.downcast_ref::<i32>() {
            // Si la colonne est de type i32, on convertit la constante en i32
            Box::new(constant.parse::<i32>().unwrap_or(0))
        } else if let Some(_) = record_value.downcast_ref::<f64>() {
            // Si la colonne est de type f64, on convertit la constante en f64
            Box::new(constant.parse::<f64>().unwrap_or(0.0))
        } else if let Some(_) = record_value.downcast_ref::<String>() {
            // Si la colonne est de type String, on conserve la constante en String
            Box::new(constant)
        } else {
            // Si le type de colonne est inconnu, on retourne la constante sous forme de String par défaut
            Box::new(constant)
        }
    }
    


    fn compare_values(&self, value1: Box<dyn Any>, value2: Box<dyn Any>, operator: &str) -> bool {
        if let (Some(v1), Some(v2)) = (value1.downcast_ref::<String>(), value2.downcast_ref::<String>()) {
            //println!("Comparaison Strings: {} {} {}", v1, operator, v2);
            self.compare_strings(v1, v2, operator)
        } else if let (Some(v1), Some(v2)) = (value1.downcast_ref::<i32>(), value2.downcast_ref::<i32>()) {
            //println!("Comparaison entier: {} {} {}", v1, operator, v2);
            self.compare_ints(v1, v2, operator)
        } else if let (Some(v1), Some(v2)) = (value1.downcast_ref::<f64>(), value2.downcast_ref::<f64>()) {
            //println!("Comparaison float: {} {} {}", v1, operator, v2);
            self.compare_floats(v1, v2, operator)
        } else {
            false  // Si les types ne correspondent pas
        }
    }

    


    fn compare_strings(&self, value1: &str, value2: &str, operator: &str) -> bool {
        match operator {
            "=" => value1 == value2,
            "<" => value1 < value2,
            ">" => value1 > value2,
            "<>" => value1 != value2,  // Comparaison "différent"
            "<=" => value1 <= value2,  // Comparaison "inférieur ou égal"
            ">=" => value1 >= value2,  // Comparaison "supérieur ou égal"
            _ => false,
        }
    }


    fn compare_ints(&self, value1: &i32, value2: &i32, operator: &str) -> bool {
        match operator {
            "=" => value1 == value2,
            "<" => value1 < value2,
            ">" => value1 > value2,
            "<>" => value1 != value2,  // Comparaison "différent"
            "<=" => value1 <= value2,  // Comparaison "inférieur ou égal"
            ">=" => value1 >= value2,  // Comparaison "supérieur ou égal"
            _ => false,
        }
    }

    fn compare_floats(&self, value1: &f64, value2: &f64, operator: &str) -> bool {
        match operator {
            "=" => value1 == value2,
            "<" => value1 < value2,
            ">" => value1 > value2,
            "<>" => value1 != value2,  // Comparaison "différent"
            "<=" => value1 <= value2,  // Comparaison "inférieur ou égal"
            ">=" => value1 >= value2,  // Comparaison "supérieur ou égal"
            _ => false,
        }
    }


    fn get_value_for_column(&self, col_name: &str, record: &Record, col_info: Vec<ColInfo>) -> Box<dyn Any> {
        let column_info = col_info.iter().find(|col| col.get_name() == col_name).unwrap(); // Récupérer les infos de la colonne
        let index = col_info.iter().position(|col| col.get_name() == col_name).unwrap(); // Index de la colonne dans l'enregistrement
    
        let record_value = record.get_value(index);  // Valeur brute du record à l'index donné
    
        match column_info.get_column_type().as_str() {
            "INT" => {
                // Convertir en entier et retourner un Box contenant un i32
                Box::new(record_value.parse::<i32>().unwrap_or(0))
            }
            "REAL" => {
                // Convertir en réel (f64) et retourner un Box contenant un f64
                Box::new(record_value.parse::<f64>().unwrap_or(0.0))
            }
            t if t.starts_with("CHAR") => {
                // Pour les types CHAR, on retourne la valeur brute sous forme de String
                Box::new(record_value.clone())
            }
            t if t.starts_with("VARCHAR") => {
                // Pour les types VARCHAR, on retourne la valeur brute sous forme de String
                Box::new(record_value.clone())
            }
            _ => {
                // Pour d'autres types éventuels, on retourne la valeur brute sous forme de String
                Box::new(record_value.clone())
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_column_condition_equal() {
        let record = Record::new(vec!["10".to_string(), "20".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "INT".to_string()),
            ColInfo::new("col2".to_string(), "INT".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), "=".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, false, "The values in col1 and col2 should not be equal.");
    }

    #[test]
    fn test_column_constant_condition_equal() {
        let record = Record::new(vec!["10".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "INT".to_string()),
        ];

        let condition = Condition::new_column_constant("col1".to_string(), "=".to_string(), "10".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be equal to 10.");
    }

    #[test]
    fn test_column_column_condition_greater_than() {
        let record = Record::new(vec!["15".to_string(), "10".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "INT".to_string()),
            ColInfo::new("col2".to_string(), "INT".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), ">".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "col1 should be greater than col2.");
    }

    #[test]
    fn test_column_constant_condition_less_than() {
        let record = Record::new(vec!["5".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "INT".to_string()),
        ];

        let condition = Condition::new_column_constant("col1".to_string(), "<".to_string(), "10".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "col1 should be less than 10.");
    }

    #[test]
    fn test_column_constant_condition_not_equal() {
        let record = Record::new(vec!["5".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "INT".to_string()),
        ];

        let condition = Condition::new_column_constant("col1".to_string(), "<>".to_string(), "10".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "col1 should not be equal to 10.");
    }

    #[test]
    fn test_column_column_condition_not_equal() {
        let record = Record::new(vec!["5".to_string(), "10".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "INT".to_string()),
            ColInfo::new("col2".to_string(), "INT".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), "<>".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "col1 should not be equal to col2.");
    }

    #[test]
    fn test_column_column_condition_equal_strings() {
        let record = Record::new(vec!["apple".to_string(), "apple".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
            ColInfo::new("col2".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), "=".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The values in col1 and col2 should be equal.");
    }

    #[test]
    fn test_column_column_condition_not_equal_strings() {
        let record = Record::new(vec!["apple".to_string(), "banana".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
            ColInfo::new("col2".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), "<>".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The values in col1 and col2 should not be equal.");
    }

    #[test]
    fn test_column_constant_condition_equal_string() {
        let record = Record::new(vec!["apple".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_constant("col1".to_string(), "=".to_string(), "apple".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be equal to 'apple'.");
    }

    #[test]
    fn test_column_constant_condition_less_than_string() {
        let record = Record::new(vec!["apple".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_constant("col1".to_string(), "<".to_string(), "banana".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be less than 'banana'.");
    }

    #[test]
    fn test_column_constant_condition_greater_than_string() {
        let record = Record::new(vec!["banana".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_constant("col1".to_string(), ">".to_string(), "apple".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be greater than 'apple'.");
    }

    #[test]
    fn test_column_constant_condition_not_equal_string() {
        let record = Record::new(vec!["apple".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_constant("col1".to_string(), "<>".to_string(), "banana".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should not be equal to 'banana'.");
    }

    #[test]
    fn test_column_column_condition_less_than_string() {
        let record = Record::new(vec!["apple".to_string(), "banana".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
            ColInfo::new("col2".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), "<".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be less than the value in col2.");
    }

    #[test]
    fn test_column_column_condition_greater_than_string() {
        let record = Record::new(vec!["banana".to_string(), "apple".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
            ColInfo::new("col2".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), ">".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be greater than the value in col2.");
    }

    #[test]
    fn test_column_column_condition_less_than_or_equal_string() {
        let record = Record::new(vec!["apple".to_string(), "banana".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
            ColInfo::new("col2".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), "<=".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be less than or equal to the value in col2.");
    }

    #[test]
    fn test_column_column_condition_greater_than_or_equal_string() {
        let record = Record::new(vec!["banana".to_string(), "apple".to_string()]);
        let col_info = vec![
            ColInfo::new("col1".to_string(), "VARCHAR".to_string()),
            ColInfo::new("col2".to_string(), "VARCHAR".to_string()),
        ];

        let condition = Condition::new_column_column("col1".to_string(), ">=".to_string(), "col2".to_string());

        let result = condition.evaluate(&record, col_info);
        assert_eq!(result, true, "The value in col1 should be greater than or equal to the value in col2.");
    }
}