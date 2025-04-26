use reactive_stores::{Field, Patch, Store};

use std::sync::atomic::{AtomicUsize, Ordering};

use super::field_value::FieldValue;

// ID starts higher than 0 because we have a few starting todos by default
static NEXT_ID: AtomicUsize = AtomicUsize::new(3);

#[derive(Debug, Clone, Store)]
pub struct Row {
    pub id: usize,
    pub rows: Vec<FieldValue>
}

impl Row {
    pub fn new(rows: Vec<FieldValue>) -> Self {
        Self { id: NEXT_ID.fetch_add(1, Ordering::Relaxed), rows }
    }
}