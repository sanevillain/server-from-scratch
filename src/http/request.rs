use super::{body::Body, header::Header, url::URL, version::Version};
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
    pub body: Body,
}

impl Request {
    pub fn new(
        method: Method,
        url: URL,
        http_version: Version,
        header: Header,
        body: Body,
    ) -> Self {
        Request {
            method,
            url,
            http_version,
            header,
            body,
        }
    }
}

impl Request {
    fn create_header(header_lines: Vec<&str>) -> Result<Header, InvalidHttpRequestError> {
        Header::from_lines(header_lines).map_err(|_| InvalidHttpRequestError())
    }

    fn create_method(first_header_line: &Vec<&str>) -> Result<Method, InvalidHttpRequestError> {
        let method = *first_header_line.get(0).ok_or(InvalidHttpRequestError())?;
        Method::from_str(method)
    }

    fn create_url(first_header_line: &Vec<&str>) -> Result<URL, InvalidHttpRequestError> {
        let url = *first_header_line.get(1).ok_or(InvalidHttpRequestError())?;
        URL::from_str(url).map_err(|_| InvalidHttpRequestError())
    }

    fn create_http_version(
        first_header_line: &Vec<&str>,
    ) -> Result<Version, InvalidHttpRequestError> {
        let http_version = *first_header_line.get(2).ok_or(InvalidHttpRequestError())?;
        Version::from_str(http_version).map_err(|_| InvalidHttpRequestError())
    }

    fn create_body(body: &str) -> Result<Body, InvalidHttpRequestError> {
        Body::from_str(body).map_err(|_| InvalidHttpRequestError())
    }
}

impl FromStr for Request {
    type Err = InvalidHttpRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let headers_and_body = s.split("\r\n\r\n").collect::<Vec<&str>>();

        let (headers, body) = (
            headers_and_body.first().ok_or(InvalidHttpRequestError())?,
            headers_and_body.last().ok_or(InvalidHttpRequestError())?,
        );

        let header_lines = headers.split("\r\n").collect::<Vec<&str>>();
        let first_header_line = header_lines
            .first()
            .ok_or(InvalidHttpRequestError())?
            .split(" ")
            .collect::<Vec<&str>>();

        let (method, url, http_version, header, body) = (
            Request::create_method(&first_header_line)?,
            Request::create_url(&first_header_line)?,
            Request::create_http_version(&first_header_line)?,
            Request::create_header(header_lines)?,
            Request::create_body(body)?,
        );

        let req = Request::new(method, url, http_version, header, body);

        Ok(req)
    }
}
