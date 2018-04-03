extern crate notify;
extern crate toml;


use std::io;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Errors {
    IO(io::Error),
    BadFile(toml::de::Error),
    NoRules,
    WatchErr(notify::Error)
}

impl From<notify::Error> for Errors {
    fn from(err: notify::Error) -> Errors {
        Errors::WatchErr(err)
    }
}
impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Errors {
        Errors::IO(err)
    }
}
impl From<toml::de::Error> for Errors {
    fn from(err: toml::de::Error) -> Errors {
        Errors::BadFile(err)
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            Errors::IO(ref err) => write!(f, "IO error: {}", err),
            Errors::WatchErr(ref err) => write!(f, "Watch error: {}", err),
            Errors::BadFile(ref err) => write!(f, "Config error: {}", err),
            Errors::NoRules => write!(f, "No rules created"),
        }
    }
}
impl error::Error for Errors {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            Errors::IO(ref err) => err.description(),
            Errors::WatchErr(ref err) => error::Error::description(err),
            Errors::BadFile(ref err) => error::Error::description(err),
            Errors::NoRules => "No rules created",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Errors::IO(ref err) => Some(err),
            Errors::WatchErr(ref err) => Some(err),
            Errors::BadFile(ref err) => Some(err),
            Errors::NoRules => None
        }
    }

}
