#[derive(PartialEq, Eq)]
pub enum Code {
    Succ,

    NotExist,

    // client
    BadRequest,
    EmptyParameter,
    BadID,
    BadNumber,
    NotFound,

    // authorization
    Unauthorized,
    InvalidToken,
    IncorrectPasswd,

    // permission
    NoPermission,
    Forbidden,
    RiskOperation,

    RateLimit,

    // server
    Unavailable,
    InternalError,
    Network,
}

impl Code {
    pub fn value(&self) -> i32 {
        match *self {
            Code::Succ => 0,
            Code::BadRequest => -10,
            Code::EmptyParameter => -11,
            Code::BadID => -12,
            Code::BadNumber => 13,
            Code::NotFound => -14,
            Code::NotExist => -15,
            Code::Unauthorized => -40,
            Code::InvalidToken => -41,
            Code::IncorrectPasswd => -42,
            Code::NoPermission => -50,
            Code::Forbidden => -51,
            Code::RiskOperation => -52,
            Code::RateLimit => -80,
            Code::Unavailable => -90,
            Code::InternalError => -91,
            Code::Network => -92,
        }
    }

    pub fn message(&self) -> String {
        match *self {
            Code::Succ => String::from("OK"),
            Code::BadRequest => String::from("bad request"),
            Code::EmptyParameter => String::from("empty or invalid parameters"),
            Code::BadID => String::from("incorrect id"),
            Code::BadNumber => String::from("invalid number parameters"),
            Code::NotFound => String::from("404 Not Found"),
            Code::NotExist => String::from("data not exist"),
            Code::Unauthorized => String::from("not authorized"),
            Code::InvalidToken => String::from("invalid token or token timeout"),
            Code::IncorrectPasswd => String::from("incorrect password or account doesn't exist"),
            Code::NoPermission => String::from("no permission"),
            Code::Forbidden => String::from("account has been forbidden"),
            Code::RiskOperation => String::from("ask admin for more permission"),
            Code::RateLimit => String::from("too many request"),
            Code::Unavailable => String::from("service temporarily unavailable"),
            Code::InternalError => String::from("service internal error"),
            Code::Network => String::from("network problem"),
        }
    }
}
