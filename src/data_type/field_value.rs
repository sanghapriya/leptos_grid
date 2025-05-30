use leptos::prelude::*;

use reactive_stores::{Field, Patch, Store};
use std::sync::atomic::{AtomicUsize, Ordering};

// ID starts higher than 0 because we have a few starting todos by default
static NEXT_ID: AtomicUsize = AtomicUsize::new(3);

#[derive(Debug, Clone,Store)]
pub struct FieldValue {
    pub id: usize,
    pub value: String, // Internal reactivity
    
}

impl FieldValue {
    pub fn new(value: String) -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), value }
    }

  
}
