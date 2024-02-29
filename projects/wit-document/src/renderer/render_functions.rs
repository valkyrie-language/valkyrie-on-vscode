use super::*;
use std::borrow::Cow;
use wit_parser::{Function, FunctionKind};

impl DocumentElementIcon for FunctionKind {
    fn get_icon_name(&self) -> char {
        match self {
            FunctionKind::Freestanding => 'f',
            FunctionKind::Method(_) => 'm',
            FunctionKind::Static(_) => 'S',
            FunctionKind::Constructor(_) => 'C',
        }
    }
}

impl DocumentElementIcon for Function {
    fn get_icon_name(&self) -> char {
        self.kind.get_icon_name()
    }
}

impl DocumentElement for Function {
    fn get_name(&self, _: &DataProvider) -> Cow<str> {
        Cow::Borrowed(self.name.as_ref())
    }

    fn get_link(&self, data: &DataProvider) -> String {
        self.get_name(data).to_string()
    }

    fn main_body(&self, data: &DataProvider) -> Element {
        todo!()
    }
}
