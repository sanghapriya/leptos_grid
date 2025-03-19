use leptos::prelude::*;
use crate::data_type::entry::Entry;



#[component]
pub fn GridCell(entry: Entry) -> impl IntoView {
    let setter = use_context::<WriteSignal<Vec<Entry>>>().unwrap();


    
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
        
        <input 
        class="grid-input"
        type="text" 
        value=entry.value 
        on:input=move |event| {
            entry.value.set(event_target_value(&event));
            
        }/>
        <select
            prop:value=entry.select
            on:change=move |event| {
                entry.select.set(event_target_value(&event));
            }
        >
            <option value="Option 1">"Option 1"</option>
            <option value="Option 2">"Option 2"</option>
            <option value="Option 3">"Option 3"</option>
        </select>
            <button class = "delete-btn" on:click=move |_| {
                
                setter.update(|entries| {
                    entries.retain(|e| e.key != entry.key);
                });
            }>
            
            <svg class="delete-icon" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path d="M3 6l1 9a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2l1-9H3zm9.5-4a1 1 0 0 1 1 1V4H2V3a1 1 0 0 1 1-1h9.5z"/>
        </svg>
        
        </button>
        </div>
    }
}