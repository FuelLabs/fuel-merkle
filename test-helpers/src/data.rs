pub mod binary;
pub mod sparse;

mod encoded_value;
mod test_error;

pub use encoded_value::{EncodedValue, ENCODING_HEX, ENCODING_UTF8};
pub use test_error::TestError;
