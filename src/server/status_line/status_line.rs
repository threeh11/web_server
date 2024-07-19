#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    HTTP1_1,
    HTTP2,
    HTTP3,
}

impl HttpVersion {
    fn as_str(&self) -> &'static str {
        match self {
            HttpVersion::HTTP1_1 => "HTTP/1.1",
            HttpVersion::HTTP2 => "HTTP2",
            HttpVersion::HTTP3 => "HTTP3",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpStatus {
    // 1xx: Informational
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    // 2xx: Success
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    // 3xx: Redirection
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,

    // 4xx: Client Error
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    AuthenticationTimeout,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    RetryWith,
    UnavailableForLegalReasons,
    ClientClosedRequest,

    // 5xx: Server Error
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    BandwidthLimitExceeded,
    NotExtended,
    NetworkAuthenticationRequired,
    UnknownError,
    WebServerIsDown,
    ConnectionTimedOut,
    OriginIsUnreachable,
    ATimeoutOccurred,
    SSLHandshakeFailed,
    InvalidSSLCertificate,
}

impl HttpStatus {
    pub fn to_code(&self) -> u16 {
        match *self {
            HttpStatus::Continue => 100,
            HttpStatus::SwitchingProtocols => 101,
            HttpStatus::Processing => 102,
            HttpStatus::EarlyHints => 103,
            HttpStatus::OK => 200,
            HttpStatus::Created => 201,
            HttpStatus::Accepted => 202,
            HttpStatus::NonAuthoritativeInformation => 203,
            HttpStatus::NoContent => 204,
            HttpStatus::ResetContent => 205,
            HttpStatus::PartialContent => 206,
            HttpStatus::MultiStatus => 207,
            HttpStatus::AlreadyReported => 208,
            HttpStatus::IMUsed => 226,
            HttpStatus::MultipleChoices => 300,
            HttpStatus::MovedPermanently => 301,
            HttpStatus::Found => 302,
            HttpStatus::SeeOther => 303,
            HttpStatus::NotModified => 304,
            HttpStatus::UseProxy => 305,
            HttpStatus::TemporaryRedirect => 307,
            HttpStatus::PermanentRedirect => 308,
            HttpStatus::BadRequest => 400,
            HttpStatus::Unauthorized => 401,
            HttpStatus::PaymentRequired => 402,
            HttpStatus::Forbidden => 403,
            HttpStatus::NotFound => 404,
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::NotAcceptable => 406,
            HttpStatus::ProxyAuthenticationRequired => 407,
            HttpStatus::RequestTimeout => 408,
            HttpStatus::Conflict => 409,
            HttpStatus::Gone => 410,
            HttpStatus::LengthRequired => 411,
            HttpStatus::PreconditionFailed => 412,
            HttpStatus::PayloadTooLarge => 413,
            HttpStatus::URITooLong => 414,
            HttpStatus::UnsupportedMediaType => 415,
            HttpStatus::RangeNotSatisfiable => 416,
            HttpStatus::ExpectationFailed => 417,
            HttpStatus::ImATeapot => 418,
            HttpStatus::AuthenticationTimeout => 419,
            HttpStatus::MisdirectedRequest => 421,
            HttpStatus::UnprocessableEntity => 422,
            HttpStatus::Locked => 423,
            HttpStatus::FailedDependency => 424,
            HttpStatus::TooEarly => 425,
            HttpStatus::UpgradeRequired => 426,
            HttpStatus::PreconditionRequired => 428,
            HttpStatus::TooManyRequests => 429,
            HttpStatus::RequestHeaderFieldsTooLarge => 431,
            HttpStatus::RetryWith => 449,
            HttpStatus::UnavailableForLegalReasons => 451,
            HttpStatus::ClientClosedRequest => 499,
            HttpStatus::InternalServerError => 500,
            HttpStatus::NotImplemented => 501,
            HttpStatus::BadGateway => 502,
            HttpStatus::ServiceUnavailable => 503,
            HttpStatus::GatewayTimeout => 504,
            HttpStatus::HTTPVersionNotSupported => 505,
            HttpStatus::VariantAlsoNegotiates => 506,
            HttpStatus::InsufficientStorage => 507,
            HttpStatus::LoopDetected => 508,
            HttpStatus::BandwidthLimitExceeded => 509,
            HttpStatus::NotExtended => 510,
            HttpStatus::NetworkAuthenticationRequired => 511,
            HttpStatus::UnknownError => 520,
            HttpStatus::WebServerIsDown => 521,
            HttpStatus::ConnectionTimedOut => 522,
            HttpStatus::OriginIsUnreachable => 523,
            HttpStatus::ATimeoutOccurred => 524,
            HttpStatus::SSLHandshakeFailed => 525,
            HttpStatus::InvalidSSLCertificate => 526,
        }
    }

    pub fn to_description(&self) -> &'static str {
        match *self {
            HttpStatus::Continue => "Continue",
            HttpStatus::SwitchingProtocols => "Switching Protocols",
            HttpStatus::Processing => "Processing",
            HttpStatus::EarlyHints => "Early Hints",
            HttpStatus::OK => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NonAuthoritativeInformation => "Non-Authoritative Information",
            HttpStatus::NoContent => "No Content",
            HttpStatus::ResetContent => "Reset Content",
            HttpStatus::PartialContent => "Partial Content",
            HttpStatus::MultiStatus => "Multi-Status",
            HttpStatus::AlreadyReported => "Already Reported",
            HttpStatus::IMUsed => "IM Used",
            HttpStatus::MultipleChoices => "Multiple Choices",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::SeeOther => "See Other",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::UseProxy => "Use Proxy",
            HttpStatus::TemporaryRedirect => "Temporary Redirect",
            HttpStatus::PermanentRedirect => "Permanent Redirect",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::PaymentRequired => "Payment Required",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::NotAcceptable => "Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::Gone => "Gone",
            HttpStatus::LengthRequired => "Length Required",
            HttpStatus::PreconditionFailed => "Precondition Failed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::URITooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "Expectation Failed",
            HttpStatus::ImATeapot => "I'm a Teapot",
            HttpStatus::AuthenticationTimeout => "Authentication Timeout",
            HttpStatus::MisdirectedRequest => "Misdirected Request",
            HttpStatus::UnprocessableEntity => "Unprocessable Entity",
            HttpStatus::Locked => "Locked",
            HttpStatus::FailedDependency => "Failed Dependency",
            HttpStatus::TooEarly => "Too Early",
            HttpStatus::UpgradeRequired => "Upgrade Required",
            HttpStatus::PreconditionRequired => "Precondition Required",
            HttpStatus::TooManyRequests => "Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpStatus::RetryWith => "Retry With",
            HttpStatus::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            HttpStatus::ClientClosedRequest => "Client Closed Request",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
            HttpStatus::HTTPVersionNotSupported => "HTTP Version Not Supported",
            HttpStatus::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpStatus::InsufficientStorage => "Insufficient Storage",
            HttpStatus::LoopDetected => "Loop Detected",
            HttpStatus::BandwidthLimitExceeded => "Bandwidth Limit Exceeded",
            HttpStatus::NotExtended => "Not Extended",
            HttpStatus::NetworkAuthenticationRequired => "Network Authentication Required",
            HttpStatus::UnknownError => "Unknown Error",
            HttpStatus::WebServerIsDown => "Web Server Is Down",
            HttpStatus::ConnectionTimedOut => "Connection Timed Out",
            HttpStatus::OriginIsUnreachable => "Origin Is Unreachable",
            HttpStatus::ATimeoutOccurred => "A Timeout Occurred",
            HttpStatus::SSLHandshakeFailed => "SSL Handshake Failed",
            HttpStatus::InvalidSSLCertificate => "Invalid SSL Certificate",
        }
    }

    pub fn from_code(code: u16) -> Option<HttpStatus> {
        match code {
            100 => Some(HttpStatus::Continue),
            101 => Some(HttpStatus::SwitchingProtocols),
            102 => Some(HttpStatus::Processing),
            103 => Some(HttpStatus::EarlyHints),
            200 => Some(HttpStatus::OK),
            201 => Some(HttpStatus::Created),
            202 => Some(HttpStatus::Accepted),
            203 => Some(HttpStatus::NonAuthoritativeInformation),
            204 => Some(HttpStatus::NoContent),
            205 => Some(HttpStatus::ResetContent),
            206 => Some(HttpStatus::PartialContent),
            207 => Some(HttpStatus::MultiStatus),
            208 => Some(HttpStatus::AlreadyReported),
            226 => Some(HttpStatus::IMUsed),
            300 => Some(HttpStatus::MultipleChoices),
            301 => Some(HttpStatus::MovedPermanently),
            302 => Some(HttpStatus::Found),
            303 => Some(HttpStatus::SeeOther),
            304 => Some(HttpStatus::NotModified),
            305 => Some(HttpStatus::UseProxy),
            307 => Some(HttpStatus::TemporaryRedirect),
            308 => Some(HttpStatus::PermanentRedirect),
            400 => Some(HttpStatus::BadRequest),
            401 => Some(HttpStatus::Unauthorized),
            402 => Some(HttpStatus::PaymentRequired),
            403 => Some(HttpStatus::Forbidden),
            404 => Some(HttpStatus::NotFound),
            405 => Some(HttpStatus::MethodNotAllowed),
            406 => Some(HttpStatus::NotAcceptable),
            407 => Some(HttpStatus::ProxyAuthenticationRequired),
            408 => Some(HttpStatus::RequestTimeout),
            409 => Some(HttpStatus::Conflict),
            410 => Some(HttpStatus::Gone),
            411 => Some(HttpStatus::LengthRequired),
            412 => Some(HttpStatus::PreconditionFailed),
            413 => Some(HttpStatus::PayloadTooLarge),
            414 => Some(HttpStatus::URITooLong),
            415 => Some(HttpStatus::UnsupportedMediaType),
            416 => Some(HttpStatus::RangeNotSatisfiable),
            417 => Some(HttpStatus::ExpectationFailed),
            418 => Some(HttpStatus::ImATeapot),
            419 => Some(HttpStatus::AuthenticationTimeout),
            421 => Some(HttpStatus::MisdirectedRequest),
            422 => Some(HttpStatus::UnprocessableEntity),
            423 => Some(HttpStatus::Locked),
            424 => Some(HttpStatus::FailedDependency),
            425 => Some(HttpStatus::TooEarly),
            426 => Some(HttpStatus::UpgradeRequired),
            428 => Some(HttpStatus::PreconditionRequired),
            429 => Some(HttpStatus::TooManyRequests),
            431 => Some(HttpStatus::RequestHeaderFieldsTooLarge),
            449 => Some(HttpStatus::RetryWith),
            451 => Some(HttpStatus::UnavailableForLegalReasons),
            499 => Some(HttpStatus::ClientClosedRequest),
            500 => Some(HttpStatus::InternalServerError),
            501 => Some(HttpStatus::NotImplemented),
            502 => Some(HttpStatus::BadGateway),
            503 => Some(HttpStatus::ServiceUnavailable),
            504 => Some(HttpStatus::GatewayTimeout),
            505 => Some(HttpStatus::HTTPVersionNotSupported),
            506 => Some(HttpStatus::VariantAlsoNegotiates),
            507 => Some(HttpStatus::InsufficientStorage),
            508 => Some(HttpStatus::LoopDetected),
            509 => Some(HttpStatus::BandwidthLimitExceeded),
            510 => Some(HttpStatus::NotExtended),
            511 => Some(HttpStatus::NetworkAuthenticationRequired),
            520 => Some(HttpStatus::UnknownError),
            521 => Some(HttpStatus::WebServerIsDown),
            522 => Some(HttpStatus::ConnectionTimedOut),
            523 => Some(HttpStatus::OriginIsUnreachable),
            524 => Some(HttpStatus::ATimeoutOccurred),
            525 => Some(HttpStatus::SSLHandshakeFailed),
            526 => Some(HttpStatus::InvalidSSLCertificate),
            _ => None,
        }
    }
}

pub struct StatusLine {
    http_version: Option<HttpVersion>,
    status_code: Option<HttpStatus>,
    descr_status_code: Option<String>,
}

pub struct StatusLineError {
    message: String,
}

impl StatusLine {
    pub fn new() -> Self {
        Self {
            http_version: None,
            status_code: None,
            descr_status_code: None,
        }
    }

    pub fn set_http_version(&mut self, ver: HttpVersion) -> &mut Self {
        self.http_version = Some(ver);
        self
    }

    pub fn set_status_code(&mut self, code: HttpStatus) -> &mut Self {
        self.status_code = Some(code);
        self
    }

    pub fn build(&mut self) -> Result<StatusLine, StatusLineError> {
        if self.http_version.is_none() {
            return Err(StatusLineError {
                message: "Версия http не была установлена".to_string(),
            });
        }

        if self.status_code.is_none() {
            return Err(StatusLineError {
                message: "Код состояния не был установлен".to_string(),
            });
        }

        let descr_code = self.status_code.as_ref().unwrap().to_description();
        self.descr_status_code = Some(String::from(descr_code));

        Ok(Self {
            http_version: self.http_version.take(),
            status_code: self.status_code.take(),
            descr_status_code: self.descr_status_code.take(),
        })
    }
}

//@todo подумать куда выносить тесты

#[cfg(test)]
mod test_status_line {
    use super::*;

    #[test]
    fn success_create_status_line() {
        const TEST_HTTP_VERSION: HttpVersion = HttpVersion::HTTP3;
        const TEST_STATUS_CODE: HttpStatus = HttpStatus::OK;

        let res_status_line = StatusLine::new()
            .set_http_version(TEST_HTTP_VERSION)
            .set_status_code(TEST_STATUS_CODE)
            .build();

        match res_status_line {
            Ok(status_line) => {
                assert_eq!(status_line.http_version, Some(TEST_HTTP_VERSION));
                assert_eq!(status_line.status_code, Some(TEST_STATUS_CODE));
            },
            Err(_) => panic!("Expected successful status line creation, but got an error"),
        }
    }

    #[test]
    fn test_http_version_not_set_error() {
        const TEST_STATUS_CODE: HttpStatus = HttpStatus::OK;

        let res_status_line = StatusLine::new()
            .set_status_code(TEST_STATUS_CODE)
            .build();

        match res_status_line {
            Ok(_) => panic!("Expected an error due to unset HTTP version, but got Ok"),
            Err(e) => assert_eq!(e.message, "Версия http не была установлена"),
        }
    }

    #[test]
    fn test_status_code_not_set_error() {
        const TEST_HTTP_VERSION: HttpVersion = HttpVersion::HTTP3;

        let res_status_line = StatusLine::new()
            .set_http_version(TEST_HTTP_VERSION)
            .build();

        match res_status_line {
            Ok(_) => panic!("Expected an error due to unset status code, but got Ok"),
            Err(e) => assert_eq!(e.message, "Код состояния не был установлен"),
        }
    }

}