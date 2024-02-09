//! Things shared by the different tools of this crate.

use std::{error, fmt};

#[derive(Debug)]
pub struct DumbError {
    message: String,
}
impl error::Error for DumbError {}
impl fmt::Display for DumbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
// impl Into<DumbError> for String {
//     fn into(self) -> DumbError {
//         DumbError { message: self }
//     }
// }
// impl Into<DumbError> for &str {
//   fn into(self) -> DumbError {
//       DumbError { message: self.to_string() }
//   }
// }
impl From<String> for DumbError {
    fn from(s: String) -> DumbError {
        DumbError { message: s }
    }
}
impl From<&str> for DumbError {
    fn from(s: &str) -> DumbError {
        DumbError {
            message: s.to_string(),
        }
    }
}
