use std::string::ToString;

pub enum Status {
    Continue,                      // 100 - RFC 7231, 6.2.1
    SwitchingProtocols,            // 101 - RFC 7231, 6.2.2
    Processing,                    // 102 - RFC 2518, 10.1
    EarlyHints,                    // 103 - RFC 8297
    OK,                            // 200 - RFC 7231, 6.3.1
    Created,                       // 201 - RFC 7231, 6.3.2
    Accepted,                      // 202 - RFC 7231, 6.3.3
    NonAuthoritativeInfo,          // 203 - RFC 7231, 6.3.4
    NoContent,                     // 204 - RFC 7231, 6.3.5
    ResetContent,                  // 205 - RFC 7231, 6.3.6
    PartialContent,                // 206 - RFC 7233, 4.1
    MultiStatus,                   // 207 - RFC 4918, 11.1
    AlreadyReported,               // 208 - RFC 5842, 7.1
    IMUsed,                        // 226 - RFC 3229, 10.4.1
    MultipleChoices,               // 300 - RFC 7231, 6.4.1
    MovedPermanently,              // 301 - RFC 7231, 6.4.2
    Found,                         // 302 - RFC 7231, 6.4.3
    SeeOther,                      // 303 - RFC 7231, 6.4.4
    NotModified,                   // 304 - RFC 7232, 4.1
    UseProxy,                      // 305 - RFC 7231, 6.4.5
    TemporaryRedirect,             // 307 - RFC 7231, 6.4.7
    PermanentRedirect,             // 308 - RFC 7538, 3
    BadRequest,                    // 400 - RFC 7231, 6.5.1
    Unauthorized,                  // 401 - RFC 7235, 3.1
    PaymentRequired,               // 402 - RFC 7231, 6.5.2
    Forbidden,                     // 403 - RFC 7231, 6.5.3
    NotFound,                      // 404 - RFC 7231, 6.5.4
    MethodNotAllowed,              // 405 - RFC 7231, 6.5.5
    NotAcceptable,                 // 406 - RFC 7231, 6.5.6
    ProxyAuthRequired,             // 407 - RFC 7235, 3.2
    RequestTimeout,                // 408 - RFC 7231, 6.5.7
    Conflict,                      // 409 - RFC 7231, 6.5.8
    Gone,                          // 410 - RFC 7231, 6.5.9
    LengthRequired,                // 411 - RFC 7231, 6.5.10
    PreconditionFailed,            // 412 - RFC 7232, 4.2
    RequestEntityTooLarge,         // 413 - RFC 7231, 6.5.11
    RequestURITooLong,             // 414 - RFC 7231, 6.5.12
    UnsupportedMediaType,          // 415 - RFC 7231, 6.5.13
    RequestedRangeNotSatisfiable,  // 416 - RFC 7233, 4.4
    ExpectationFailed,             // 417 - RFC 7231, 6.5.14
    Teapot,                        // 418 - RFC 7168, 2.3.3
    MisdirectedRequest,            // 421 - RFC 7540, 9.1.2
    UnprocessableEntity,           // 422 - RFC 4918, 11.2
    Locked,                        // 423 - RFC 4918, 11.3
    FailedDependency,              // 424 - RFC 4918, 11.4
    TooEarly,                      // 425 - RFC 8470, 5.2.
    UpgradeRequired,               // 426 - RFC 7231, 6.5.15
    PreconditionRequired,          // 428 - RFC 6585, 3
    TooManyRequests,               // 429 - RFC 6585, 4
    RequestHeaderFieldsTooLarge,   // 431 - RFC 6585, 5
    UnavailableForLegalReasons,    // 451 - RFC 7725, 3
    InternalServerError,           // 500 - RFC 7231, 6.6.1
    NotImplemented,                // 501 - RFC 7231, 6.6.2
    BadGateway,                    // 502 - RFC 7231, 6.6.3
    ServiceUnavailable,            // 503 - RFC 7231, 6.6.4
    GatewayTimeout,                // 504 - RFC 7231, 6.6.5
    HTTPVersionNotSupported,       // 505 - RFC 7231, 6.6.6
    VariantAlsoNegotiates,         // 506 - RFC 2295, 8.1
    InsufficientStorage,           // 507 - RFC 4918, 11.5
    LoopDetected,                  // 508 - RFC 5842, 7.2
    NotExtended,                   // 510 - RFC 2774, 7
    NetworkAuthenticationRequired, // 511 - RFC 6585, 6
}

impl Status {
    pub fn get_code(&self) -> u16 {
        self.get_code_and_string().0
    }

    pub fn get_string(&self) -> &'static str {
        self.get_code_and_string().1
    }

    pub fn get_code_and_string(&self) -> (u16, &'static str) {
        match self {
            Status::Continue => (100, "Continue"),
            Status::SwitchingProtocols => (101, "Switching Protocols"),
            Status::Processing => (102, "Processing"),
            Status::EarlyHints => (103, "Early Hints"),
            Status::OK => (200, "OK"),
            Status::Created => (201, "Created"),
            Status::Accepted => (202, "Accepted"),
            Status::NonAuthoritativeInfo => (203, "Non-Authoritative Information"),
            Status::NoContent => (204, "No Content"),
            Status::ResetContent => (205, "Reset Content"),
            Status::PartialContent => (206, "Partial Content"),
            Status::MultiStatus => (207, "Multi-Status"),
            Status::AlreadyReported => (208, "Already Reported"),
            Status::IMUsed => (226, "IM Used"),
            Status::MultipleChoices => (300, "Multiple Choices"),
            Status::MovedPermanently => (301, "Moved Permanently"),
            Status::Found => (302, "Found"),
            Status::SeeOther => (303, "See Other"),
            Status::NotModified => (304, "Not Modified"),
            Status::UseProxy => (305, "Use Proxy"),
            Status::TemporaryRedirect => (307, "Temporary Redirect"),
            Status::PermanentRedirect => (308, "Permanent Redirect"),
            Status::BadRequest => (400, "Bad Request"),
            Status::Unauthorized => (401, "Unauthorized"),
            Status::PaymentRequired => (402, "Payment Required"),
            Status::Forbidden => (403, "Forbidden"),
            Status::NotFound => (404, "Not Found"),
            Status::MethodNotAllowed => (405, "Method Not Allowed"),
            Status::NotAcceptable => (406, "Not Acceptable"),
            Status::ProxyAuthRequired => (407, "Proxy Authentication Required"),
            Status::RequestTimeout => (408, "Request Timeout"),
            Status::Conflict => (409, "Conflict"),
            Status::Gone => (410, "Gone"),
            Status::LengthRequired => (411, "Length Required"),
            Status::PreconditionFailed => (412, "Precondition Failed"),
            Status::RequestEntityTooLarge => (413, "Request Entity Too Large"),
            Status::RequestURITooLong => (414, "Request URI Too Long"),
            Status::UnsupportedMediaType => (415, "Unsupported Media Type"),
            Status::RequestedRangeNotSatisfiable => (416, "Requested Range Not Satisfiable"),
            Status::ExpectationFailed => (417, "Expectation Failed"),
            Status::Teapot => (418, "Teapot"),
            Status::MisdirectedRequest => (421, "Misdirected Request"),
            Status::UnprocessableEntity => (422, "Unprocessable Entity"),
            Status::Locked => (423, "Locked"),
            Status::FailedDependency => (424, "Failed Dependency"),
            Status::TooEarly => (425, "Too Early"),
            Status::UpgradeRequired => (426, "Upgrade Required"),
            Status::PreconditionRequired => (428, "Precondition Required"),
            Status::TooManyRequests => (429, "Too Many Requests"),
            Status::RequestHeaderFieldsTooLarge => (431, "Request Header Fields Too Large"),
            Status::UnavailableForLegalReasons => (451, "Unavailable For Legal Reasons"),
            Status::InternalServerError => (500, "Internal Server Error"),
            Status::NotImplemented => (501, "Not Implemented"),
            Status::BadGateway => (502, "Bad Gateway"),
            Status::ServiceUnavailable => (503, "Service Unavailable"),
            Status::GatewayTimeout => (504, "Gateway Timeout"),
            Status::HTTPVersionNotSupported => (505, "HTTP Version Not Supported"),
            Status::VariantAlsoNegotiates => (506, "Variant Also Negotiates"),
            Status::InsufficientStorage => (507, "Insufficient Storage"),
            Status::LoopDetected => (508, "Loop Detected"),
            Status::NotExtended => (510, "Not Extended"),
            Status::NetworkAuthenticationRequired => (511, "Network Authentication Required"),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        self.get_code_and_string().1.to_string()
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::OK
    }
}
