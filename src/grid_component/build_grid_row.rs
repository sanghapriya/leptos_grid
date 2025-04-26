use leptos::{logging::log, prelude::*};


use reactive_stores::{Field, Patch, Store};
use crate::data_type::grid_row::GridRow;
use crate::data_type::grid_row::GridRowStoreFields;
use crate::data_type::grid_field::GridFieldStoreFields;
use crate::data_type::grid_field::GridField;


use super::build_grid_field::BuildGridField;


#[component]
pub fn BuildGridRow(
    
    #[prop(into)] grid_row: Field<GridRow>,

) -> impl IntoView {


    
   


    view! {
       

        <For each=move || grid_row.fields()
        key=move |item| item.id().get()
        let(grid_field)
        >
        <td>
        <BuildGridField grid_field = grid_field/>
       
        </td>
        </For>

        // button to remove row
        
       
    }


}