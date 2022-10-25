use crate::error::PKillError;

pub type Result<T, E = PKillError> = std::result::Result<T, E>;
