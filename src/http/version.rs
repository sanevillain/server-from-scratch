use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug)]
pub enum Version {
    V1P1,
}

impl Default for Version {
    fn default() -> Self {
        Version::V1P1
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Version::V1P1),
            _ => Err(Error::new(ErrorKind::Other, "HTTP version not supported")),
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Version::V1P1 => String::from("HTTP/1.1"),
        }
    }
}
