pub mod decode;
pub mod execution;
mod instruction;

use std::ops::Deref;

use crate::bus::dram;
use crate::bus::dram::Dram;

pub struct CPU {
    pub pc: usize,
    pub reg: [i32; 32],
}

impl CPU {
    pub fn new(entry_address: usize) -> CPU {
        CPU {
            pc: entry_address,
            reg: [0; 32],
        }
    }
}

pub struct Wrapper {
    value: Box<dyn Decode>,
}

impl Wrapper {
    fn new(value: Box<dyn Decode>) -> Self {
        Self { value }
    }
}

impl Deref for Decode {
    type Target = Box<dyn Decode>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

use crate::cpu::decode::Decode;

pub fn fetch(dram: &dram::Dram, index_pc: usize) -> Decode {
    // return instruction data

    let is_cinst: bool = dram.raw_byte(index_pc) & 0x3 != 0x3;

    if is_cinst {
        let value = (Dram::raw_byte(dram, index_pc + 4) as u32) << 24
            | (Dram::raw_byte(dram, index_pc + 3) as u32) << 16
            | (Dram::raw_byte(dram, index_pc + 2) as u32) << 8
            | (Dram::raw_byte(dram, index_pc + 1) as u32);
        Decode::new(Box::new(value))
    } else {
        let value = (Dram::raw_byte(dram, index_pc + 1) as u16) << 8
            | (Dram::raw_byte(dram, index_pc + 0) as u16);
        Decode::new(Box::new(value))
    }
}
