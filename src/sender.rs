use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::{books_env::BooksEnv, error::BooksError};
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn load_book_to_kindle(path: &str, env: &BooksEnv) -> Result<(), BooksError> {
    let kindle_address = &env.kindle_address;
    let sender_address = &env.sender_address;
    let password = &env.password;

    let attachment = make_attachment(&path)?;

    let email = Message::builder()
        .from(sender_address.parse()?)
        .to(kindle_address.parse()?)
        .subject("")
        .multipart(MultiPart::mixed().singlepart(attachment))?;

    let creds = Credentials::new(sender_address.to_string(), password.to_string());

    let mailer = SmtpTransport::relay("smtp.yandex.ru")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    return Ok(());
}

fn make_attachment(path: &str) -> Result<SinglePart, BooksError> {
    let filename = match path.split("/").last() {
        Some(v) => v,
        None => return Err(BooksError::new("No filename")),
    };

    let filebody = fs::read(path)?;
    let content_type = choose_mime_type(filename)?.parse()?;
    let attachment = Attachment::new(String::from(filename)).body(filebody, content_type);
    return Ok(attachment);
}

fn choose_mime_type(file_name: &str) -> Result<&'static str, BooksError> {
    let ext = match Path::new(file_name).extension().and_then(OsStr::to_str) {
        Some(v) => v,
        None => return Err(BooksError::new("No extension")),
    };

    return match ext {
        "pdf" => Ok("application/pdf"),
        "mobi" => Ok("application/x-mobipocket-ebook"),
        _ => Err(BooksError::new("Unsupported extension")),
    };
}
