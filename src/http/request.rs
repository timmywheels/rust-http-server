use std::collections::HashMap;
use super::http_method::{HttpMethod, HttpMethodError};
use super::{QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult, write};
use std::str;
use std::str::Utf8Error;
use crate::http::headers::Headers;

#[derive(Debug)]
pub struct Request<'buf_lifetime> {
    path: &'buf_lifetime str,
    query_string: Option<QueryString<'buf_lifetime>>,
    method: HttpMethod,
    headers: Option<Headers<'buf_lifetime>>,
}

impl<'buf_lifetime> Request<'buf_lifetime> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf_lifetime> TryFrom<&'buf_lifetime [u8]> for Request<'buf_lifetime> {
    type Error = ParseError;

    fn try_from(buf: &'buf_lifetime [u8]) -> Result<Request<'buf_lifetime>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: HttpMethod = method.parse()?;

        let mut query_string = None;
        if let Some(index) = path.find('?') {
            query_string = Some(QueryString::from(&path[index + 1..]));
            path = &path[..index];
        }

        let mut headers = None;
        if let Some(index) = request.find(':') {
            headers = Some(Headers::from(request))
        }

        Ok(Self {
            path,
            query_string,
            method,
            headers,
        })
    }
}

pub fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        } else if char == '\n' {
            return Some((&request[index + 1..], &request[..index]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<HttpMethodError> for ParseError {
    fn from(_: HttpMethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

