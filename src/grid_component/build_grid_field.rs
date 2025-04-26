use leptos::{logging::log, prelude::*};


use reactive_stores::{Field, Patch, Store};
use crate::data_type::grid_field::GridField;
use crate::data_type::grid_field::GridFieldStoreFields;

#[component]
pub fn BuildGridField(
    
    #[prop(into)] grid_field: Field<GridField>,

) -> impl IntoView {


    
    let value = grid_field.value();


    view! {
        <input
                type="text"
                prop:value=move || value.get()
                on:change=move |ev| {
                    value.set(event_target_value(&ev));
                    
                }
        />
    }


}