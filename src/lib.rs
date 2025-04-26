use data_type::grid;
use leptos::prelude::*;

use std::sync::atomic::{AtomicUsize, Ordering};
use reactive_stores::{Field, Patch, Store};
use crate::data_type::grid::Grid;
use crate::data_type::grid_rows::GridRows;
use crate::data_type::grid_row::GridRow;
use crate::data_type::grid_field::GridField;
use crate::grid_component::build_grid_rows::BuildGridRows;


mod component;
mod data_type;
mod grid_component;

use crate::component::grid_layout_multi::GridLayoutMulti;

#[component]
fn App() -> impl IntoView {

    provide_context(Store::new(Grid::default()));



    let grid_store = Store::new(GridRows::default());

    provide_context(grid_store);



    let grid_header = expect_context::<Store<Grid>>();

    view! {

        <style>
        ".data-table {
  
    border-collapse: collapse;
    table-layout: fixed; /* important! */
}

    .data-table thead {
    background-color: #f2f2f2;
    font-weight: bold;

    }

.data-table th{
    border: 1px solid #ccc;
    padding: 6px;
    text-align: left;
    vertical-align: middle;
    box-sizing: border-box;
    overflow: visible;
    white-space: nowrap;
    position: relative;
}
.data-table td{
    border: 1px solid #ccc;
    padding: 6px;
    text-align: left;
    vertical-align: middle;
    box-sizing: border-box;
    overflow: visible;
    white-space: nowrap;
  
}




.small-btn {
    width: 20px;
    height: 20px;
    font-size: 12px;
    padding: 0;
}

"
</style>

        <table class ="data-table">
        <thead>

        <GridLayoutMulti/>
        </thead>
        <tbody>
        <BuildGridRows grid_rows = grid_store/>
        </tbody>
        </table>
        <br/>

        <Show 
            when=move || (grid_header.read().headers.len() > 0 )
            fallback=|| view! { <p>"Add Headers to Start"</p> }
        >
        <button
        on:click=move |_| {

            let headers = grid_header.get().headers;

            grid_store.update(|grid_store| {
                let mut grid_fields: Vec<GridField> = vec![];

                for header in headers {

                    grid_fields.push(GridField::new("".to_string(),header.id));

                    
                }

                grid_store.rows.push(
                    GridRow::new(grid_fields)
                );
                
            });
            }
    >
        "Add Row"
    </button>
    </Show>
       


  
    }
}



pub fn run_app() {
    // Initialize the app
    mount_to_body(App);
}