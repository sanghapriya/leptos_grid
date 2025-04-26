use leptos::prelude::*;
use reactive_stores::{Field, Patch, Store};
use crate::data_type::header_config::HeaderConfig;
use crate::data_type::header_config::HeaderConfigStoreFields;

use crate::data_type::grid_rows::GridRows;



#[component]
pub fn GridHeaderCellElement(
    
    #[prop(into)] header: Field<HeaderConfig>) -> impl IntoView {

        let label = header.label();


    view! {
        
        <input
                // style="font-weight: bold;"
                type="text"
                prop:value=move || label.get()
                on:change=move |ev| {
                    label.set(event_target_value(&ev));
                }
        /> 
        
      
    }
}
