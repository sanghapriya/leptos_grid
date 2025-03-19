use leptos::prelude::*;


mod component;
mod data_type;

use crate::component::grid_layout::GridLayout;
use crate::data_type::entry::Entry;


fn InputBox() -> impl IntoView {

    let (text, set_text) = signal("".to_string());

    let setter = use_context::<WriteSignal<Vec<Entry>>>().unwrap();


    view! {
         <input
            type="text"
            bind:value=(text,set_text)
            on:input=move |event| {
            
            set_text.set(event_target_value(&event));
            }
        />

        
        <button
                on:click=move |event| setter.update(|value| {
                    
                    value.push(Entry { 
                        key: chrono::Local::now().timestamp().to_string(), 
                        value: RwSignal::new(text.read().to_string()),
                        select: RwSignal::new("".to_string())
                    
                    });
                    
                    set_text.set("".to_string());

                    

                    
                })>
            "Add"
        </button>

        <div>
        {move || text.get()}

        </div>
    }
}


fn App() -> impl IntoView {

    let (entries, set_entries) = signal(Vec::<Entry>::new());

    provide_context(set_entries);
    provide_context(entries);

    view! {


        <InputBox/>

        <GridLayout/>

        <GridLayout/>
        
    }
}



fn main() {
    
    mount_to_body(App);
}
