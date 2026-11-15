pub use lucene_query_builder_rs_derive::*;

use std::fmt;

pub enum Operator {
    Or,
    And,
    End,
}

#[derive(Debug, Clone)]
pub struct QueryString(pub String);

pub struct QueryTerm {
    pub negate: bool,
    pub value: String,
    pub operator: Operator,
}

impl QueryTerm {
    pub fn new(value: String) -> Self {
        Self {
            negate: false,
            value,
            operator: Operator::End,
        }
    }

    pub fn placeholder(negate: bool) -> Self {
        Self {
            negate,
            value: String::new(),
            operator: Operator::End,
        }
    }

    pub fn is_placeholder(&self) -> bool {
        self.value.is_empty()
    }
}

impl fmt::Display for QueryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.contains(' ') {
            write!(f, "\"{}\"", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl fmt::Display for QueryTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negate {
            write!(f, "NOT ")?;
        }

        write!(f, "{}{}", self.value, self.operator)
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
