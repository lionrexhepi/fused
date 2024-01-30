use std::{ cell::{ Cell, RefCell }, collections::HashMap };

pub type SymbolId = u32;

#[derive(Debug)]
pub struct SymbolTable {
    contents: RefCell<Vec<HashMap<String, SymbolId>>>,
    count: Cell<SymbolId>,
    current_free: Cell<SymbolId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            contents: RefCell::new(vec![HashMap::new()]),
            count: Cell::new(0),
            current_free: Cell::new(0),
        }
    }

    pub fn declare(&self, name: String) -> SymbolId {
        let mut contents = self.contents.borrow_mut();
        let map = contents.last_mut().expect("Attempted to declare symbol in empty symbol table");

        let id = self.current_free.get();
        map.insert(name, id);
        self.count.set(self.count.get() + 1);
        self.current_free.set(id + 1);
        id
    }

    pub fn push(&self) {
        self.contents.borrow_mut().push(HashMap::new());
    }

    pub fn pop(&self) {
        if self.contents.borrow().len() == 1 {
            panic!("Attempted to pop last symbol table");
        }
        let freed = self.contents.borrow_mut().pop().expect("Attempted to pop empty symbol table");

        self.current_free.set(self.current_free.get() - (freed.len() as SymbolId));
    }

    pub fn get(&self, name: &str) -> Option<SymbolId> {
        for table in self.contents.borrow().iter().rev() {
            if let Some(index) = table.get(name) {
                return Some(*index);
            }
        }
        None
    }

    pub fn total_vars(&self) -> SymbolId {
        self.count.get()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_new_table() {
        let table = super::SymbolTable::new();
        assert_eq!(table.total_vars(), 0);
    }

    #[test]
    fn test_declare() {
        let table = super::SymbolTable::new();
        assert_eq!(table.total_vars(), 0);

        let id = table.declare("test".to_string());
        assert_eq!(id, 0);

        assert_eq!(table.total_vars(), 1);
    }

    #[test]
    fn test_declare_multiple() {
        let table = super::SymbolTable::new();
        assert_eq!(table.total_vars(), 0);

        let id = table.declare("test".to_string());
        assert_eq!(id, 0);
        assert_eq!(table.total_vars(), 1);

        let id = table.declare("test2".to_string());
        assert_eq!(id, 1);
        assert_eq!(table.total_vars(), 2);
    }

    #[test]
    fn test_declare_multiple_scopes() {
        let table = super::SymbolTable::new();
        assert_eq!(table.total_vars(), 0);

        let id = table.declare("test".to_string());
        assert_eq!(id, 0);
        assert_eq!(table.total_vars(), 1);

        table.push();

        let id = table.declare("test2".to_string());
        assert_eq!(id, 1);
        assert_eq!(table.total_vars(), 2);

        table.pop();

        let id = table.declare("test3".to_string());
        assert_eq!(id, 1); //Since the previous "1" went out of scope, the slot 1 is now free to reuse
        assert_eq!(table.total_vars(), 3); //The total number of variables is still 2 because symbols from a higher scope should not overlap
    }

    #[test]
    fn test_complex_nesting() {
        let table = super::SymbolTable::new();
        assert_eq!(table.total_vars(), 0);

        let id = table.declare("test".to_string());
        assert_eq!(id, 0);
        assert_eq!(table.total_vars(), 1);

        table.push();

        let id = table.declare("test2".to_string());
        assert_eq!(id, 1);
        assert_eq!(table.total_vars(), 2);

        table.push();

        let id = table.declare("test3".to_string());
        assert_eq!(id, 2);
        assert_eq!(table.total_vars(), 3);

        table.pop();

        let id = table.declare("test4".to_string());
        assert_eq!(id, 2);
        assert_eq!(table.total_vars(), 4);

        table.pop();

        let id = table.declare("test5".to_string());
        assert_eq!(id, 1);
        assert_eq!(table.total_vars(), 5);
    }
}
