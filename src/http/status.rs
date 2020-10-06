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
            Status::Continue => String::from("Continue"),
            Status::SwitchingProtocols => String::from("Switching Protocols"),
            Status::Processing => String::from("Processing"),
            Status::EarlyHints => String::from("Early Hints"),
            Status::OK => String::from("OK"),
            Status::Created => String::from("Created"),
            Status::Accepted => String::from("Accepted"),
            Status::NonAuthoritativeInfo => String::from("Non-Authoritative Information"),
            Status::NoContent => String::from("No Content"),
            Status::ResetContent => String::from("Reset Content"),
            Status::PartialContent => String::from("Partial Content"),
            Status::MultiStatus => String::from("Multi-Status"),
            Status::AlreadyReported => String::from("Already Reported"),
            Status::IMUsed => String::from("IM Used"),
            Status::MultipleChoices => String::from("Multiple Choices"),
            Status::MovedPermanently => String::from("Moved Permanently"),
            Status::Found => String::from("Found"),
            Status::SeeOther => String::from("See Other"),
            Status::NotModified => String::from("Not Modified"),
            Status::UseProxy => String::from("Use Proxy"),
            Status::TemporaryRedirect => String::from("Temporary Redirect"),
            Status::PermanentRedirect => String::from("Permanent Redirect"),
            Status::BadRequest => String::from("Bad Request"),
            Status::Unauthorized => String::from("Unauthorized"),
            Status::PaymentRequired => String::from("Payment Required"),
            Status::Forbidden => String::from("Forbidden"),
            Status::NotFound => String::from("Not Found"),
            Status::MethodNotAllowed => String::from("Method Not Allowed"),
            Status::NotAcceptable => String::from("Not Acceptable"),
            Status::ProxyAuthRequired => String::from("Proxy Authentication Required"),
            Status::RequestTimeout => String::from("Request Timeout"),
            Status::Conflict => String::from("Conflict"),
            Status::Gone => String::from("Gone"),
            Status::LengthRequired => String::from("Length Required"),
            Status::PreconditionFailed => String::from("Precondition Failed"),
            Status::RequestEntityTooLarge => String::from("Request Entity Too Large"),
            Status::RequestURITooLong => String::from("Request URI Too Long"),
            Status::UnsupportedMediaType => String::from("Unsupported Media Type"),
            Status::RequestedRangeNotSatisfiable => String::from("Requested Range Not Satisfiable"),
            Status::ExpectationFailed => String::from("Expectation Failed"),
            Status::Teapot => String::from("Teapot"),
            Status::MisdirectedRequest => String::from("Misdirected Request"),
            Status::UnprocessableEntity => String::from("Unprocessable Entity"),
            Status::Locked => String::from("Locked"),
            Status::FailedDependency => String::from("Failed Dependency"),
            Status::TooEarly => String::from("Too Early"),
            Status::UpgradeRequired => String::from("Upgrade Required"),
            Status::PreconditionRequired => String::from("Precondition Required"),
            Status::TooManyRequests => String::from("Too Many Requests"),
            Status::RequestHeaderFieldsTooLarge => String::from("Request Header Fields Too Large"),
            Status::UnavailableForLegalReasons => String::from("Unavailable For Legal Reasons"),
            Status::InternalServerError => String::from("Internal Server Error"),
            Status::NotImplemented => String::from("Not Implemented"),
            Status::BadGateway => String::from("Bad Gateway"),
            Status::ServiceUnavailable => String::from("Service Unavailable"),
            Status::GatewayTimeout => String::from("Gateway Timeout"),
            Status::HTTPVersionNotSupported => String::from("HTTP Version Not Supported"),
            Status::VariantAlsoNegotiates => String::from("Variant Also Negotiates"),
            Status::InsufficientStorage => String::from("Insufficient Storage"),
            Status::LoopDetected => String::from("Loop Detected"),
            Status::NotExtended => String::from("Not Extended"),
            Status::NetworkAuthenticationRequired => {
                String::from("Network Authentication Required")
            }
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::OK
    }
}
