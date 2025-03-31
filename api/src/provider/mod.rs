pub mod tmdb;

use crate::model::content::ContentInput;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

pub enum ProviderKey {
    TMDB(String),
}

impl FromStr for ProviderKey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<_> = s.split(":").collect();

        let pk = *spl.first().ok_or("no provider key")?;

        let id = spl.get(1).ok_or("no id")?.to_string();

        match pk {
            "tmdb" => Ok(ProviderKey::TMDB(id)),
            _ => Err("unknown provider"),
        }
    }
}

impl Deref for ProviderKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            ProviderKey::TMDB(id) => id,
        }
    }
}

impl Display for ProviderKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProviderKey::TMDB(id) => format!("tmdb:{}", id),
            }
        )
    }
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Str(&'static str),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Self::Str(e)
    }
}

pub trait Provider {
    fn search(&self, query: &str) -> impl Future<Output = Result<Vec<ContentInput>, Error>> + Send;

    fn get_recommendations(
        &self,
        id: &ProviderKey,
    ) -> impl Future<Output = Result<Vec<ContentInput>, Error>> + Send;
}
