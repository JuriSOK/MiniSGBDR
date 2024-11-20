use bytebuffer::ByteBuffer;

use string_builder::Builder;
use crate::buffer::{self, Buffer};
use crate::col_info::ColInfo;
use crate::page::{self, PageId};
use crate::record::Record;
use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::cell::{Ref, RefCell};
use crate::buffer_manager::BufferManager;
use crate::record_id::RecordId;
use crate::DBConfig;
use crate::disk_manager::DiskManager;
use std::env;
use std::collections::HashMap;
use crate::relation::Relation;

pub struct Database<'a> {
    nom : String,
    relations : Vec<Relation<'a>>,

}
impl<'a> Database<'a> {
    pub fn new(name : String) -> Self {
        Database{
            nom : String::from(name),
            relations : Vec::new()
        }
    }
    pub fn set_relations(&mut self, relations : Vec<Relation<'a>>) {
        self.relations = relations;
    }
    pub fn get_relations(&self) -> &Vec<Relation<'a>> {
        return &self.relations;
    }
    pub fn get_nom(&self) -> &str {
        return &self.nom;
    }
    pub fn add_relation(&mut self, relation : Relation<'a>) {
        self.relations.push(relation);
    }
    pub fn remove_relation(&mut self, relation : &str) {
        if let Some(index) = self.relations.iter().position(|r| r.get_name() == relation){
            self.relations.swap_remove(index); //complexité en O(1), met le dernier element à la place de l'element qu'on veut supprimer
        }
    }
}
