use leptos::prelude::*;




#[component]
pub fn GridCellElement(element: RwSignal<String>) -> impl IntoView {


    view! {
        <input 
        class="grid-input"
        type="text" 
        value=element
        on:input=move |event| {
            
            element.set(event_target_value(&event));
        }
        /> 
    }
}