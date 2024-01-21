use std::collections::HashMap;

use crate::{ Result, stack::Register, RuntimeError };

pub struct SymbolTable {
    parent: Option<Box<Self>>,
    own: HashMap<String, Symbol>,
    depth: u8,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            parent: None,
            own: Default::default(),
            depth: 0,
        }
    }

    pub fn get(&self, name: &str) -> Option<(Register, u8)> {
        if let Some(sym) = self.own.get(name) {
            Some((sym.value, self.depth))
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn declare(&mut self, name: &str, value: Register, mutable: bool) -> Option<u8> {
        self.own.insert(name.to_string(), Symbol { value, mutable }).map(|s| { s.value })
    }

    pub fn set(&mut self, name: &str, value: Register) -> Result<Register> {
        if let Some(sym) = self.own.get_mut(name) {
            if sym.mutable {
                sym.value = value;
                Ok(value)
            } else {
                Err(RuntimeError::ImmutableSymbol(name.to_string()))
            }
        } else if let Some(parent) = &mut self.parent {
            parent.set(name, value)
        } else {
            Err(RuntimeError::UndefinedSymbol(name.to_string()))
        }
    }

    pub fn push(self) -> Self {
        let depth = self.depth;
        let new = Self {
            parent: Some(Box::new(self)),
            own: Default::default(),
            depth: depth + 1,
        };

        new
    }

    pub fn pop(self) -> Option<Self> {
        self.parent.map(|parent| *parent)
    }
}

pub struct Symbol {
    pub value: Register,
    pub mutable: bool,
}
