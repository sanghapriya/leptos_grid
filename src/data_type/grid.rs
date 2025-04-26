use reactive_stores::{Field, Patch, Store};
use super::rows::Row; 
use crate::data_type::header_config::HeaderConfig;



#[derive(Debug, Clone, Store, Default)]
pub struct Grid {
    #[store(key: usize = |header| header.id)]
    pub headers: Vec<HeaderConfig>,
    #[store(key: usize = |row| row.id)]
    pub rows: Vec<Row>,
}

impl Grid {
    pub fn new(headers: Vec<String>, rows: Vec<Row>) -> Self {
        let headers = headers.into_iter().map(|header| HeaderConfig::new(header, false)).collect();
        Self { headers, rows }
    }
}