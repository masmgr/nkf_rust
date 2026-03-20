use super::*;

#[test]
fn test_display_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = NkfError::Io(io_err);
    let msg = err.to_string();
    assert!(msg.contains("I/O error:"));
    assert!(msg.contains("file not found"));
}

#[test]
fn test_display_conversion_error() {
    let err = NkfError::Conversion("bad data".into());
    assert_eq!(err.to_string(), "Encoding conversion error: bad data");
}

#[test]
fn test_display_unsupported_encoding() {
    let err = NkfError::UnsupportedEncoding("EBCDIC".into());
    assert_eq!(err.to_string(), "Unsupported encoding: EBCDIC");
}

#[test]
fn test_display_invalid_mime() {
    let err = NkfError::InvalidMime("broken".into());
    assert_eq!(err.to_string(), "Invalid MIME encoding: broken");
}

#[test]
fn test_display_invalid_args() {
    let err = NkfError::InvalidArgs("--bad".into());
    assert_eq!(err.to_string(), "Invalid arguments: --bad");
}

#[test]
fn test_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let nkf_err = NkfError::from(io_err);
    assert!(matches!(nkf_err, NkfError::Io(_)));
    assert!(nkf_err.to_string().contains("access denied"));
}

#[test]
fn test_error_is_std_error() {
    let err = NkfError::Conversion("test".into());
    let dyn_err: &dyn std::error::Error = &err;
    // Verify it implements std::error::Error
    assert!(dyn_err.source().is_none());
    assert!(!dyn_err.to_string().is_empty());
}
