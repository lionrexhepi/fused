use std::{ collections::{ HashMap, BinaryHeap }, rc::Rc };

use parser::{ ast::{ ident::Ident, Spanned }, Span };
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub(crate) struct FusedValue;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub(crate) struct FusedValueRef(Rc<FusedValue>);

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum RuntimeError {
    #[error("Couldn't find symbol {0}")] UndefinedSymbol(Ident),
    #[error("Symbol {0} has duplicate definitions!")] DuplicateSymbol(Ident),
}

pub struct Runtime {
    values: BinaryHeap<Rc<FusedValue>>,
}

pub(crate) struct Context {
    symbols: HashMap<Ident, FusedValueRef>,
    modules: HashMap<Ident, Context>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            modules: HashMap::new(),
        }
    }

    pub fn get_symbol(&self, ident: &Ident) -> RuntimeResult<FusedValueRef> {
        self.symbols
            .get(ident)
            .cloned()
            .ok_or_else(|| RuntimeError::UndefinedSymbol(ident.clone()))
    }

    pub fn get_module(&self, ident: &Ident) -> RuntimeResult<&Context> {
        self.modules.get(ident).ok_or_else(|| RuntimeError::UndefinedSymbol(ident.clone()))
    }

    pub fn define_symbol(
        &mut self,
        ident: Ident,
        value: FusedValueRef,
        overwrite: bool
    ) -> RuntimeResult<()> {
        if self.symbols.contains_key(&ident) && !overwrite {
            return Err(RuntimeError::DuplicateSymbol(ident));
        }

        self.symbols.insert(ident, value);
        Ok(())
    }
}
