use super::common::{header::Header, Version, URL};
use std::{default::Default, error::Error, fmt, str::FromStr};

#[derive(Debug)]
pub struct InvalidHttpRequestError();

impl Error for InvalidHttpRequestError {}

impl fmt::Display for InvalidHttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Http Request!")
    }
}

#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

impl FromStr for Method {
    type Err = InvalidHttpRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(InvalidHttpRequestError()),
        }
    }
}

#[derive(Debug, Default)]
pub struct Request {
    pub method: Method,
    pub url: URL,
    pub http_version: Version,
    pub header: Header,
}

impl Request {
    pub fn new() -> Self {
        Default::default()
    }
}

impl FromStr for Request {
    type Err = InvalidHttpRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut request: Request = Default::default();
        request.header = Header::from_str(s).unwrap();

        let lines: Vec<_> = s.split("\r\n").collect();
        let first_line: Vec<_> = lines
            .get(0)
            .ok_or(InvalidHttpRequestError())?
            .split(" ")
            .collect();

        if first_line.len() < 3 {
            return Err(InvalidHttpRequestError());
        }

        let (method, url, http_version) = (
            *first_line.get(0).ok_or(InvalidHttpRequestError())?,
            *first_line.get(1).ok_or(InvalidHttpRequestError())?,
            *first_line.get(2).ok_or(InvalidHttpRequestError())?,
        );

        request.method = Method::from_str(method)?;
        request.url = URL::from_str(url).map_err(|_| InvalidHttpRequestError())?;
        request.http_version =
            Version::from_str(http_version).map_err(|_| InvalidHttpRequestError())?;

        Ok(request)
    }
}