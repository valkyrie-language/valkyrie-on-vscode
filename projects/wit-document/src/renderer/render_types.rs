use super::*;
use crate::helpers::DocumentElement;
use std::borrow::Cow;

impl DocumentElementIcon for TypeDefKind {
    fn get_kind_name(&self) -> &'static str {
        match self {
            TypeDefKind::Record(_) => {"record"}
            TypeDefKind::Resource => {
                "resource"
            }
            TypeDefKind::Handle(_) => {
                "handle"
            }
            TypeDefKind::Flags(_) => {
                "flags"
            }
            TypeDefKind::Tuple(_) => {
                "tuple"
            }
            TypeDefKind::Variant(_) => {
                "variant"
            }
            TypeDefKind::Enum(_) => {
                "enum"
            }
            TypeDefKind::Option(_) =>  {
                "option"
            }
            TypeDefKind::Result(_) => {
                "result"
            }
            TypeDefKind::List(_) => {
                "list"
            }
            TypeDefKind::Future(_) =>  {
                "future"
            }
            TypeDefKind::Stream(_) =>  {
                "stream"
            }
            TypeDefKind::Type(_) =>  {
                "type"
            }
            TypeDefKind::Unknown =>  {
                "unknown"
            }
        }
    }

    fn get_icon_name(&self) -> char {
        match self {
            Self::Record(_) => 'S',
            Self::Resource => 'R',
            Self::Handle(_) => 'T',
            Self::Flags(_) => 'F',
            Self::Tuple(_) => 'T',
            Self::Variant(_) => 'U',
            Self::Enum(_) => 'E',
            Self::Option(_) => 'T',
            Self::Result(_) => 'T',
            Self::List(_) => 'T',
            Self::Future(_) => 'T',
            Self::Stream(_) => 'T',
            Self::Type(_) => 'T',
            Self::Unknown => '?',
        }
    }
}

impl DocumentElementIcon for TypeDef {
    fn get_kind_name(&self) -> &'static str {
        self.kind.get_kind_name()
    }

    fn get_icon_name(&self) -> char {
        self.kind.get_icon_name()
    }
}

impl DocumentElement for TypeDef {
    fn get_introduce(&self, data: &DataProvider) -> Element {
        let document = match &self.docs.contents {
            None => "",
            Some(document) => document.lines().next().unwrap_or(""),
        };
        rsx! {
            {document}
        }
    }
    fn get_name(&self, _: &DataProvider) -> Cow<str> {
        match self.name.as_ref() {
            Some(name) => Cow::Borrowed(name),
            None => Cow::Borrowed(""),
        }
    }

    fn get_link(&self, data: &DataProvider) -> String {
        self.get_name(data).to_string()
    }

    fn main_body(&self, data: &DataProvider) -> Element {
        todo!()
    }
}


