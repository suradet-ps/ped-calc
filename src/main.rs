mod app;
mod components;
mod data;
mod logic;
mod pages;
mod state;
mod types;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
