const INTERNAL: u8 = 0x01;
const LEAF: u8 = 0x00;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prefix {
    Internal,
    Leaf,
}

impl From<Prefix> for u8 {
    fn from(prefix: Prefix) -> Self {
        match prefix {
            Prefix::Internal => INTERNAL,
            Prefix::Leaf => LEAF,
        }
    }
}

impl AsRef<[u8]> for Prefix {
    fn as_ref(&self) -> &[u8] {
        match self {
            Prefix::Internal => &[INTERNAL],
            Prefix::Leaf => &[LEAF],
        }
    }
}

impl AsRef<[u8; 1]> for Prefix {
    fn as_ref(&self) -> &[u8; 1] {
        match self {
            Prefix::Internal => &[INTERNAL],
            Prefix::Leaf => &[LEAF],
        }
    }
}

impl From<u8> for Prefix {
    fn from(byte: u8) -> Self {
        match byte {
            INTERNAL => Prefix::Internal,
            LEAF => Prefix::Leaf,
            _ => unreachable!(),
        }
    }
}
