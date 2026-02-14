use error_stack::Context;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthErrorTypes {
    InvalidJWTToken,
}

impl fmt::Display for AuthErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthErrorTypes::InvalidJWTToken => write!(f, "Invalid token"),
        }
    }
}

impl Context for AuthErrorTypes {}
