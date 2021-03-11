pub use lucene_query_builder_rs_derive::*;

use std::fmt;

pub enum Operator {
    Or,
    And,
    End,
}

#[derive(Debug, Clone)]
pub struct QueryString(pub String);

impl fmt::Display for QueryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.contains(' ') {
            write!(f, "\"{}\"", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Or => write!(f, " OR "),
            Self::And => write!(f, " AND "),
            Self::End => write!(f, ""),
        }
    }
}
