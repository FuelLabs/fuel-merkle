use serde::{Deserialize, Serialize};

use crate::data::TestError;

// Supported value encodings:
pub const ENCODING_HEX: &str = "hex";
pub const ENCODING_UTF8: &str = "utf-8";

#[derive(Serialize, Deserialize, Clone)]
pub struct EncodedValue {
    value: String,
    encoding: String,
}

enum Encoding {
    Hex,
    Utf8,
}

impl EncodedValue {
    pub fn new(value: String, encoding: String) -> Self {
        Self { value, encoding }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, TestError> {
        match self.encoding_type()? {
            Encoding::Hex => Ok(hex::decode(self.value).unwrap()),
            Encoding::Utf8 => Ok(self.value.into_bytes()),
        }
    }

    // Translate the encoding string found in the value definition to an Encoding enum variant.
    fn encoding_type(&self) -> Result<Encoding, TestError> {
        match self.encoding.as_str() {
            ENCODING_HEX => Ok(Encoding::Hex),
            ENCODING_UTF8 => Ok(Encoding::Utf8),

            // Unsupported encoding
            _ => Err(TestError::UnsupportedEncoding(self.encoding.clone())),
        }
    }
}
