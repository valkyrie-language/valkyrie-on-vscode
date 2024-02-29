use crate::{
    helpers::{DocumentElement, DocumentElementIcon},
    DataProvider,
};
use dioxus::{
    html::{s, KeyCode::N},
    prelude::*,
};
use indexmap::IndexMap;
use wit_parser::{Function, Interface, TypeDef, TypeDefKind, TypeId, World};

mod render_functions;
mod render_interface;
mod render_types;
mod render_world;

pub fn render_interface(data: &DataProvider, interface: &Interface) -> Element {
    let words = data.package.worlds.iter().map(|(key, value)| left_link(value, data));
    let interfaces = data.package.interfaces.iter().map(|(key, value)| left_link(value, data));
    let card = interface.main_body(data);
    rsx! {
        div {
            class: "container",
            div { class: "lift-list", {words}, {interfaces} }
            div { class: "left-separator" }
            {card}
        }
    }
}

fn left_link<T: DocumentElement + DocumentElementIcon>(item: &T, data: &DataProvider) -> Element {
    match item.get_name(data).as_ref() {
        "" => rsx! {},
        name => {
            let link = item.get_link(data);
            let icon = item.get_icon_name();
            rsx! {
               li {
                    class: "left-link",
                    span { class: "type-icon", "{icon}" }
                    a { href: "{link}", "{name}" }
                }
            }
        }
    }
}

fn main_link<T: DocumentElement + DocumentElementIcon>(item: &T, data: &DataProvider) -> Element {
    let kind = item.get_kind_name();
    match item.get_name(data).as_ref() {
        "" => rsx! {},
        name => {
            let link = item.get_link(data);
            let icon = item.get_icon_name();
            rsx! {
               td {
                    class: "main-link {kind}",
                    span { class: "type-icon", "{icon}" }
                    a { href: "{link}", "{name}" }
                }
            }
        }
    }
}

fn main_resources<'a>(data: &'a DataProvider, item: &'a IndexMap<String, TypeId>) -> Element {
    let title = if data.has_resources(item) {
        rsx! {
             h2 {
                id: "resources",
                "Resources"
            }
        }
    }
    else {
        None
    };
    let terms = data.get_resources(item).into_iter().map(|x| make_card(x, data));
    rsx! {
        div {
            {title}
            table {
                {terms}
            }
        }
    }
}
fn main_flags<'a>(data: &'a DataProvider, item: &'a IndexMap<String, TypeId>) -> Element {
    let title = if data.has_enumerate(item) {
        rsx! {
             h2 {
                id: "flags",
                "flags"
            }
        }
    }
    else {
        None
    };
    let terms = data.get_enumerate(item).into_iter().map(|x| make_card(x.0, data));
    rsx! {
        div {
            {title}
            table {
                {terms}
            }
        }
    }
}
fn main_enumerate<'a>(data: &'a DataProvider, item: &'a IndexMap<String, TypeId>) -> Element {
    let title = if data.has_enumerate(item) {
        rsx! {
             h2 {
                id: "enumerate",
                "Enumerate"
            }
        }
    }
    else {
        None
    };
    let terms = data.get_enumerate(item).into_iter().map(|x| make_card(x.0, data));
    rsx! {
        div {
            {title}
            table {
                {terms}
            }
        }
    }
}

fn main_variant<'a>(data: &'a DataProvider, item: &'a IndexMap<String, TypeId>) -> Element {
    let title = if data.has_variant(item) {
        rsx! {
             h2 {
                id: "variant",
                "Variant"
            }
        }
    }
    else {
        None
    };
    let terms = data.get_variant(item).into_iter().map(|x| make_card(x.0, data));
    rsx! {
        div {
            {title}
            table {
                {terms}
            }
        }
    }
}
fn main_functions<'a>(data: &'a DataProvider, item: &'a IndexMap<String, Function>) -> Element {
    let title = if data.has_functions(item) {
        rsx! {
             h2 {
                id: "functions",
                "Functions"
            }
        }
    }
    else {
        None
    };
    let terms = data.get_functions(item).into_iter().map(|x| make_card(x, data));
    rsx! {
        div {
            {title}
            table {
                {terms}
            }
        }
    }
}

fn make_card<'a, T: DocumentElement + DocumentElementIcon>(item: &T, data: &DataProvider) -> Element {
    let link = main_link(item, data);
    let introduce = item.get_introduce(data);

    rsx! {
        tr {
            td {
                class: "main-card-title",
                {link}
            }
            td {
                class: "main-card-introduce",
                {introduce}
            }
        }
    }
}
