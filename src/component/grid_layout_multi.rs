use leptos::prelude::*;
use crate::component::grid_cell::GridCell;
use crate::component::grid_cell_multi::GridCellMulti;
use crate::data_type::entry::Entry;
use crate::data_type::entry_multiple::EntryMultiple;
use crate::data_type::field_value::FieldValue;
use std::collections::HashMap;


pub fn GridLayoutMulti() -> impl IntoView {


    let (name, set_name) = signal("".to_string());
    let (last_name, set_last_name) = signal("".to_string());    
    let (age, set_age) = signal("".to_string());
    let (address, set_address) = signal("".to_string());
    let (city, set_city) = signal("".to_string());
    


    let setter = use_context::<WriteSignal<Vec<EntryMultiple>>>().unwrap();
    let getter = use_context::<ReadSignal<Vec<EntryMultiple>>>().unwrap();
    

    view! {
        <h2>"Grid Multi"</h2>

        <input
            type="text"
            bind:value=(name,set_name)
            on:input=move |event| {
            
            set_name.set(event_target_value(&event));
            }   

        />
        <input
            type="text"
            bind:value=(last_name,set_last_name)
            on:input=move |event| {
            
            set_last_name.set(event_target_value(&event));
            }   

        />
        <input
            type="text"
            bind:value=(age,set_age)
            on:input=move |event| {
            
            set_age.set(event_target_value(&event));
            }   

        />
        <input
            type="text"
            bind:value=(address,set_address)
            on:input=move |event| {
            
            set_address.set(event_target_value(&event));
            }
            />
        <input
            type="text"
            bind:value=(city,set_city)
            on:input=move |event| {
            
            set_city.set(event_target_value(&event));
            }
            />

        <button
            on:click=move |event| setter.update(|value| {
                
                let mut fields = HashMap::new();
                fields.insert("name".to_string(), FieldValue::new(name.read().to_string(), "text"));
                fields.insert("last_name".to_string(), FieldValue::new(last_name.read().to_string(), "text"));
                fields.insert("age".to_string(), FieldValue::new(age.read().to_string(), "number"));
                fields.insert("address".to_string(), FieldValue::new(address.read().to_string(), "text"));
                fields.insert("city".to_string(), FieldValue::new(city.read().to_string(), "text"));
                
                value.push(EntryMultiple::new(chrono::Local::now().timestamp().to_string(), fields));
                
                set_name.set("".to_string());
                set_last_name.set("".to_string());
                set_age.set("".to_string());
                set_address.set("".to_string());
                set_city.set("".to_string());
                
            })>
            "Add"
        </button>

        // <button
        //         on:click=move |event| getter.update(|value| {
                    
                    // value.push(Entry { 
                    //     key: chrono::Local::now().timestamp().to_string(), 
                    //     value: RwSignal::new(text.read().to_string()),
                    //     select: RwSignal::new("".to_string())
                    
                    // });
                    
        //             set_name.set("".to_string());
        //             set_last_name.set("".to_string());
        //             set_age.set("".to_string());
                    
        //         })>
        //     "Add"
        // </button>


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

    .grid-input-header {
  font-weight: bold;
  background-color: #f3f4f6; /* light gray like a header */
  border: 1px solid #d1d5db; /* subtle border */
  padding: 6px 10px;
  text-align: center;
  width: 100%;
  box-sizing: border-box;
  outline: none;
  cursor: default;
}


  .plus-button {
  all: unset;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  font-size: 20px;
  font-weight: bold;
  background-color: #4f46e5; /* indigo */
  color: white;
  border-radius: 50%;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.plus-button::before {
  content: "+";
}

.plus-button:hover {
  background-color: #4338ca; /* darker indigo on hover */
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
                class="grid-input-header"
                type="text" 
                value="Column 1"
                
                /> 
                <input 
                class="grid-input-header"
                type="text" 
                value="Column 2"
                
                /> 
                <input 
                class="grid-input-header"
                type="text" 
                value="Column 3"
                
                /> 
                <input 
                class="grid-input-header"
                type="text" 
                value="Column 4"
                
                /> 
                <input 
                class="grid-input-header"
                type="text" 
                value="Column 5"
                
                /> 
                <button  aria-label="Add">+</button>



        </div>


        


        <For each=move || getter.get()
        key=move |item| item.key.clone()
        let(it)
        
        >
        <GridCellMulti entry = it/>
        </For>

        <br/>

        <button> + </button>

        
        
   
      
    }
}