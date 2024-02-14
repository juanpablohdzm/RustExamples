use std::fmt::{Display};
use std::io::Write;
use super::status_code::StatusCode;

#[derive(Debug)]
pub  struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) ->Self{
        Response {status_code, body}
    }
    
    // Write is a trait, we don't know what concrete type will call it for example vector
    // so we need to write dynamic (dyn) to say that someone can override it
    // this is possible because of the use of a vtable which is a map between traits and pointers to concrete types
    // this will be handled at runtime
    // at compile time we use impl, creates bigger binary as it needs to create a function for every type
    pub  fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n {}",
               self.status_code,
               self.status_code.reason_phrase(),
               body)
    }
}