use std::fs::read_to_string;
use serde_json::{Value, from_str};

use crate::errors::api_token_error::{Error, ErrorCode};

const SECRETS_FILE: &str = "secrets.json";
const STRATZ_TOKEN_KEY: &str = "stratz_token";

pub fn get_token() -> Result<String, Error> {
    let file_string = match read_to_string(SECRETS_FILE) {
        Ok(value) => Ok(value),
        Err(e) => Err(Error { code: ErrorCode::ReadError, message: e.to_string() })
    }?;
    let file_json: Value = match from_str(file_string.as_str()) {
        Ok(value) => Ok(value),
        Err(e) => Err(Error { code: ErrorCode::ParseError, message: e.to_string() })
    }?;
    let token_value = match file_json.get(STRATZ_TOKEN_KEY) {
        Some(value) => Ok(value),
        None => Err(Error { code: ErrorCode::MissingTokenError, message: format!("[{}] does not exist in [{}]", STRATZ_TOKEN_KEY, SECRETS_FILE) })
    }?;

    match token_value.as_str() {
        Some(value) => Ok(value.to_string()),
        None => Err(Error { code: ErrorCode::ParseError, message: format!("[{}] was not in a string format", STRATZ_TOKEN_KEY) })
    }
}