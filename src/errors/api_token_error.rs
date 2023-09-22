use std::fmt;

pub enum ErrorCode {
    ReadError,
    ParseError,
    MissingTokenError
}
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = match self {
            ErrorCode::ReadError => "READ_ERROR",
            ErrorCode::ParseError => "PARSE_ERROR",
            ErrorCode::MissingTokenError => "MISSING_TOKEN_ERROR"
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
        write!(f, "ApiTokenError {{ code: {}, message: {} }}", self.code, self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            ErrorCode::ReadError => "Could not read the secrets.json file! Check if the file exists in the root folder.",
            ErrorCode::ParseError => "Could not parse the secrets.json file! Check if the file is formatted correctly.",
            ErrorCode::MissingTokenError => "Could not read the token! Check if a 'stratz_token' entry exists in the json file.",
        };

        write!(f, "{}", err_msg)
    }
}