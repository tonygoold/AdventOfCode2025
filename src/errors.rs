use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    Generic(String),
    InvalidRegex(Box<dyn Error>),
}

impl ParseError {
    pub fn generic(s: &str) -> Self {
        Self::Generic(s.to_owned())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(s) => write!(f, "parse error: {}", s),
            Self::InvalidRegex(s) => s.fmt(f),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Generic(_) => None,
            Self::InvalidRegex(err) => Some(err.as_ref()),
        }
    }
}
