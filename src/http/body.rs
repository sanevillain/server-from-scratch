use std::{io::Error, str::FromStr};

#[derive(Debug)]
pub struct Body {
    inner: Vec<u8>,
}

impl Body {
    pub fn new(inner: Vec<u8>) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> Vec<u8> {
        self.inner.clone()
    }
}

impl Default for Body {
    fn default() -> Self {
        Body::new(vec![])
    }
}

impl FromStr for Body {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Body::new(s.to_string().into_bytes()))
    }
}
