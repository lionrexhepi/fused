use std::{ vec, cell::Cell, ops::{ Range, Index }, result };

use libimmixcons::{ object::{ HeapObject, Tracer, GCRTTI }, make_rtti_for };

use crate::{ Result, RuntimeError, alloc::GuardedCell };

pub type Register = u8;

pub struct FusedObject;

impl HeapObject for FusedObject {
    const RTTI: GCRTTI = make_rtti_for!(finalize FusedObject);
}

#[derive(Clone, Copy)]
pub enum StackValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Object(GuardedCell<FusedObject>),
    None,
}
impl PartialEq<StackValue> for StackValue {
    fn eq(&self, other: &StackValue) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::Object(l0), Self::Object(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl StackValue {
    pub fn try_add(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => Ok(Self::Int(l + r)),
            (Self::Float(l), Self::Float(r)) => Ok(Self::Float(l + r)),
            (Self::None, _) | (_, Self::None) => Err(RuntimeError::RegisterEmpty),
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

    pub fn type_name(&self) -> &'static str {
        match self {
            StackValue::Int(_) => "int",
            StackValue::Float(_) => "float",
            StackValue::Bool(_) => "bool",
            StackValue::Char(_) => "char",
            StackValue::Object(_) => "object",
            StackValue::None => "none",
        }
    }
}

pub struct Stack {
    pub registers: Vec<StackValue>,
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
        let start = self.depth.get() as usize;
        let end = start + (Register::MAX as usize);
        start..end
    }

    #[inline]
    pub(self) fn current_frame(&self) -> Result<&[StackValue]> {
        let range = self.current_frame_range();
        if self.registers.len() < range.clone().max().expect("Expected a closed range") {
            Err(RuntimeError::BadStackFrame(self.depth.get()))
        } else {
            Ok(&self.registers[range])
        }
    }

    #[inline]
    pub(self) fn current_frame_mut(&mut self) -> Result<&mut [StackValue]> {
        let range = self.current_frame_range();
        if self.registers.len() < range.clone().max().expect("Expected a closed range") {
            Err(RuntimeError::BadStackFrame(self.depth.get()))
        } else {
            Ok(&mut self.registers[range])
        }
    }

    pub fn push_frame(&mut self) -> StackFrame {
        self.depth.set(self.depth.get() + (Register::MAX as u16));
        self.registers.extend([StackValue::None; Register::MAX as usize]);
        let range = self.current_frame_range();
        StackFrame {
            stack: self,
            range,
        }
    }

    pub(self) fn pop_frame(&mut self) {
        self.registers.drain(self.current_frame_range());
        self.depth.set(self.depth.get() - (Register::MAX as u16));
    }
}

pub struct StackFrame<'a> {
    stack: &'a mut Stack,
    range: Range<usize>,
}

impl<'a> StackFrame<'a> {
    pub fn get_value(&self, register: Register) -> Result<StackValue> {
        let frame = self.stack.current_frame()?;
        Ok(frame[register as usize])
    }

    pub fn set_value(&mut self, register: Register, value: StackValue) -> Result<()> {
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
