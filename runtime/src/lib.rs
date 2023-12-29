#![feature(non_null_convenience)]

use std::{ marker::PhantomData, ptr::NonNull };

use alloc::{ Guard, GuardedHeap };
use chunk::{ Chunk, Instruction };
use libimmixcons::{
    threading::{ immix_register_thread, immix_unregister_thread },
    object::{ GCRTTI, RawGc },
    immix_collect,
};
use stack::{ Stack, StackValue };
use thiserror::Error;
use array::ArrayCapacity;

pub mod constants;
mod chunk;
pub mod stack;
mod array;
mod alloc;

#[derive(Error, Debug)]
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
}

pub(crate) type Result<T> = std::result::Result<T, RuntimeError>;

pub struct Thread {
    stack: Stack,
}

impl Thread {
    pub fn run_chunk(&mut self, chunk: Chunk) -> Result<StackValue> {
        let value;
        {
            immix_register_thread();
            let guard = Guard(PhantomData);
            value = self.run_guarded(guard, chunk)?;
            immix_unregister_thread();
        }
        immix_collect(true);
        Ok(value)
    }

    fn run_guarded(&mut self, guard: Guard, chunk: Chunk) -> Result<StackValue> {
        let mut ip = 0;
        let mut frame = self.stack.push_frame();
        let return_value = loop {
            if ip == chunk.len() {
                break StackValue::None;
            }
            let (instruction, offset) = Instruction::read_from_chunk(&chunk[ip..])?;
            match instruction {
                Instruction::Return(from) => {
                    break frame.get_value(from)?;
                }
                Instruction::Add { left, right, dst } => {
                    let left = frame.get_value(left)?;
                    let right = frame.get_value(right)?;
                    frame.set_value(dst, left.try_add(&right)?)?;
                }
                Instruction::Sub { left, right, dst } => todo!(),
            }
            ip += offset as usize;
        };

        Ok(return_value)
    }
}
