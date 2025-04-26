use crate::data_type::field_value::FieldValue;
use std::collections::HashMap;
use reactive_stores::{Field, Patch, Store};
use leptos::prelude::*;


#[derive(Debug, Clone, Store)]
pub struct EntryMultiple {
    pub key: String,
    // pub headers: RwString<Vec<String>>,
    pub fields: HashMap<String, FieldValue>, 
}

impl EntryMultiple{
   
    pub fn new(key: String, fields: HashMap<String, FieldValue>) -> Self {
        Self { key, fields }
    }
}