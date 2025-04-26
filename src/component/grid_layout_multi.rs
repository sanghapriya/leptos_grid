use leptos::prelude::*;
use crate::component::grid_header_cell::GridHeaderCell;

#[component]
pub fn GridLayoutMulti() -> impl IntoView {
    view! {

        <tr>
        
            <GridHeaderCell />
        </tr>
    }
}
