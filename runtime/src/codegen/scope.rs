use std::{ cell::{ Cell, RefCell }, collections::HashMap };

pub type SymbolId = u32;

#[derive(Debug, Default)]
pub struct SymbolTable {
    contents: RefCell<Vec<HashMap<String, SymbolId>>>,
    count: Cell<SymbolId>,
}

impl SymbolTable {
    pub fn declare(&self, name: String) -> SymbolId {
        let mut contents = self.contents.borrow_mut();
        let map = contents.last_mut().expect("Attempted to declare symbol in empty symbol table");

        let id = self.count.get();
        map.insert(name, id);
        self.count.set(id + 1);
        id
    }

    pub fn push(&self) {
        self.contents.borrow_mut().push(HashMap::new());
    }

    pub fn pop(&self) {
        let freed = self.contents.borrow_mut().pop().expect("Attempted to pop empty symbol table");
        self.count.set(self.count.get() - (freed.len() as SymbolId));
    }

    pub fn get(&self, name: &str) -> Option<SymbolId> {
        for table in self.contents.borrow().iter().rev(){
            if let Some(index) = table.get(name) {
                return Some(*index);
            }
        }
        None
    }

    pub fn total_vars(&self) -> SymbolId {
        self.count.get() + 1
    }
}
