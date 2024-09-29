use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct PageId{
    FileIdx :  u32, 
    PageIdx : u32, 
}

impl PageId {
    
    pub fn new( fidx : u32, pidx : u32 ) -> Self{ //Constructeur de la classe
        Self{ //dans ce scope on met les attributs de la classe
            FileIdx : fidx ,
            PageIdx : pidx ,
        }
    }
    
    pub fn get_FileIdx(&self) -> u32{
        self.FileIdx
    }
    pub fn get_PageIdx(&self) -> u32{
        self.PageIdx
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_constructeur() {
        
        let f_test: u32 = 1 ;
        let p_test : u32 = 3;

        let classe = PageId :: new(f_test, p_test);
        assert_eq!(classe.FileIdx, 1 );
        assert_eq!(classe.PageIdx, 3);
        
        
        
    }
}

