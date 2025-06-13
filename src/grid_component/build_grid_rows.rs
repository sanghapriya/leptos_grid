use leptos::{logging::log, prelude::*};
use reactive_stores::{Field, Patch, Store};
use crate::data_type::grid_rows::GridRows;
use crate::data_type::grid_rows::GridRowsStoreFields;
use crate::data_type::grid_row::GridRowStoreFields;
use super::build_grid_row::BuildGridRow;

#[component]
pub fn BuildGridRows(
    #[prop(into)] grid_rows: Store<GridRows>,
    search_query: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <For
            each=move || {
                let q = search_query.get().to_lowercase();
                grid_rows
                    .rows()
                    .into_iter()
                    .filter(|row| {
                        if q.is_empty() {
                            true
                        } else {
                            row
                                .fields()
                                .iter()
                                .any(|field| field.value().get().to_lowercase().contains(&q))
                        }
                    })
                    .collect::<Vec<_>>()
            }
            key=move |item| item.id().get()
            let(grid_row)
        >
            <tr>
                <BuildGridRow grid_row=grid_row />
                <td>
                    <button
                        on:click=move |_| {
                            let id = grid_row.id().get();
                            grid_rows.rows().write().retain(|grid_row| grid_row.id != id);
                        }
                    >
                        "-"
                    </button>
                </td>
            </tr>
        </For>
    }
}
