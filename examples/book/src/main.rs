mod app;
mod code_example;
mod pages;

use app::App;
use leptos::prelude::*;

fn main() {
    birei::embed_assets().expect("failed to embed birei assets");
    mount_to_body(App);
}
