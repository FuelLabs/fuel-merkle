use crate::common::{Bytes32, Position};
use core::mem::size_of;

const BUFFER_SIZE: usize = size_of::<Position>() + size_of::<Bytes32>();
pub type Buffer = [u8; BUFFER_SIZE];
pub const DEFAULT_BUFFER: &Buffer = &[0; BUFFER_SIZE];

struct Schema {}

impl Schema {
    const fn position_offset() -> isize {
        0
    }

    const fn hash_offset() -> isize {
        Self::position_offset() + size_of::<Position>() as isize
    }
}

pub struct ReadView<'a> {
    buffer: &'a Buffer,
}

impl<'a> ReadView<'a> {
    fn buffer(&self) -> &Buffer {
        self.buffer
    }

    unsafe fn position_ptr(&self) -> *const Position {
        let offset = Schema::position_offset();
        let position = self.buffer().as_ptr().offset(offset) as *const Position;
        position
    }

    unsafe fn hash_ptr(&self) -> *const Bytes32 {
        let offset = Schema::hash_offset();
        let hash = self.buffer().as_ptr().offset(offset) as *const Bytes32;
        hash
    }

    pub fn new(buffer: &'a Buffer) -> Self {
        Self { buffer }
    }

    pub fn position(&self) -> Position {
        // SAFETY: position_ptr is guaranteed to point to a valid Position
        unsafe { *self.position_ptr() }
    }

    pub fn hash(&self) -> &Bytes32 {
        // SAFETY: hash_ptr is guaranteed to point to a valid Bytes32
        unsafe { &*self.hash_ptr() }
    }
}

pub struct WriteView<'a> {
    buffer: &'a mut Buffer,
}

impl<'a> WriteView<'a> {
    fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }

    unsafe fn position_mut_ptr(&mut self) -> *mut Position {
        let offset = Schema::position_offset();
        let position = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Position;
        position
    }

    unsafe fn hash_mut_ptr(&mut self) -> *mut Bytes32 {
        let offset = Schema::hash_offset();
        let hash = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Bytes32;
        hash
    }

    pub fn new(buffer: &'a mut Buffer) -> Self {
        Self { buffer }
    }

    pub fn position_mut(&mut self) -> &mut Position {
        // SAFETY: position_mut_ptr is guaranteed to point to a valid Position
        unsafe { &mut *self.position_mut_ptr() }
    }

    pub fn hash_mut(&mut self) -> &mut Bytes32 {
        // SAFETY: hash_mut_ptr is guaranteed to point to a valid Bytes32
        unsafe { &mut *self.hash_mut_ptr() }
    }
}
