use core::hash;
use std::{ cell::Cell, fmt::{ Display, LowerHex }, ops::{ Index, IndexMut }, vec };

use crate::{ Result, RuntimeError };

pub struct FusedObject;

#[derive(Clone, Copy, Debug)]
pub enum RegisterContents {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Object(()),
    None,
}

impl Display for RegisterContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.type_name())?;
        (match self {
            RegisterContents::Int(int) => write!(f, "{int}"),
            RegisterContents::Float(float) => write!(f, "{float:.2}"),
            RegisterContents::Bool(bool) => write!(f, "{bool}"),
            RegisterContents::Char(char) => write!(f, "'{char}'"),
            RegisterContents::Object(object) => write!(f, "{object:?}"),
            RegisterContents::None => f.write_str("empty register"),
        })?;
        f.write_str(")")
    }
}

impl PartialEq<RegisterContents> for RegisterContents {
    fn eq(&self, other: &RegisterContents) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

impl Eq for RegisterContents {}

impl hash::Hash for RegisterContents {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Int(int) => int.hash(state),
            Self::Float(float) => float.to_bits().hash(state),
            Self::Bool(bool) => bool.hash(state),
            Self::Char(char) => char.hash(state),
            Self::Object(object) => object.hash(state),
            Self::None => (0).hash(state),
        }
    }
}

impl RegisterContents {
    pub fn type_name(&self) -> &'static str {
        match self {
            RegisterContents::Int(_) => "int",
            RegisterContents::Float(_) => "float",
            RegisterContents::Bool(_) => "bool",
            RegisterContents::Char(_) => "char",
            RegisterContents::Object(_) => "object",
            RegisterContents::None => "none",
        }
    }

    pub(crate) fn try_add(&self, other: &Self) -> Result<Self> {
        println!("{}", self == &Self::None);
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l + r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l + r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("add")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "add",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_sub(&self, other: &RegisterContents) -> Result<Self> {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l - r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l - r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("sub")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "sub",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_mul(&self, right: &RegisterContents) -> Result<Self> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l * r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l * r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("mul")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "mul",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_div(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l / r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l / r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("div")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "div",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_mod(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l % r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l % r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("mod")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "mod",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_bitand(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l & r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("band")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "bitand",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_bitor(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l | r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("bor")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "bitor",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_bitxor(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l ^ r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("bxor")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "bitxor",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_leftshift(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l << r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("lshift")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "leftshift",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_rightshift(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l >> r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("rshift")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "rightshift",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_or(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Bool(l), Self::Bool(r)) => Ok(Self::Bool(*l || *r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("or")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "or",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_and(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Bool(l), Self::Bool(r)) => Ok(Self::Bool(*l && *r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("and")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "and",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }

    pub(crate) fn try_eq(&self, right: &RegisterContents) -> Result<RegisterContents> {
        match (self, right) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Bool(l == r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Bool(l == r)),
            (Self::Bool(l), Self::Bool(r)) => Ok(Self::Bool(l == r)),
            (Self::Char(l), Self::Char(r)) => Ok(Self::Bool(l == r)),
            (Self::Object(l), Self::Object(r)) => Ok(Self::Bool(l == r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess("eq")),
            (other_left, other_right) =>
                Err(
                    RuntimeError::InvalidOperation(
                        "eq",
                        other_left.type_name(),
                        other_right.type_name()
                    )
                ),
        }
    }
}

pub struct Stack {
    pub values: Vec<RegisterContents>,
    pub variables: Vec<RegisterContents>,
    depth: Cell<u16>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: vec![],
            variables: vec![],
            depth: Cell::new(0),
        }
    }

    pub fn push_frame(&mut self) {
        self.variables.extend([RegisterContents::None; u16::MAX as usize]);
        self.depth.set(self.depth.get() + 1)
    }

    pub fn pop_frame(&mut self) {
        self.variables.drain(0..u16::MAX as usize);
        self.depth.set(self.depth.get() - 1);
    }

    pub fn push(&mut self, value: RegisterContents) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Result<RegisterContents> {
        self.values.pop().ok_or(RuntimeError::BadStackFrame(0))
    }

    pub fn load(&self, variable: u16) -> RegisterContents {
        self.variables[variable as usize]
    }

    pub fn store(&mut self, variable: u16, value: RegisterContents) {
        self.variables[variable as usize] = value;
    }

    pub fn is_root(&self) -> bool {
        self.depth.get() == 0
    }
}
