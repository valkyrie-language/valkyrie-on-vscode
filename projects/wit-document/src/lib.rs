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

use crate::renderer::render_interface;

use dioxus::{dioxus_core::Mutations, prelude::*};
use std::path::Path;
use wit_parser::{SourceMap, UnresolvedPackage};

// use axum::{response::Html, routing::get, Router};
// pub async fn app_endpoint() -> Html<String> {
//     let mut app = VirtualDom::new(app);
//     let mut mutations = Mutations::default();
//     app.rebuild(&mut mutations);
//     Html(dioxus_ssr::render(&app))
// }

// create a component that renders a div with the text "hello world"

#[component]
pub fn HttpExample() -> Element {
    rsx! {
        {filesystem()}
        {https()}
    }
}

fn https() -> Element {
    let mut map = SourceMap::default();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    {
        let file = "preview2/http/types.wit";
        let contents = include_str!("../../preview2/http/types.wit");
        map.push(here, contents);
    }
    {
        let file = "preview2/http/proxy.wit";
        let contents = include_str!("../../preview2/http/proxy.wit");
        map.push(here, contents);
    }
    {
        let file = "preview2/http/handler.wit";
        let contents = include_str!("../../preview2/http/handler.wit");
        map.push(here, contents);
    }
    let store = DataProvider { package: map.parse().unwrap() };
    let example = store.get_interfaces().into_iter().map(|x| render_interface(&store, x));
    rsx! {
        {example}
    }
}

fn filesystem() -> Element {
    let mut map = SourceMap::default();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    {
        let file = "preview2/filesystem/world.wit";
        let contents = include_str!("../../preview2/filesystem/world.wit");
        map.push(here, contents);
    }
    {
        let file = "preview2/filesystem/types.wit";
        let contents = include_str!("../../preview2/filesystem/types.wit");
        map.push(here, contents);
    }
    {
        let file = "preview2/filesystem/preopens.wit";
        let contents = include_str!("../../preview2/filesystem/preopens.wit");
        map.push(here, contents);
    }
    let store = DataProvider { package: map.parse().unwrap() };
    let example = store.get_interfaces().into_iter().map(|x| render_interface(&store, x));
    rsx! {
        {example}
    }
}
