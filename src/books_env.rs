use crate::error::BooksError;
use std::env;

pub struct BooksEnv {
    pub kindle_address: String,
    pub sender_address: String,
    pub password: String,
}

const KINDLE_ADDRESS: &'static str = "KINDLE_ADDRESS";
const SENDER_ADDRESS: &'static str = "SENDER_ADDRESS";
const PASSWORD: &'static str = "YA_APP_PASSWORD";

pub fn create_env() -> Result<BooksEnv, BooksError> {
    let kindle_address = env::var(KINDLE_ADDRESS)?;
    let sender_address = env::var(SENDER_ADDRESS)?;
    let password = env::var(PASSWORD)?;

    let books_env = BooksEnv { kindle_address, sender_address, password};
    return Ok(books_env);
}

