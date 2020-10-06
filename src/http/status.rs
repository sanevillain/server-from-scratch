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
        match self {
            Status::Continue => 100,
            Status::SwitchingProtocols => 101,
            Status::Processing => 102,
            Status::EarlyHints => 103,
            Status::OK => 200,
            Status::Created => 201,
            Status::Accepted => 202,
            Status::NonAuthoritativeInfo => 203,
            Status::NoContent => 204,
            Status::ResetContent => 205,
            Status::PartialContent => 206,
            Status::MultiStatus => 207,
            Status::AlreadyReported => 208,
            Status::IMUsed => 226,
            Status::MultipleChoices => 300,
            Status::MovedPermanently => 301,
            Status::Found => 302,
            Status::SeeOther => 303,
            Status::NotModified => 304,
            Status::UseProxy => 305,
            Status::TemporaryRedirect => 307,
            Status::PermanentRedirect => 308,
            Status::BadRequest => 400,
            Status::Unauthorized => 401,
            Status::PaymentRequired => 402,
            Status::Forbidden => 403,
            Status::NotFound => 404,
            Status::MethodNotAllowed => 405,
            Status::NotAcceptable => 406,
            Status::ProxyAuthRequired => 407,
            Status::RequestTimeout => 408,
            Status::Conflict => 409,
            Status::Gone => 410,
            Status::LengthRequired => 411,
            Status::PreconditionFailed => 412,
            Status::RequestEntityTooLarge => 413,
            Status::RequestURITooLong => 414,
            Status::UnsupportedMediaType => 415,
            Status::RequestedRangeNotSatisfiable => 416,
            Status::ExpectationFailed => 417,
            Status::Teapot => 418,
            Status::MisdirectedRequest => 421,
            Status::UnprocessableEntity => 422,
            Status::Locked => 423,
            Status::FailedDependency => 424,
            Status::TooEarly => 425,
            Status::UpgradeRequired => 426,
            Status::PreconditionRequired => 428,
            Status::TooManyRequests => 429,
            Status::RequestHeaderFieldsTooLarge => 431,
            Status::UnavailableForLegalReasons => 451,
            Status::InternalServerError => 500,
            Status::NotImplemented => 501,
            Status::BadGateway => 502,
            Status::ServiceUnavailable => 503,
            Status::GatewayTimeout => 504,
            Status::HTTPVersionNotSupported => 505,
            Status::VariantAlsoNegotiates => 506,
            Status::InsufficientStorage => 507,
            Status::LoopDetected => 508,
            Status::NotExtended => 510,
            Status::NetworkAuthenticationRequired => 511,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Continue => "Continue".to_string(),
            Status::SwitchingProtocols => "Switching Protocols".to_string(),
            Status::Processing => "Processing".to_string(),
            Status::EarlyHints => "Early Hints".to_string(),
            Status::OK => "OK".to_string(),
            Status::Created => "Created".to_string(),
            Status::Accepted => "Accepted".to_string(),
            Status::NonAuthoritativeInfo => "Non-Authoritative Information".to_string(),
            Status::NoContent => "No Content".to_string(),
            Status::ResetContent => "Reset Content".to_string(),
            Status::PartialContent => "Partial Content".to_string(),
            Status::MultiStatus => "Multi-Status".to_string(),
            Status::AlreadyReported => "Already Reported".to_string(),
            Status::IMUsed => "IM Used".to_string(),
            Status::MultipleChoices => "Multiple Choices".to_string(),
            Status::MovedPermanently => "Moved Permanently".to_string(),
            Status::Found => "Found".to_string(),
            Status::SeeOther => "See Other".to_string(),
            Status::NotModified => "Not Modified".to_string(),
            Status::UseProxy => "Use Proxy".to_string(),
            Status::TemporaryRedirect => "Temporary Redirect".to_string(),
            Status::PermanentRedirect => "Permanent Redirect".to_string(),
            Status::BadRequest => "Bad Request".to_string(),
            Status::Unauthorized => "Unauthorized".to_string(),
            Status::PaymentRequired => "Payment Required".to_string(),
            Status::Forbidden => "Forbidden".to_string(),
            Status::NotFound => "Not Found".to_string(),
            Status::MethodNotAllowed => "Method Not Allowed".to_string(),
            Status::NotAcceptable => "Not Acceptable".to_string(),
            Status::ProxyAuthRequired => "Proxy Authentication Required".to_string(),
            Status::RequestTimeout => "Request Timeout".to_string(),
            Status::Conflict => "Conflict".to_string(),
            Status::Gone => "Gone".to_string(),
            Status::LengthRequired => "Length Required".to_string(),
            Status::PreconditionFailed => "Precondition Failed".to_string(),
            Status::RequestEntityTooLarge => "Request Entity Too Large".to_string(),
            Status::RequestURITooLong => "Request URI Too Long".to_string(),
            Status::UnsupportedMediaType => "Unsupported Media Type".to_string(),
            Status::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable".to_string(),
            Status::ExpectationFailed => "Expectation Failed".to_string(),
            Status::Teapot => "Teapot".to_string(),
            Status::MisdirectedRequest => "Misdirected Request".to_string(),
            Status::UnprocessableEntity => "Unprocessable Entity".to_string(),
            Status::Locked => "Locked".to_string(),
            Status::FailedDependency => "Failed Dependency".to_string(),
            Status::TooEarly => "Too Early".to_string(),
            Status::UpgradeRequired => "Upgrade Required".to_string(),
            Status::PreconditionRequired => "Precondition Required".to_string(),
            Status::TooManyRequests => "Too Many Requests".to_string(),
            Status::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large".to_string(),
            Status::UnavailableForLegalReasons => "Unavailable For Legal Reasons".to_string(),
            Status::InternalServerError => "Internal Server Error".to_string(),
            Status::NotImplemented => "Not Implemented".to_string(),
            Status::BadGateway => "Bad Gateway".to_string(),
            Status::ServiceUnavailable => "Service Unavailable".to_string(),
            Status::GatewayTimeout => "Gateway Timeout".to_string(),
            Status::HTTPVersionNotSupported => "HTTP Version Not Supported".to_string(),
            Status::VariantAlsoNegotiates => "Variant Also Negotiates".to_string(),
            Status::InsufficientStorage => "Insufficient Storage".to_string(),
            Status::LoopDetected => "Loop Detected".to_string(),
            Status::NotExtended => "Not Extended".to_string(),
            Status::NetworkAuthenticationRequired => "Network Authentication Required".to_string(),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::OK
    }
}
