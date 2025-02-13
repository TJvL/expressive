use crate::error::DataError;
use bigdecimal::{BigDecimal, Num};
use num_bigint::BigInt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

const DEFAULT_DECIMAL_SCALE: i64 = 10;

pub trait Numeric<N>
where
    N: Clone + Add + Sub + Mul + Div + PartialEq + PartialOrd,
{
    fn add(self, other: N) -> Self;
    fn subtract(self, other: N) -> Self;
    fn multiply(self, other: N) -> Self;
    fn divide(self, other: N) -> Self;
    fn equal(self, other: N) -> bool;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumericValue {
    Decimal(BigDecimal),
    Integer(BigInt),
}

impl TryFrom<String> for NumericValue {
    type Error = DataError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let result = BigInt::from_str_radix(&value, 10);
        if let Ok(number) = result {
            Ok(NumericValue::Integer(number))
        } else {
            let result = BigDecimal::from_str_radix(value.as_str(), 10);
            let big_decimal = result.map_err(|_| DataError::StringConversionError {
                value,
                type_name: "numeric value".to_string(),
            })?;
            Ok(NumericValue::Decimal(big_decimal))
        }
    }
}

impl TryFrom<&String> for NumericValue {
    type Error = DataError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        NumericValue::try_from(value.clone())
    }
}

impl TryFrom<&str> for NumericValue {
    type Error = DataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        NumericValue::try_from(value.to_string())
    }
}

impl Numeric<NumericValue> for NumericValue {
    fn add(self, other: NumericValue) -> Self {
        self + other
    }

    fn subtract(self, other: NumericValue) -> Self {
        self - other
    }

    fn multiply(self, other: NumericValue) -> Self {
        self * other
    }

    fn divide(self, other: NumericValue) -> Self {
        self / other
    }

    fn equal(self, other: NumericValue) -> bool {
        self == other
    }
}

impl Display for NumericValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericValue::Decimal(d) => write!(f, "{}", d),
            NumericValue::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl Add for NumericValue {
    type Output = Self;

    fn add(self, other: NumericValue) -> NumericValue {
        match (self, other) {
            (NumericValue::Decimal(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number + right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Integer(left_number + right_number)
            }
            (NumericValue::Decimal(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Decimal(left_number + right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number + right_number)
            }
        }
    }
}

impl Sub for NumericValue {
    type Output = Self;

    fn sub(self, other: NumericValue) -> NumericValue {
        match (self, other) {
            (NumericValue::Decimal(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number - right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Integer(left_number - right_number)
            }
            (NumericValue::Decimal(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Decimal(left_number - right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number - right_number)
            }
        }
    }
}

impl Mul for NumericValue {
    type Output = Self;

    fn mul(self, other: NumericValue) -> NumericValue {
        match (self, other) {
            (NumericValue::Decimal(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number * right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Integer(left_number * right_number)
            }
            (NumericValue::Decimal(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Decimal(left_number * right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number * right_number)
            }
        }
    }
}

impl Div for NumericValue {
    type Output = Self;

    fn div(self, other: NumericValue) -> NumericValue {
        match (self, other) {
            (NumericValue::Decimal(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number - right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Decimal(
                    BigDecimal::from_bigint(left_number, DEFAULT_DECIMAL_SCALE)
                        / BigDecimal::from_bigint(right_number, DEFAULT_DECIMAL_SCALE),
                )
            }
            (NumericValue::Decimal(left_number), NumericValue::Integer(right_number)) => {
                NumericValue::Decimal(left_number - right_number)
            }
            (NumericValue::Integer(left_number), NumericValue::Decimal(right_number)) => {
                NumericValue::Decimal(left_number - right_number)
            }
        }
    }
}
