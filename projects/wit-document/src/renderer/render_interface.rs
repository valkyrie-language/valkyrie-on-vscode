use super::*;
use std::borrow::Cow;

impl DocumentElementIcon for Interface {
    fn get_kind_name(&self) -> &'static str {
        "interface"
    }

    fn get_icon_name(&self) -> char {
        'I'
    }
}

impl DocumentElement for Interface {
    fn get_name(&self, _: &DataProvider) -> Cow<str> {
        match self.name.as_ref() {
            Some(name) => Cow::Borrowed(name),
            None => Cow::Borrowed(""),
        }
    }

    fn get_link(&self, data: &DataProvider) -> String {
        let interface = self.name.as_ref().expect("Check for empty interface name first!");
        let package = &data.package.name;
        match package.version.as_ref() {
            Some(version) => format!("/{}:{}/{}@{}", package.namespace, package.name, interface, version,),
            None => format!("/{}:{}/{}", package.namespace, package.name, interface,),
        }
    }

    fn main_body(&self, data: &DataProvider) -> Element {
        let resources = main_resources(data, &self.types);
        let functions = main_functions(data, &self.functions);
        let flags = main_flags(data, &self.types);
        let enumerate = main_enumerate(data, &self.types);
        let variants = main_variant(data, &self.types);
        rsx! {
            div {
                class: "main-card",
                {resources}
                {functions}
                {flags}
                {enumerate}
                {variants}
            }
        }
    }
}
