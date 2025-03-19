use leptos::prelude::*;
use crate::component::grid_cell::GridCell;
use crate::data_type::entry::Entry;


pub fn GridLayout() -> impl IntoView {

    let getter = use_context::<ReadSignal<Vec<Entry>>>().unwrap();
    

    view! {
        <h2>"Grid"</h2>

        <For each=move || getter.get()
        key=move |item| item.key.clone()
        let(it)
        
        >
        <GridCell entry = it/>
        </For>


   
      
    }
}