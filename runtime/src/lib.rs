use std::{ collections::{ HashMap, BinaryHeap }, rc::Rc };

use parser::ast::ident::Ident;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub(crate) struct FusedValue;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub(crate) struct FusedValueRef(Rc<FusedValue>);

pub struct Runtime {
    values: BinaryHeap<Rc<FusedValue>>,
    global: Module,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
            global: Module::global(),
        }
    }

    pub(crate) fn add_value(&mut self, value: FusedValue) -> FusedValueRef {
        let value = Rc::new(value);
        self.values.push(value.clone());
        FusedValueRef(value)
    }
}

struct Scope<'a> {
    current_values: HashMap<Ident, FusedValueRef>,
    visible_modules: HashMap<Ident, &'a Module>,
    runtime: &'a Runtime,
}

struct Module {
    name: String,
    items: HashMap<String, FusedValueRef>,
    children: HashMap<String, Module>,
}

impl Module {
    pub fn global() -> Self {
        Self {
            name: String::from(""),
            items: Default::default(),
            children: Default::default(),
        }
    }
}
