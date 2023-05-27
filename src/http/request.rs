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

    //G ET /serach?name=abc&sort=1 HTTP/1.1

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        // match str::from_utf(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }
        //
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
        //     Ok(request) =>{},
        //     Err(e) => return Err(e),
        // } same with line underneath
        let request = str::from_utf8(buf)?;
        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)>{
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {},
    //         None => break,
    //     }
    // }
    for (i, c) in request.chars().enumerate(){
         if c == ' ' {
             return Some((&request[..i],&request[i + 1..]));
         }
    }
    None
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
