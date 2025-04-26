use leptos::prelude::*;
use crate::data_type::header_config::HeaderConfig;



#[component]
pub fn GridCellElement(element: String) -> impl IntoView {


    view! {
        <input 
        class="grid-input"
        type="text" 
        value=element
        
        /> 
    }
}