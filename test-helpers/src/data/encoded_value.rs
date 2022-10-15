use serde::{Deserialize, Serialize};

use crate::data::TestError;

// Supported value encodings:
pub const ENCODING_BASE_64: &str = "base64";
pub const ENCODING_HEX: &str = "hex";
pub const ENCODING_UTF8: &str = "utf-8";

#[derive(Serialize, Deserialize, Clone)]
pub struct EncodedValue {
    value: String,
    encoding: String,
}

enum Encoding {
    Base64,
    Hex,
    Utf8,
}

impl EncodedValue {
    pub fn new(value: String, encoding: &str) -> Self {
        Self {
            value,
            encoding: encoding.to_string(),
        }
    }

    pub fn from_raw<T: AsRef<[u8]>>(value: T, encoding: &str) -> Result<Self, TestError> {
        let encoded_value = match Self::encoding_type(encoding)? {
            Encoding::Base64 => base64::encode(value),
            Encoding::Hex => hex::encode(value),
            Encoding::Utf8 => String::from_utf8_lossy(value.as_ref()).to_string(),
        };
        Ok(Self {
            value: encoded_value,
            encoding: encoding.to_string(),
        })
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, TestError> {
        match Self::encoding_type(&self.encoding)? {
            Encoding::Base64 => Ok(base64::decode(self.value).unwrap()),
            Encoding::Hex => Ok(hex::decode(self.value).unwrap()),
            Encoding::Utf8 => Ok(self.value.into_bytes()),
        }
    }

    // Translate the encoding string found in the value definition to an Encoding enum variant.
    fn encoding_type(encoding: &str) -> Result<Encoding, TestError> {
        match encoding {
            ENCODING_BASE_64 => Ok(Encoding::Base64),
            ENCODING_HEX => Ok(Encoding::Hex),
            ENCODING_UTF8 => Ok(Encoding::Utf8),

            // Unsupported encoding
            _ => Err(TestError::UnsupportedEncoding(encoding.to_string())),
        }
    }
}
