use std::result;

use crate::error::Error;

/// pkill library result type that defaults to self error type
pub type Result<T, E = Error> = result::Result<T, E>;
