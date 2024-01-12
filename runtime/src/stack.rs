use core::hash;
use std::{ vec, cell::Cell, fmt::Display };

use crate::{ Result, RuntimeError };

pub type Register = u8;

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
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l + r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l + r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::NullAccess),
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
    pub registers: Vec<RegisterContents>,
    depth: Cell<u16>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            registers: vec![],
            depth: Cell::new(0),
        }
    }

    #[inline]
    fn current_frame_range(&self) -> std::ops::Range<usize> {
        let end = (self.depth.get() as usize) * (Register::MAX as usize);
        let start = end - (Register::MAX as usize);
        start..end
    }

    #[inline]
    pub(self) fn current_frame(&self) -> Result<&[RegisterContents]> {
        let range = self.current_frame_range();
        if self.registers.len() < range.clone().max().expect("Expected a closed range") {
            Err(RuntimeError::BadStackFrame(self.depth.get()))
        } else {
            Ok(&self.registers[range])
        }
    }

    #[inline]
    pub(self) fn current_frame_mut(&mut self) -> Result<&mut [RegisterContents]> {
        let range = self.current_frame_range();
        if self.registers.len() < range.clone().max().expect("Expected a closed range") {
            Err(RuntimeError::BadStackFrame(self.depth.get()))
        } else {
            Ok(&mut self.registers[range])
        }
    }

    pub fn push_frame(&mut self) -> StackFrame {
        self.depth.set(self.depth.get() + 1);
        self.registers.extend([RegisterContents::None; Register::MAX as usize]);
        StackFrame {
            stack: self,
        }
    }

    pub(self) fn pop_frame(&mut self) {
        self.registers.drain(self.current_frame_range());
        self.depth.set(self.depth.get() - 1);
    }
}

pub struct StackFrame<'a> {
    stack: &'a mut Stack,
}

impl<'a> StackFrame<'a> {
    pub fn get_value(&self, register: Register) -> Result<RegisterContents> {
        let frame = self.stack.current_frame()?;
        Ok(frame[register as usize])
    }

    pub fn set_value(&mut self, register: Register, value: RegisterContents) -> Result<()> {
        let frame = self.stack.current_frame_mut()?;
        frame[register as usize] = value;
        Ok(())
    }

    pub fn create_child(&'a mut self) -> Self {
        self.stack.push_frame()
    }
}

impl<'a> Drop for StackFrame<'a> {
    fn drop(&mut self) {
        self.stack.pop_frame()
    }
}
