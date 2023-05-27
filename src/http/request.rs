use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display,Formatter,Result as FmtResult, Debug};
use std::str;


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;

    //GET /serach?

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        match str::from_utf(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding),
        }

        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
            Ok(request) =>{},
            Err(e) => return Err(e),
        }

        let request = str::from_utf8(buf)?;
        unimplemented!();
    }
}
pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str{
        match self{
           Self::InvalidRequest => "InvalidRequest",
           Self::InvalidEncoding => "InvalidEncoding",
           Self::InvalidProtocol => "InvalidProtocol",
           Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }

}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f,"{}", self.message())
    }
}

impl Error for ParseError{}
