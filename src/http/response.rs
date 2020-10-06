use super::{header::Header, status::Status, version::Version};

#[derive(Default)]
pub struct Response {
    pub http_version: Version,
    pub status: Status,
    pub header: Header,
    pub body: Vec<u8>,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder(Default::default())
    }

    pub fn build_headers_string(&self) -> String {
        format!(
            "{} {} {}\r\n{}",
            self.http_version.to_string(),
            self.status.get_code(),
            self.status.to_string(),
            self.header.to_string(),
        )
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut res = self.build_headers_string().into_bytes();
        res.extend(self.body);
        res
    }
}

impl From<ResponseBuilder> for Response {
    fn from(rb: ResponseBuilder) -> Self {
        rb.0
    }
}

pub struct ResponseBuilder(Response);

impl ResponseBuilder {
    pub fn status(mut self, status: Status) -> Self {
        self.0.status = status;
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.0.body = body;
        self
    }

    pub fn header(mut self, key: &str, val: &str) -> Self {
        self.0.header.add(key, val);
        self
    }
}
