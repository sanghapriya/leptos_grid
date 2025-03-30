use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FieldValue {
    pub value: RwSignal<String>, // Internal reactivity
    field_type: String, // Metadata (e.g., "text", "number", "select")
}

impl FieldValue {
    
    pub fn new(value: String, field_type: &str) -> Self {
        Self {
            value: RwSignal::new(value),
            field_type: field_type.to_string(),
        }
    }

  
}
