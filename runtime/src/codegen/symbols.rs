use std::collections::HashMap;

use crate::{ stack::Register, Result };

pub struct Symbol(u16, bool);

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>, //TODO: make this less inefficient
}

impl SymbolTable {
    pub fn push_scope(&mut self) {
        self.scopes.push(Default::default())
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn register(&mut self, name: String, symbol: Symbol) {
        self.scopes
            .last_mut()

            .expect("Cannot pop global scope")
            .insert(name, symbol);
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(sym) = scope.get_mut(name) {
                return Some(sym);
            }
        }
        None
    }

    pub fn assign(&mut self, name: &str, symbol: Symbol) {}
}
