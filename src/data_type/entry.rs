

use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: String,
    pub value: RwSignal<String>,
    pub select: RwSignal<String>,
}

impl Entry {
    pub fn new(key: String, value: RwSignal<String>, select: RwSignal<String>) -> Self {
        Self { key, value, select }
    }
}