use crate::common::{Bit, Msb};

pub enum Instruction {
    LEFT,
    RIGHT,
}

impl From<Bit> for Instruction {
    fn from(bit: Bit) -> Self {
        match bit {
            Bit::_0 => Instruction::LEFT,
            Bit::_1 => Instruction::RIGHT,
        }
    }
}

pub trait Path {
    fn get_instruction(&self, index: usize) -> Option<Instruction>;
}

impl<T> Path for T
where
    T: Msb,
{
    fn get_instruction(&self, index: usize) -> Option<Instruction> {
        self.get_bit_at_index_from_msb(index).map(Into::into)
    }
}