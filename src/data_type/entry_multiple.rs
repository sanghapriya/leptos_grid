use crate::data_type::field_value::FieldValue;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct EntryMultiple {
    pub key: String,
    pub fields: HashMap<String, FieldValue>, 
}

impl EntryMultiple{
   
    pub fn new(key: String, fields: HashMap<String, FieldValue>) -> Self {
        Self { key, fields }
    }
}