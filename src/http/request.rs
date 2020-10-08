use super::{body::Body, header::Header, method::Method, url::URL, version::Version};
use std::{default::Default, error::Error, fmt, str::FromStr};

#[derive(Debug)]
pub struct InvalidHttpRequestError();

impl Error for InvalidHttpRequestError {}

impl fmt::Display for InvalidHttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Http Request!")
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
        let lines = header_lines
            .iter()
            .skip(1)
            .map(|s| *s)
            .collect::<Vec<&str>>();
        Header::from_lines(lines).map_err(|_| InvalidHttpRequestError())
    }

    fn create_method(method: &str) -> Result<Method, InvalidHttpRequestError> {
        Method::from_str(method).map_err(|_| InvalidHttpRequestError())
    }

    fn create_url(url: &str) -> Result<URL, InvalidHttpRequestError> {
        URL::from_str(url).map_err(|_| InvalidHttpRequestError())
    }

    fn create_http_version(http_version: &str) -> Result<Version, InvalidHttpRequestError> {
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

        let headers = headers_and_body.first().ok_or(InvalidHttpRequestError())?;
        let body = headers_and_body.last().ok_or(InvalidHttpRequestError())?;

        let header_lines = headers.split("\r\n").collect::<Vec<&str>>();
        let fst_header_line = header_lines
            .first()
            .ok_or(InvalidHttpRequestError())?
            .split(" ")
            .collect::<Vec<&str>>();

        if fst_header_line.len() < 3 {
            return Err(InvalidHttpRequestError());
        }

        let (method_str, url_str, http_version_str) =
            (fst_header_line[0], fst_header_line[1], fst_header_line[2]);

        let method = Request::create_method(method_str)?;
        let url = Request::create_url(url_str)?;
        let http_version = Request::create_http_version(http_version_str)?;
        let header = Request::create_header(header_lines)?;
        let body = Request::create_body(body)?;

        let req = Request::new(method, url, http_version, header, body);
        Ok(req)
    }
}
