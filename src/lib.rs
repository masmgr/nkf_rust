pub mod cli;
pub mod convert;
pub mod detect;
pub mod encoding_type;
pub mod error;
pub mod fold;
pub mod input_decode;
pub mod kana;
pub mod line_ending;
pub mod mime;
pub mod pipeline;

pub use convert::convert;
pub use detect::{DetectionResult, detect};
pub use encoding_type::EncodingType;
pub use error::NkfError;
pub use pipeline::{NkfOptions, process};
