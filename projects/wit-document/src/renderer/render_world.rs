use super::*;
use std::borrow::Cow;

impl DocumentElementIcon for World {
    fn get_icon_name(&self) -> char {
        'W'
    }
}

impl DocumentElement for World {
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
