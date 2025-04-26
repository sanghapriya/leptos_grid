use leptos::prelude::*;

use reactive_stores::{Field, Patch, Store};
use std::sync::atomic::{AtomicUsize, Ordering};

// ID starts higher than 0 because we have a few starting todos by default
static NEXT_ID: AtomicUsize = AtomicUsize::new(3);
use super::grid_field::GridField;

#[derive(Debug,Default, Clone,Store)]
pub struct GridRow {
    pub id: usize,
    #[store(key: usize = |field| field.id)]
    pub fields: Vec<GridField>, // Internal reactivity
    
}

impl GridRow {
    pub fn new(fields: Vec<GridField>) -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), fields }
    }
  
}
