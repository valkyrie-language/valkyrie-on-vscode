//! Run with:
//!
//! ```sh
//! dx build --features web --release
//! cargo run --features server
//! ```

#![allow(unused)]
use dioxus::prelude::*;
use dioxus_router::prelude::Routable;
use dioxus_fullstack::{launch, prelude::*};
use dioxus_ssr::incremental::{DefaultRenderer, IncrementalRenderer};
use serde::{Deserialize, Serialize};
use dioxus_router::prelude::Link;

fn main() {
    dioxus_web::launch::launch_cfg(
        Home,
        dioxus_web::Config::default().hydrate(true),
    );
}

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/blog")]
    Blog,
}

#[component]
fn Blog() -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        table {
            tbody {
                for _ in 0..100 {
                    tr {
                        for _ in 0..100 {
                            td { "hello world!" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let text = use_signal(|| "...".to_string());

    rsx! {
        Link { to: Route::Blog {}, "Go to blog" }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}