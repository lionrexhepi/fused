use std::{ hash::Hash, collections::HashMap };

use crate::chunk::Chunk;

use super::Codegen;

pub type ChildId = usize;

#[derive(Debug, Default)]
pub struct CodegenScope<'a> {
    code: Vec<u8>,
    symbols: HashMap<String, u16>,
    children: Vec<Self>,
    parent: Option<&'a mut CodegenScope<'a>>,
}

impl<'a> CodegenScope<'a> {
    fn with_parent(parent: &'a mut Self) -> Self {
        Self {
            code: Vec::new(),
            symbols: HashMap::new(),
            children: Vec::new(),
            parent: Some(parent),
        }
    }

    pub fn enter_child(&mut self, gen: impl FnOnce(&mut Self)) -> ChildId {
        let mut child = Self::default();

        gen(&mut child);
        self.children.push(child);
        self.children.len() - 1
    }

    pub fn declare(&mut self, name: String) -> u16 {
        let index = self.symbols.len() as u16;
        self.symbols.insert(name, index);
        index
    }

    pub fn get(&mut self, name: &str) -> Option<u16> {
        if let Some(index) = self.symbols.get(name) {
            Some(*index)
        } else if let Some(parent) = self.parent.as_mut() {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn to_chunks() -> Vec<Chunk> {
        todo!()
    }
}
