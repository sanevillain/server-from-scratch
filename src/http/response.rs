use super::{
    common::{header::Header, Version},
    status::Status,
};

pub struct Response {
    pub http_version: Version,
    pub status_code: Status,
    pub header: Header,
}

impl Response {
    pub fn new() -> Self {
        let http_version = Version::default();
        let status_code = Status::default();
        let header = Header::new();

        Self {
            http_version,
            header,
            status_code,
        }
    }
}
