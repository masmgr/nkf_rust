pub mod cli;
pub mod convert;
pub mod detect;
pub mod encoding_type;
pub mod error;
pub mod kana;
pub mod line_ending;
pub mod mime;
pub mod pipeline;

pub use encoding_type::EncodingType;
pub use detect::{detect, DetectionResult};
pub use convert::convert;
pub use error::NkfError;
pub use pipeline::{NkfOptions, process};
