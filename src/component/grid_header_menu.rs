use leptos::prelude::*;
use reactive_stores::Store;

use crate::data_type::{grid::Grid, grid_rows::{self, GridRows}};

use crate::data_type::grid_rows::GridRowsStoreFields;
use crate::data_type::grid_row::GridRowStoreFields;
use crate::data_type::grid::GridStoreFields;


#[component]
pub fn GridHeaderMenu(grid_rows: Store<GridRows>, header_store: Store<Grid>, header_id: usize) -> impl IntoView {
    view! {
        <style>
        "
        .context-menu {
            width: 100px;
            
            border: 1px solid #3C3C3C;
            border-radius: 6px;
            
            font-family: sans-serif;
            padding: 8px;
        }

        /* Menu list */
        .menu-list {
            list-style: none;
            margin: 0;
            padding: 4px 0;
            border-top: 1px solid #4A4A4A;
            border-bottom: 1px solid #4A4A4A;
        }

        /* Menu items */
        .menu-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 6px 8px;
            cursor: pointer;
            font-size: 14px;
        }


        /* Label and shortcuts */
        .menu-label {
            flex: 1;
        }

        .shortcut {
            
            font-size: 0.85em;
            margin-left: 8px;
        }

        /* Footer text */
        .footer-text {
            margin-top: 8px;
            font-size: 12px;
            
            line-height: 1.4;
        }
        "
        </style>

        

        <table>
            
            <tbody>
                <tr>
                    
                    <td>
                        <button on:click = move |_| {
                            for row in grid_rows.rows() {
                                row.fields().write().retain(|field| field.header_id != header_id);
                            }
        
                            header_store.headers().write().retain(|header| header.id != header_id);

                        }> "Delete"
                        </button>
                    </td>
                </tr>
            </tbody>
        </table>
       
    }
}
