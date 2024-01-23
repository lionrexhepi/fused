use std::{collections::HashMap, hash::Hash};

use crate::chunk::Chunk;

use super::{symbols::Symbol, Codegen};

pub type ChildId = usize;
pub type SymbolId = u16;

#[derive(Debug, Default)]
pub struct CodegenScope<'a> {
    symbols: HashMap<String, u16>,
    parent: Option<&'a mut CodegenScope<'a>>,
}

impl<'a> CodegenScope<'a> {
    fn with_parent(parent: &'a mut Self) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn enter_child(&'a mut self) -> Self {
        Self {
            parent: Some(self),
            symbols: Default::default(),
        }
    }

    pub fn declare(&mut self, name: String) -> SymbolId {
        let index = self.symbols.len() as u16;
        self.symbols.insert(name, index);
        index
    }

    pub fn get(&mut self, name: &str) -> Option<(u8, SymbolId)> {
        if let Some(index) = self.symbols.get(name) {
            Some((0, *index))
        } else if let Some(parent) = &mut self.parent {
            let (depth, index) = parent.get(name)?;
            Some((depth + 1, index))
        } else {
            None
        }
    }
}
