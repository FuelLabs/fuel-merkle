use crate::common::{Bytes1, Bytes32, Bytes4, Prefix};

use core::mem::size_of;

/// **Leaf buffer:**
///
/// | Allocation | Data                       |
/// |------------|----------------------------|
/// | `00 - 04`  | Height (4 bytes)           |
/// | `04 - 05`  | Prefix (1 byte, `0x00`)    |
/// | `05 - 37`  | hash(Key) (32 bytes)       |
/// | `37 - 69`  | hash(Data) (32 bytes)      |
///
/// **Node buffer:**
///
/// | Allocation | Data                       |
/// |------------|----------------------------|
/// | `00 - 04`  | Height (4 bytes)           |
/// | `04 - 05`  | Prefix (1 byte, `0x01`)    |
/// | `05 - 37`  | Left child key (32 bytes)  |
/// | `37 - 69`  | Right child key (32 bytes) |
///
pub const DATA_SIZE: usize = size_of::<Prefix>() + size_of::<Bytes32>() + size_of::<Bytes32>();
pub type Data = [u8; DATA_SIZE];
pub const BUFFER_SIZE: usize = size_of::<u32>() + size_of::<Data>();
pub type Buffer = [u8; BUFFER_SIZE];
pub const DEFAULT_BUFFER: &Buffer = &[0; BUFFER_SIZE];

pub struct Schema {}

impl Schema {
    const fn bytes_height_offset() -> isize {
        0
    }

    const fn bytes_prefix_offset() -> isize {
        Self::bytes_height_offset() + size_of::<u32>() as isize
    }

    const fn bytes_lo_offset() -> isize {
        Self::bytes_prefix_offset() + size_of::<Prefix>() as isize
    }

    const fn bytes_hi_offset() -> isize {
        Self::bytes_lo_offset() + size_of::<Bytes32>() as isize
    }

    const fn bytes_hash_offset() -> isize {
        Self::bytes_prefix_offset()
    }
}

pub struct ReadView<'a> {
    buffer: &'a Buffer,
}

impl<'a> ReadView<'a> {
    fn buffer(&self) -> &Buffer {
        self.buffer
    }

    unsafe fn bytes_height_ptr(&self) -> *const Bytes4 {
        let offset = Schema::bytes_height_offset();
        let height = self.buffer().as_ptr().offset(offset) as *const Bytes4;
        height
    }

    unsafe fn bytes_prefix_ptr(&self) -> *const Bytes1 {
        let offset = Schema::bytes_prefix_offset();
        let bytes_prefix = self.buffer().as_ptr().offset(offset) as *const Bytes1;
        bytes_prefix
    }

    unsafe fn bytes_lo_ptr(&self) -> *const Bytes32 {
        let offset = Schema::bytes_lo_offset();
        let bytes_lo = self.buffer().as_ptr().offset(offset) as *const Bytes32;
        bytes_lo
    }

    unsafe fn bytes_hi_ptr(&self) -> *const Bytes32 {
        let offset = Schema::bytes_hi_offset();
        let bytes_hi = self.buffer().as_ptr().offset(offset) as *const Bytes32;
        bytes_hi
    }

    unsafe fn bytes_hash_ptr(&self) -> *const Data {
        let offset = Schema::bytes_hash_offset();
        let bytes_hash = self.buffer().as_ptr().offset(offset) as *const [u8; 65];
        bytes_hash
    }

    pub fn new(buffer: &'a Buffer) -> Self {
        Self { buffer }
    }

    pub fn bytes_height(&self) -> &Bytes4 {
        unsafe { &*self.bytes_height_ptr() }
    }

    pub fn bytes_prefix(&self) -> &Bytes1 {
        unsafe { &*self.bytes_prefix_ptr() }
    }

    pub fn bytes_lo(&self) -> &Bytes32 {
        unsafe { &*self.bytes_lo_ptr() }
    }

    pub fn bytes_hi(&self) -> &Bytes32 {
        unsafe { &*self.bytes_hi_ptr() }
    }

    pub fn bytes_hash(&self) -> &Data {
        unsafe { &*self.bytes_hash_ptr() }
    }
}

pub struct WriteView<'a> {
    buffer: &'a mut Buffer,
}

impl<'a> WriteView<'a> {
    fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }

    unsafe fn bytes_height_mut_ptr(&mut self) -> *mut Bytes4 {
        let offset = Schema::bytes_height_offset();
        let bytes_height = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Bytes4;
        bytes_height
    }

    unsafe fn bytes_prefix_mut_ptr(&mut self) -> *mut Bytes1 {
        let offset = Schema::bytes_prefix_offset();
        let bytes_prefix = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Bytes1;
        bytes_prefix
    }

    unsafe fn bytes_lo_mut_ptr(&mut self) -> *mut Bytes32 {
        let offset = Schema::bytes_lo_offset();
        let bytes_lo = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Bytes32;
        bytes_lo
    }

    unsafe fn bytes_hi_mut_ptr(&mut self) -> *mut Bytes32 {
        let offset = Schema::bytes_hi_offset();
        let bytes_hi = self.buffer_mut().as_mut_ptr().offset(offset) as *mut Bytes32;
        bytes_hi
    }

    pub fn new(buffer: &'a mut Buffer) -> Self {
        Self { buffer }
    }

    pub fn bytes_height_mut(&mut self) -> &mut Bytes4 {
        unsafe { &mut *self.bytes_height_mut_ptr() }
    }

    pub fn bytes_prefix_mut(&mut self) -> &mut Bytes1 {
        unsafe { &mut *self.bytes_prefix_mut_ptr() }
    }

    pub fn bytes_lo_mut(&mut self) -> &mut Bytes32 {
        unsafe { &mut *self.bytes_lo_mut_ptr() }
    }

    pub fn bytes_hi_mut(&mut self) -> &mut Bytes32 {
        unsafe { &mut *self.bytes_hi_mut_ptr() }
    }
}
