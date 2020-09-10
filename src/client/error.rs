use reqwest::Error as HTTPError;
use serde::{Deserialize, Serialize};
use serde_json::Error as JSONError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    APIError(Box<APIError>),
    JSONError(Box<JSONError>),
    HTTPError(Box<HTTPError>),
    ResponseError(Box<ResponseError>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIError {
    errcode: u32,
    errmsg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseError {
    code: String,
    zh_desc: String,
    en_desc: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::APIError(ref err) => err.fmt(f),
            Error::JSONError(ref err) => err.fmt(f),
            Error::HTTPError(ref err) => err.fmt(f),
            Error::ResponseError(ref err) => err.fmt(f),
        }
    }
}

impl From<JSONError> for Error {
    fn from(err: JSONError) -> Error {
        Error::JSONError(Box::new(err))
    }
}

impl From<HTTPError> for Error {
    fn from(err: HTTPError) -> Error {
        Error::HTTPError(Box::new(err))
    }
}

impl From<APIError> for Error {
    fn from(err: APIError) -> Error {
        Error::APIError(Box::new(err))
    }
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Error {
        Error::ResponseError(Box::new(err))
    }
}

impl APIError {
    pub fn new<S>(code: u32, msg: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            errcode: code,
            errmsg: msg.into(),
        }
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.errmsg)
    }
}

impl StdError for APIError {
    fn description(&self) -> &str {
        &self.errmsg
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.zh_desc)
    }
}

impl StdError for ResponseError {
    fn description(&self) -> &str {
        &self.zh_desc
    }
}
