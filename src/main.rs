use leptos::{mount_to_body, view};

use routes::R;

mod components;
mod pages;
mod routes;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <R /> });
}
