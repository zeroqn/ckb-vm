#![cfg_attr(not(feature = "std"), no_std)]

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::cmp;
        use std::fmt;
        use std::ops;
        use std::rc;
        use std::ptr;
        use std::marker;
        use std::vec;
        use std::boxed;
        use std::string;
    } else {
        use core::cmp;
        use core::fmt;
        use core::ops;
        use alloc::rc;
        use core::ptr;
        use core::marker;
        use alloc::boxed;
        use alloc::string;
        #[macro_use]
        extern crate alloc;
        use alloc::vec;
    }
}

#[macro_use]
extern crate derive_more;

pub mod bits;
pub mod debugger;
pub mod decoder;
pub mod error;
pub mod instructions;
pub mod machine;
pub mod memory;
#[cfg(feature = "std")]
pub mod snapshot;
pub mod syscalls;

pub use bytes;
pub use ckb_vm_definitions;

pub use crate::{
    debugger::Debugger,
    instructions::{Instruction, Register},
    machine::{
        trace::TraceMachine, CoreMachine, DefaultCoreMachine, DefaultMachine,
        DefaultMachineBuilder, InstructionCycleFunc, Machine, SupportMachine,
    },
    memory::{sparse::SparseMemory, wxorx::WXorXMemory, Memory},
    syscalls::Syscalls,
};
pub use bytes::Bytes;
#[cfg(feature = "std")]
pub use memory::flat::FlatMemory;

pub use ckb_vm_definitions::{
    registers, DEFAULT_STACK_SIZE, ISA_B, ISA_IMC, ISA_MOP, MEMORY_FRAMES, MEMORY_FRAMESIZE,
    MEMORY_FRAME_SHIFTS, RISCV_GENERAL_REGISTER_NUMBER, RISCV_MAX_MEMORY, RISCV_PAGES,
    RISCV_PAGESIZE, RISCV_PAGE_SHIFTS,
};

pub use error::Error;

pub fn run<R: Register, M: Memory<REG = R>>(
    program: &Bytes,
    args: &[Bytes],
    memory_size: usize,
) -> Result<i8, Error> {
    let core_machine = DefaultCoreMachine::<R, WXorXMemory<M>>::new(
        ISA_IMC | ISA_B | ISA_MOP,
        machine::VERSION1,
        u64::max_value(),
        memory_size,
    );
    let mut machine = TraceMachine::new(DefaultMachineBuilder::new(core_machine).build());
    machine.load_program(program, args)?;
    machine.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_memory_must_be_multiple_of_pages() {
        assert_eq!(RISCV_MAX_MEMORY % RISCV_PAGESIZE, 0);
    }

    #[test]
    fn test_page_size_be_power_of_2() {
        assert!(RISCV_PAGESIZE.is_power_of_two());
    }
}
