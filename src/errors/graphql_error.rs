use std::fmt;

pub enum ErrorCode {
    HttpError,
    QueryError
}
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = match self {
            ErrorCode::HttpError => "HTTP_ERROR",
            ErrorCode::QueryError => "QUERY_ERROR"
        };

        write!(f, "{}", code)
    }
}

pub struct Error {
    pub code : ErrorCode,
    pub message: String
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GraphQlError {{ code: {}, message: {} }}", self.code, self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            ErrorCode::HttpError => "Some kind of HttpError",
            ErrorCode::QueryError => "Some kind of QueryError"
        };

        write!(f, "{}", err_msg)
    }
}