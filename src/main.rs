use std::fs;
use regex::Regex;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{Attachment, MultiPart};
use std::env;
use std::path::Path;
use std::ffi::OsStr;

const SUPPORTED_FORMATS: &str = "pdf|epub";
const BOOKS_DIR: &str = "/Users/kon3gor/Documents/books";

fn main() {
    let books_re = format!(r"[^\\]*\.({})$", SUPPORTED_FORMATS);
    let re = Regex::new(&books_re).unwrap();

    iterate_through_dir(BOOKS_DIR, &re);
}

fn iterate_through_dir(origin: &str, re: &Regex) {
    let paths = fs::read_dir(origin).unwrap();
    for entry in paths {
        let path = entry.unwrap().path();

        let metadata = fs::metadata(&path).unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if metadata.is_file() && re.is_match(&file_name) {
            process_file(origin, file_name);
        } else if metadata.is_dir() {
            iterate_through_dir(&path.display().to_string(), re);
        }
    }
}

fn process_file(origin: &str, file_name: &str) {
    if file_name.starts_with("+") {
        return;
    }

    let mut original_path = "".to_owned(); 
    original_path.push_str(origin);
    original_path.push_str("/");
    original_path.push_str(file_name);

    load_book_to_kindle(&original_path);

    let mut new_path = "".to_owned(); 
    new_path.push_str(origin);
    new_path.push_str("/+");
    new_path.push_str(file_name);

    println!("Loading book: {}", file_name);
    fs::rename(original_path, new_path).unwrap();
}

fn load_book_to_kindle(path: &str) {
    let kindle_address = env::var("KINDLE_ADDRESS").unwrap();
    let sender_address = env::var("SENDER_ADDRESS").unwrap();
    let password = env::var("YA_APP_PASSWORD").unwrap();

    let filename = path.split("/").last().unwrap();
    let filebody = fs::read(path).unwrap();
    let content_type = choose_mime_type(filename).parse().unwrap();
    let attachment = Attachment::new(String::from(filename)).body(filebody, content_type);

    let email = Message::builder()
        .from(sender_address.parse().unwrap())
        .to(kindle_address.parse().unwrap())
        .subject("")
        .multipart(
            MultiPart::mixed().singlepart(attachment)
        )
        .unwrap();

    let creds = Credentials::new(sender_address.to_string(), password.to_string());

    let mailer = SmtpTransport::relay("smtp.yandex.ru")
        .unwrap()
        .credentials(creds)
        .build();
    
    mailer.send(&email).unwrap();
}

fn choose_mime_type(file_name: &str) -> &'static str {
    let ext = match Path::new(file_name).extension().and_then(OsStr::to_str) {
        Some(v) => v,
        None => panic!("oops"),
    };

    return match ext {
        "pdf" => "application/pdf",
        "mobi" => "application/x-mobipocket-ebook",
        "epub" => "application/epub+zip",
        _ => "WTF",
    };
}

