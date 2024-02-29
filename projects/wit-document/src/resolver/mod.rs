use indexmap::IndexMap;
use wit_parser::{Enum, Flags, Function, FunctionKind, Interface, TypeDef, TypeDefKind, TypeId, UnresolvedPackage, Variant};

pub struct DataProvider {
    pub package: UnresolvedPackage,
}

impl DataProvider {
    pub fn get_worlds(&self) -> Vec<&String> {
        let mut outputs = Vec::with_capacity(self.package.worlds.len());
        for (_, interface) in self.package.worlds.iter() {
            if !interface.name.is_empty() {
                outputs.push(&interface.name);
            }
        }
        outputs
    }

    pub fn get_interfaces(&self) -> Vec<&Interface> {
        let mut outputs = Vec::with_capacity(self.package.interfaces.len());
        for (_, interface) in self.package.interfaces.iter() {
            match &interface.name {
                Some(name) if !name.is_empty() => {
                    outputs.push(interface);
                }
                _ => {}
            }
        }
        outputs
    }
    pub fn has_resources(&self, dict: &IndexMap<String, TypeId>) -> bool {
        self.get_resources(dict).len() > 0
    }
    pub fn get_resources<'a>(&'a self, dict: &'a IndexMap<String, TypeId>) -> Vec<&'a TypeDef> {
        let mut resources = vec![];
        for ty in dict.values() {
            match self.package.types.get(*ty) {
                Some(s) => match s.kind {
                    TypeDefKind::Resource => {
                        resources.push(s);
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }
    pub fn has_flags<'a>(&'a self, dict: &'a IndexMap<String, TypeId>) -> bool {
        self.get_flags(dict).len() > 0
    }
    pub fn get_flags<'a>(&'a self, dict: &'a IndexMap<String, TypeId>) -> Vec<(&'a TypeDef, &'a Flags)> {
        let mut resources = vec![];
        for ty in dict.values() {
            match self.package.types.get(*ty) {
                Some(s) => match &s.kind {
                    TypeDefKind::Flags(flags) => {
                        resources.push((s, flags));
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }

    pub fn has_enumerate<'a>(&'a self, dict: &'a IndexMap<String, TypeId>) -> bool {
        self.get_enumerate(dict).len() > 0
    }
    pub fn get_enumerate<'a>(&'a self, dict: &'a IndexMap<String, TypeId>) -> Vec<(&'a TypeDef, &'a Enum)> {
        let mut resources = vec![];
        for ty in dict.values() {
            match self.package.types.get(*ty) {
                Some(s) => match &s.kind {
                    TypeDefKind::Enum(e) => {
                        resources.push((s, e));
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }
    pub fn has_variant<'a>(&'a self, dict: &'a IndexMap<String, TypeId>) -> bool {
        self.get_variant(dict).len() > 0
    }
    pub fn get_variant<'a>(&'a self, dict: &'a IndexMap<String, TypeId>) -> Vec<(&'a TypeDef, &'a Variant)> {
        let mut resources = vec![];
        for ty in dict.values() {
            match self.package.types.get(*ty) {
                Some(s) => match &s.kind {
                    TypeDefKind::Variant(v) => {
                        resources.push((s, v));
                    }
                    _ => {}
                },
                None => {}
            }
        }
        resources
    }
    /// Check
    pub fn has_functions<'a>(&'a self, functions: &'a IndexMap<String, Function>) -> bool {
        self.get_functions(functions).len() > 0
    }
    /// Get functions from dict
    pub fn get_functions<'a>(&'a self, functions: &'a IndexMap<String, Function>) -> Vec<&'a Function> {
        functions.values().filter(|x| x.kind == FunctionKind::Freestanding).collect()
    }
}
