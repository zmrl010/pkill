use std::{
    error,
    fmt::{self, Formatter},
};

use sysinfo::Pid;

#[derive(Debug)]
pub enum Error {
    NotKilled(Pid),
    Unknown,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Unknown => None,
            Self::NotKilled(_) => None,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unknown => write!(f, "unknown pkill error"),
            Self::NotKilled(process) => write!(f, "process `{}` was not killed", process),
        }
    }
}
