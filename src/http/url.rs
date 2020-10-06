use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug)]
pub struct URL {
    pub path: String,
}

impl URL {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl Default for URL {
    fn default() -> Self {
        URL::new("/")
    }
}

impl FromStr for URL {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("/") {
            Err(Error::new(ErrorKind::Other, "Not a valid path"))
        } else {
            Ok(URL::new(s))
        }
    }
}
