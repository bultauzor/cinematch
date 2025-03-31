use crate::provider;

pub mod cinematch;
pub mod utils;

// #[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
    Provider(provider::Error),
    Str(&'static str),
}

impl Error {}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl From<provider::Error> for Error {
    fn from(e: provider::Error) -> Self {
        Self::Provider(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Self::Str(e)
    }
}
