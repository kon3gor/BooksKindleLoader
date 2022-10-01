mod books_env;
mod error;
mod sender;

use books_env::BooksEnv;
use error::BooksError;
use regex::Regex;
use std::{fs, ffi::OsStr};
use sender::load_book_to_kindle;

const SUPPORTED_FORMATS: &'static str = "pdf|epub";
const BOOKS_DIR: &'static str = "/Users/kon3gor/Documents/books";

fn main() {
    let books_re = format!(r"[^\\]*\.({})$", SUPPORTED_FORMATS);
    let re = Regex::new(&books_re).unwrap();
    let books_env = match books_env::create_env() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    match iterate_through_dir(BOOKS_DIR, &re, &books_env) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
}

fn iterate_through_dir(origin: &str, re: &Regex, env: &BooksEnv) -> Result<(), BooksError> {
    let paths = fs::read_dir(origin)?;
    for entry in paths {
        let path = entry?.path();

        let metadata = fs::metadata(&path)?;
        let file_name = match path.file_name().and_then(OsStr::to_str) {
            Some(v) => v,
            None => return Err(BooksError::new("No filename")),
        };
        if metadata.is_file() && re.is_match(&file_name) {
            match process_file(origin, file_name, env) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
        } else if metadata.is_dir() {
            match iterate_through_dir(&path.display().to_string(), re, env) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
        }
    }
    return Ok(());
}

fn process_file(origin: &str, file_name: &str, env: &BooksEnv) -> Result<(), BooksError> {
    if file_name.starts_with("+") {
        return Ok(());
    }

    let original_path = create_string_from_origin(&origin, &file_name, false);
    println!("Loading book: {}", file_name);
    load_book_to_kindle(&original_path, env)?;

    let new_path = create_string_from_origin(&origin, &file_name, true);
    fs::rename(original_path, new_path)?;

    return Ok(());
}

fn create_string_from_origin(origin: &str, file_name: &str, with_plus: bool) -> String {
    let mut built = origin.to_owned();
    built.push_str("/");
    if with_plus {
        built.push_str("+");
    }
    built.push_str(file_name);
    return built;
}

