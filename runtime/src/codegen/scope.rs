use std::{ cell::RefCell, collections::HashMap };

pub type SymbolId = u16;

#[derive(Debug, Default)]
pub struct SymbolTable {
    contents: RefCell<Vec<HashMap<String, SymbolId>>>,
}

impl SymbolTable {
    pub fn declare(&self, name: String) -> SymbolId {
        let mut contents = self.contents.borrow_mut();
        let map = contents.last_mut().expect("Attempted to declare symbol in empty symbol table");

        let id = map.len() as SymbolId;
        map.insert(name, id);
        id
    }

    pub fn push(&self) {
        self.contents.borrow_mut().push(HashMap::new());
    }

    pub fn pop(&self) {
        self.contents.borrow_mut().pop().expect("Attempted to pop empty symbol table");
    }

    pub fn get(&self, name: &str) -> Option<(u8, SymbolId)> {
        for (depth, table) in self.contents.borrow().iter().rev().enumerate() {
            if let Some(index) = table.get(name) {
                return Some((depth as u8, *index));
            }
        }
        None
    }
}
