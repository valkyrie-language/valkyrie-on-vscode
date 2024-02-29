// #![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod errors;
pub mod renderer;
mod resolver;

mod helpers;

pub use crate::{
    errors::{DocumentError, Result},
    resolver::DataProvider,
};

use crate::{renderer::render_interface, };
use axum::{response::Html, routing::get, Router};
use dioxus::{dioxus_core::Mutations, prelude::*};
use std::path::Path;
use wit_parser::UnresolvedPackage;

pub async fn app_endpoint() -> Html<String> {
    let mut app = VirtualDom::new(app);
    let mut mutations = Mutations::default();
    app.rebuild(&mut mutations);
    Html(dioxus_ssr::render(&app))
}

// create a component that renders a div with the text "hello world"
pub fn app() -> Element {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let store = DataProvider { package: UnresolvedPackage::parse_dir(&here.join("../preview2/http")).unwrap() };
    let example = store.get_interfaces().into_iter().map(|x| render_interface(&store, x));
    rsx! {
        {example}
    }
}
