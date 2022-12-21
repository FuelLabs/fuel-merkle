mod internal {
    use crate::common::{Bytes32, Position};
    use core::mem::size_of;

    const BUFFER_SIZE: usize = size_of::<Position>() + size_of::<Bytes32>();
    pub type Buffer = [u8; BUFFER_SIZE];
    pub const DEFAULT_BUFFER: Buffer = [0; BUFFER_SIZE];
}

use crate::common::{Bytes32, Position};
use core::mem::size_of;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Buffer(internal::Buffer);

impl Buffer {
    const fn position_offset() -> isize {
        0
    }

    const fn hash_offset() -> isize {
        Self::position_offset() + size_of::<Position>() as isize
    }

    fn data(&self) -> &internal::Buffer {
        &self.0
    }

    fn data_mut(&mut self) -> &mut internal::Buffer {
        &mut self.0
    }

    unsafe fn position_ptr(&self) -> *const Position {
        let offset = Self::position_offset();
        let position = self.data().as_ptr().offset(offset) as *const Position;
        position
    }

    unsafe fn position_mut_ptr(&mut self) -> *mut Position {
        let offset = Self::position_offset();
        let position = self.data_mut().as_mut_ptr().offset(offset) as *mut Position;
        position
    }

    unsafe fn hash_ptr(&self) -> *const Bytes32 {
        let offset = Self::hash_offset();
        let hash = self.data().as_ptr().offset(offset) as *const Bytes32;
        hash
    }

    unsafe fn hash_mut_ptr(&mut self) -> *mut Bytes32 {
        let offset = Self::hash_offset();
        let hash = self.data_mut().as_mut_ptr().offset(offset) as *mut Bytes32;
        hash
    }

    pub fn position(&self) -> Position {
        unsafe { *self.position_ptr() }
    }

    pub fn position_mut(&mut self) -> &mut Position {
        unsafe { &mut *self.position_mut_ptr() }
    }

    pub fn hash(&self) -> &Bytes32 {
        unsafe { &*self.hash_ptr() }
    }

    pub fn hash_mut(&mut self) -> &mut Bytes32 {
        unsafe { &mut *self.hash_mut_ptr() }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self(internal::DEFAULT_BUFFER.clone())
    }
}

impl<'a, 'b> From<&'a Buffer> for &'b [u8]
where
    'a: 'b,
{
    fn from(buffer: &'a Buffer) -> Self {
        buffer.data()
    }
}
