mod sync;
pub use self::sync::sync;

use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct SyncError(SyncErrorKind);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum SyncErrorKind {
    /// The Sync or Cursor token was invalid, either missing the version or timestamp.
    InvalidToken,

    /// Failure to parse token, may not meet expectations
    ParseErrorToken
}

impl Error for SyncError {
    fn description(&self) -> &str {
        match self.0 {
            SyncErrorKind::InvalidToken => "Sync or cursor token was invalid.",
            SyncErrorKind::ParseErrorToken => "Unable to parse timestamp for sync or cursor token.",
        }
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

type SyncResult<T> = Result<T, SyncError>;
