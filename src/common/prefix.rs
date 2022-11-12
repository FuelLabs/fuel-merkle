const INTERNAL: u8 = 0x01;
const LEAF: u8 = 0x00;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prefix {
    INTERNAL,
    LEAF,
}

impl From<Prefix> for u8 {
    fn from(prefix: Prefix) -> Self {
        match prefix {
            Prefix::INTERNAL => INTERNAL,
            Prefix::LEAF => LEAF,
        }
    }
}

impl From<u8> for Prefix {
    fn from(byte: u8) -> Self {
        match byte {
            INTERNAL => Prefix::INTERNAL,
            LEAF => Prefix::LEAF,
            _ => unreachable!(),
        }
    }
}
