use crate::common::{Bytes32, Position};
use core::mem::size_of;

const BUFFER_SIZE: usize = size_of::<Position>() + size_of::<Bytes32>();
pub type Buffer = [u8; BUFFER_SIZE];
pub const DEFAULT_BUFFER: &Buffer = &[0; BUFFER_SIZE];

const POSITION_OFFSET: usize = 0;
const HASH_OFFSET: usize = POSITION_OFFSET + size_of::<Position>();

pub struct ReadView<'a> {
    buffer: &'a Buffer,
}

impl<'a> ReadView<'a> {
    fn buffer(&self) -> &Buffer {
        self.buffer
    }

    unsafe fn position_ptr(&self) -> *const Position {
        let offset = POSITION_OFFSET as isize;
        let position = self.buffer().as_ptr().offset(offset) as *const Position;
        position
    }

    unsafe fn hash_ptr(&self) -> *const Bytes32 {
        let offset = HASH_OFFSET as isize;
        let hash = self.buffer().as_ptr().offset(offset) as *const Bytes32;
        hash
    }

    pub fn new(buffer: &'a Buffer) -> Self {
        Self { buffer }
    }

    pub fn position(&self) -> Position {
        // SAFETY: position_ptr is guaranteed to point to a valid Position.
        //         Note that the returned Position is copied from the Position
        //         data in the buffer.
        unsafe { *self.position_ptr() }
    }

    pub fn hash(&self) -> &Bytes32 {
        // SAFETY: hash_ptr is guaranteed to point to a valid Bytes32.
        //         Note that the returned &Bytes32 is a direct reference to
        //         immutable hash data in the buffer.
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
        let offset = POSITION_OFFSET as isize;
        let position = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Position;
        position
    }

    unsafe fn hash_mut_ptr(&mut self) -> *mut Bytes32 {
        let offset = HASH_OFFSET as isize;
        let hash = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Bytes32;
        hash
    }

    pub fn new(buffer: &'a mut Buffer) -> Self {
        Self { buffer }
    }

    pub fn position_mut(&mut self) -> &mut Position {
        // SAFETY: position_mut_ptr is guaranteed to point to a valid Position.
        //         Note that the returned &Position is a direct reference to
        //         mutable Position data in the buffer.
        unsafe { &mut *self.position_mut_ptr() }
    }

    pub fn hash_mut(&mut self) -> &mut Bytes32 {
        // SAFETY: hash_mut_ptr is guaranteed to point to a valid Bytes32.
        //         Note that the returned &Bytes32 is a direct reference to
        //         mutable hash data in the buffer.
        unsafe { &mut *self.hash_mut_ptr() }
    }
}
