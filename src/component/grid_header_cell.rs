use leptos::{logging::log, prelude::*};
use crate::data_type::grid::GridStoreFields;
use crate::component::grid_header_cell_element::GridHeaderCellElement;
use crate::data_type::grid::Grid;
use crate::data_type::grid_rows::GridRows;
use crate::data_type::grid_field::GridField;

use reactive_stores::{Field, Patch, Store};
use crate::data_type::header_config::HeaderConfig;
use crate::data_type::header_config::HeaderConfigStoreFields;
use crate::data_type::grid_row::GridRowStoreFields;
use crate::data_type::grid_rows::GridRowsStoreFields;

use crate::component::grid_header_menu::GridHeaderMenu;

#[component]
pub fn GridHeaderCell() -> impl IntoView {
    let grid = expect_context::<Store<Grid>>();
    let grid_store = expect_context::<Store<GridRows>>();

    view! {
        <style>
        "
        .floating {
            position: absolute;
            top: 80%; /* places it just below the header */
            // left: 50%;  /* center of the parent */
            transform: translateX(-50%);
            background-color: white;
            border: 1px solid #ccc;
            z-index: 1000;
            box-shadow: 0 2px 6px rgba(0, 0, 0, 0.15);
        }

        .close-btn {
            all: unset;
            position: absolute;
            top: 4px;
            right: 6px;
            font-size: 14px;
            font-weight: bold;
            cursor: pointer;
            color: #999;
            transition: color 0.2s;
        }

        .close-btn:hover {
            color: red;
        }
        "
        </style>

        <For
            each=move || grid.headers()
            key=|header| header.id().get()
            let:child
        >
            <th>
                <GridHeaderCellElement header=child />

                <button
                    on:click=move |_| {
                        let show_menu = child.show_menu();

                        // Close other menus if open
                        for header in grid.headers() {
                            if header.id().get() != child.id().get() {
                                header.show_menu().set(false);
                            }
                        }
                        // Toggle the menu visibility
                        show_menu.set(!show_menu.get());
                    }
                >
                    "m"
                </button>

                <Show
                    when=move || child.show_menu().get()
                    fallback=|| view! { <></> }
                >
                    <div class="floating">
                        <button
                            // class="close-btn"
                            on:click=move |_| {
                                child.show_menu().set(false);
                            }
                        >
                            x
                        </button>
                        <GridHeaderMenu
                            grid_rows=grid_store
                            header_store=grid
                            header_id=child.id().get()
                        />
                    </div>
                </Show>
            </th>
        </For>

        <th>
            <button
                class="small-btn"
                on:click=move |_| {
                    let new_header = HeaderConfig::new("New Header".to_string(), false);
                    let new_header_id = new_header.id;

                    grid.headers().write().push(new_header);

                    let grid_rows = grid_store.rows().get();

                    // Clone and make mutable
                    let mut grid_rows_clone = grid_rows.clone();

                    // Mutate each row to add a new field
                    for row in grid_rows_clone.iter_mut() {
                        row.fields.push(GridField::new("".to_string(), new_header_id));
                    }

                    // Update the store
                    grid_store.rows().set(grid_rows_clone);
                }
            >
                +
            </button>
        </th>
    }
}
