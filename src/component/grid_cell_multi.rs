use leptos::prelude::*;
use crate::data_type::entry_multiple::EntryMultiple;
use crate::component::grid_cell_element::GridCellElement;




#[component]
pub fn GridCellMulti(entry: EntryMultiple) -> impl IntoView {
    let setter = use_context::<WriteSignal<Vec<EntryMultiple>>>().unwrap();

    let all_fields_string = entry.fields.iter().map(|(key, _)| {
        format!("{}", key)
    }).collect::<Vec<String>>();


    view! {
        <style>
        "
    .appian-grid { 
        display: flex; 
        flex-direction: column; 
        gap: 8px; 
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 12px;
        background-color: white;
        width: 100%;
        max-width: 600px; /* Ensures a structured grid layout */
    }

    .grid-row { 
        display: flex; 
        align-items: center; 
        justify-content: space-between; /* Ensures input and button are on opposite sides */
        gap: 10px; 
        border-bottom: 1px solid #e0e0e0; 
        padding: 6px; 
        width: 100%;
    }

    .grid-input { 
        flex: 1; /* Allows input field to take available space */
        padding: 6px; 
        border: 1px solid #ccc; 
        border-radius: 4px; 
        min-width: 0; /* Prevents input from overflowing */
    }

    .delete-btn { 
        background: transparent; 
        border: none; 
        cursor: pointer; 
        display: flex; /* Ensures proper alignment */
        align-items: center;
        justify-content: center;
        padding: 4px;
    }

    .delete-icon { 
        width: 20px; 
        height: 20px; 
        fill: #d32f2f; 
        transition: fill 0.2s; 
    }

    .delete-btn:hover .delete-icon { 
        fill: #b71c1c; 
    }"
</style>

        <div class="grid-row">
         
         {
             all_fields_string.into_iter()
                .map(|fild_key| 
                    view! {
                    <GridCellElement element = entry.fields.get(&fild_key).unwrap().value.clone()/>
                    }).collect::<Vec<_>>()
                
            }
            <button  on:click=move |_| {
                
                setter.update(|entries| {
                    entries.retain(|e| e.key != entry.key);
                });
            }>
            
            -
        
        
        </button>
        </div>
    }
}