use leptos::prelude::*;

mod component;
mod data_type;

use crate::component::grid_layout_multi::GridLayoutMulti;
use crate::data_type::entry_multiple::EntryMultiple;

fn App() -> impl IntoView {
    let (entries_multiple, set_entries_multiple) = signal(Vec::<EntryMultiple>::new());

    provide_context(entries_multiple);
    provide_context(set_entries_multiple);

    view! {
        <GridLayoutMulti/>
        <GridLayoutMulti/>


    }
}

fn main() {
    mount_to_body(App);
}
