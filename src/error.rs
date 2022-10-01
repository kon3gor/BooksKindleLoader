use std::{
    env::VarError, 
    fmt::Display,
    io::Error as IoError,
};
use lettre::message::header::ContentTypeErr;
use lettre::transport::smtp::Error as SmtpError;
use lettre::error::Error as EmailError;
use lettre::address::AddressError;

#[derive(Debug)]
pub struct BooksError {
    msg: String,
}

impl Display for BooksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl From<VarError> for BooksError {
    fn from(e: VarError) -> Self {
        let msg = format!("{}", e);
        return BooksError { msg };
    }
}

impl From<IoError> for BooksError {
    fn from(e: IoError) -> Self {
        let msg = format!("{}", e);
        return BooksError { msg };
    }
}

impl From<ContentTypeErr> for BooksError {
    fn from(e: ContentTypeErr) -> Self {
        let msg = format!("{}", e);
        return BooksError { msg };
    }
}

impl From<SmtpError> for BooksError {
    fn from(e: SmtpError) -> Self {
        let msg = format!("{}", e);
        return BooksError { msg };
    }
}

impl From<EmailError> for BooksError {
    fn from(e: EmailError) -> Self {
        let msg = format!("{}", e);
        return BooksError { msg };
    }
}

impl From<AddressError> for BooksError {
    fn from(e: AddressError) -> Self {
        let msg = format!("{}", e);
        return BooksError { msg };
    }
}


impl BooksError {
    pub fn new(msg: &str) -> Self {
        return BooksError { msg : msg.to_string() };
    }
}
