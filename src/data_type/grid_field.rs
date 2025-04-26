use leptos::prelude::*;

use reactive_stores::{Field, Patch, Store};
use std::sync::atomic::{AtomicUsize, Ordering};

// ID starts higher than 0 because we have a few starting todos by default
static NEXT_ID: AtomicUsize = AtomicUsize::new(3);

#[derive(Debug, Default, Clone,Store)]
pub struct GridField {
    pub id: usize,
    pub value: String, // Internal reactivity
    pub header_id: usize,
    
}

impl GridField {
    pub fn new(value: String, header_id: usize) -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), value , header_id}
    }

  
}
