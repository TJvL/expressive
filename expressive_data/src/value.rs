use bigdecimal::num_bigint::BigInt;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    DateTime(DateTime<Utc>),
    Decimal(BigDecimal),
    Integer(BigInt),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::DateTime(d) => write!(f, "{}", d),
            Value::Decimal(d) => write!(f, "{}", d),
            Value::Integer(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}
