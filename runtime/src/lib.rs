#![feature(non_null_convenience)]

use std::marker::PhantomData;

use alloc::{ Guard, GuardedHeap };
use chunk::Chunk;
use instructions::Instruction;
use libimmixcons::{ threading::{ immix_register_thread, immix_unregister_thread }, immix_collect };
use stack::{ Stack, RegisterContents };
use thiserror::Error;
use array::ArrayCapacity;

pub mod constants;
mod chunk;
pub mod stack;
mod array;
mod alloc;
pub mod codegen;
mod instructions;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("Cannot create an array with capacity {0} of this type")] InvalidArrayCapacity(
        ArrayCapacity,
    ),
    #[error("Failed to allocate memory")]
    AllocationFailure,
    #[error("Invalid instruction: {0:x}")] InvalidInstruction(u8),
    #[error("Chunk ends in the middle of instruction")] InvalidChunkEnd,
    #[error("Invalid register: {0:x}")] InvalidRegister(u8),
    #[error("Bad stack frame {0:x}")] BadStackFrame(u16),
    #[error("Operation {0} unsupported for types {1} and {2}")] InvalidOperation(
        &'static str,
        &'static str,
        &'static str,
    ),
    #[error("Attempted to access empty register")] RegisterEmpty,
    #[error("Chunk does not contain a constant at index {0:x}")] InvalidConstant(u16),
}

pub(crate) type Result<T> = std::result::Result<T, RuntimeError>;

pub struct Thread {
    pub stack: Stack,
}

impl Thread {
    pub fn run_chunk(&mut self, chunk: Chunk) -> Result<RegisterContents> {
        let value;
        {
            immix_register_thread();
            let guard = Guard(PhantomData);
            let heap = GuardedHeap::new(guard);
            value = self.run_guarded(heap, chunk)?;
            immix_unregister_thread();
        }
        immix_collect(true);
        Ok(value)
    }

    fn run_guarded(&mut self, heap: GuardedHeap, chunk: Chunk) -> Result<RegisterContents> {
        let mut ip = 0;
        let mut frame = self.stack.push_frame();
        let return_value = loop {
            if ip == chunk.buffer.len() {
                break RegisterContents::None;
            }
            let (instruction, offset) = Instruction::read_from_chunk(
                &chunk.buffer[ip..chunk.buffer.len()]
            )?;
            match instruction {
                Instruction::Return(from) => {
                    break frame.get_value(from)?;
                }
                Instruction::Add { left, right, dst } => {
                    let left = frame.get_value(left)?;
                    let right = frame.get_value(right)?;
                    frame.set_value(dst, left.try_add(&right)?)?;
                }
                Instruction::Sub { left, right, dst } => {
                    let left = frame.get_value(left)?;
                    let right = frame.get_value(right)?;
                    frame.set_value(dst, left.try_sub(&right)?)?;
                }
                Instruction::Const(address, dst) => {
                    println!("{address} {dst}");
                    let const_val = chunk.consts
                        .get(address as usize)
                        .ok_or(RuntimeError::InvalidConstant(address))?;
                    frame.set_value(dst, *const_val)?;
                }
            }
            ip += offset as usize;
        };

        Ok(return_value)
    }
}

#[cfg(test)]
mod test {
    use std::{ thread::Thread, ptr::{ null, NonNull }, marker::PhantomData };

    use libimmixcons::{ immix_init, immix_noop_callback, object::Gc };

    use crate::{
        chunk::Chunk,
        stack::{ RegisterContents, Stack },
        alloc::GuardedCell,
        codegen::Codegen,
    };

    #[test]
    fn test_file() {
        immix_init(512 * 1000, 0, immix_noop_callback, core::ptr::null_mut());
        let mut codegen = Codegen::new();

        let left = codegen.emit_const(RegisterContents::Int(5));
        let right = codegen.emit_const(RegisterContents::Int(3));

        let result = codegen.emit_add(left, right);
        codegen.emit_return(result);

        let chunk = codegen.chunk();

        let mut thread = super::Thread {
            stack: Stack::new(),
        };

        let result = thread.run_chunk(chunk).unwrap();
        println!("{result}")
    }

    #[test]
    fn test_display() {
        let values = [
            RegisterContents::Int(5),
            RegisterContents::Float(3.445),
            RegisterContents::Bool(true),
            RegisterContents::Char('o'),
            RegisterContents::Object(
                GuardedCell::new(Gc {
                    //SAFETY: dont access this value, only print its pointer address
                    ptr: unsafe {
                        NonNull::new_unchecked(0 as *mut _)
                    },
                    marker: PhantomData,
                })
            ),
            RegisterContents::None,
        ];

        for value in values {
            println!("{value}\n");
        }
    }
}
