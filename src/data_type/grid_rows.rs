use leptos::prelude::*;

use reactive_stores::{Field, Patch, Store};
use std::sync::atomic::{AtomicUsize, Ordering};
use super::grid_row::GridRow;

// ID starts higher than 0 because we have a few starting todos by default
static NEXT_ID: AtomicUsize = AtomicUsize::new(3);

#[derive(Debug, Default,Clone,Store)]
pub struct GridRows {
    pub id: usize,
    #[store(key: usize = |row| row.id)]
    pub rows: Vec<GridRow>, // Internal reactivity
    
}

impl GridRows {
    pub fn new(rows: Vec<GridRow>) -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), rows }
    }
  
}
