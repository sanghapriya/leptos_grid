
use reactive_stores::{Field, Patch, Store};
use std::sync::atomic::{AtomicUsize, Ordering};

// ID starts higher than 0 because we have a few starting todos by default
static NEXT_ID: AtomicUsize = AtomicUsize::new(3);

#[derive(Debug, Clone, Store, Default)]

pub struct HeaderConfig {

    pub id: usize,
    pub label: String,
    pub  show_menu: bool
}
impl HeaderConfig {
    pub fn new(label: String, show_menu: bool) -> Self {
        Self { 
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            
            label, 
            show_menu }
    }
}


