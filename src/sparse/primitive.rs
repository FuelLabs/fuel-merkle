use crate::{
    common::{error::DeserializeError, Prefix},
    sparse::Node,
};

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
pub type Primitive = [u8; 69];

impl From<&Node> for Primitive {
    fn from(node: &Node) -> Self {
        let mut primitive = [0u8; 69];
        primitive[0..4].copy_from_slice(&node.height().to_be_bytes());
        primitive[4] = node.prefix() as u8;
        primitive[5..37].copy_from_slice(node.bytes_lo());
        primitive[37..69].copy_from_slice(node.bytes_hi());
        primitive
    }
}

impl TryFrom<&Primitive> for Node {
    type Error = DeserializeError;

    fn try_from(primitive: &Primitive) -> Result<Self, Self::Error> {
        let mut height_array = [0u8; 4];
        height_array.copy_from_slice(&primitive[0..4]);
        let height = u32::from_be_bytes(height_array);
        let prefix = Prefix::try_from(primitive[4])?;
        let mut bytes_lo = [0u8; 32];
        bytes_lo.copy_from_slice(&primitive[5..37]);
        let mut bytes_hi = [0u8; 32];
        bytes_hi.copy_from_slice(&primitive[37..69]);
        let node = Self::new(height, prefix, bytes_lo, bytes_hi);
        Ok(node)
    }
}
