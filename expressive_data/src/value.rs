use bigdecimal::num_bigint::BigInt;
use bigdecimal::{BigDecimal, FromPrimitive};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub enum Value {
    PreciseDecimal(BigDecimal),
    PreciseInteger(BigInt),
    FastDecimal(f32),
    FastInteger(i32),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::PreciseDecimal(d) => write!(f, "{}", d),
            Value::PreciseInteger(i) => write!(f, "{}", i),
            Value::FastDecimal(d) => write!(f, "{}", d),
            Value::FastInteger(i) => write!(f, "{}", i),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        use Value::*;
        match (self, rhs) {
            (PreciseDecimal(a), PreciseDecimal(b)) => PreciseDecimal(a + b),
            (PreciseDecimal(a), PreciseInteger(b)) => PreciseDecimal(a + BigDecimal::from(b)),
            (PreciseInteger(a), PreciseDecimal(b)) => PreciseDecimal(BigDecimal::from(a) + b),
            (PreciseInteger(a), PreciseInteger(b)) => PreciseInteger(a + b),
            (FastDecimal(a), FastDecimal(b)) => FastDecimal(a + b),
            (FastDecimal(a), FastInteger(b)) => FastDecimal(a + b as f32),
            (FastInteger(a), FastDecimal(b)) => FastDecimal(a as f32 + b),
            (FastInteger(a), FastInteger(b)) => FastInteger(a + b),
            (PreciseDecimal(a), FastDecimal(b)) => {
                let b_bd = BigDecimal::from_f64(b as f64).unwrap();
                PreciseDecimal(a + b_bd)
            }
            (FastDecimal(a), PreciseDecimal(b)) => {
                let a_bd = BigDecimal::from_f64(a as f64).unwrap();
                PreciseDecimal(a_bd + b)
            }
            (PreciseDecimal(a), FastInteger(b)) => {
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a + b_bd)
            }
            (FastInteger(a), PreciseDecimal(b)) => {
                let a_bd = BigDecimal::from(a);
                PreciseDecimal(a_bd + b)
            }
            (PreciseInteger(a), FastDecimal(b)) => {
                let a_bd = BigDecimal::from(a);
                let b_bd = BigDecimal::from_f64(b as f64).unwrap();
                PreciseDecimal(a_bd + b_bd)
            }
            (FastDecimal(a), PreciseInteger(b)) => {
                let a_bd = BigDecimal::from_f64(a as f64).unwrap();
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a_bd + b_bd)
            }
            (PreciseInteger(a), FastInteger(b)) => {
                let a_bd = BigDecimal::from(a);
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a_bd + b_bd)
            }
            (FastInteger(a), PreciseInteger(b)) => {
                let a_bd = BigDecimal::from(a);
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a_bd + b_bd)
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Value {
        use Value::*;
        match (self, rhs) {
            (PreciseDecimal(a), PreciseDecimal(b)) => PreciseDecimal(a - b),
            (PreciseDecimal(a), PreciseInteger(b)) => PreciseDecimal(a - BigDecimal::from(b)),
            (PreciseInteger(a), PreciseDecimal(b)) => PreciseDecimal(BigDecimal::from(a) - b),
            (PreciseInteger(a), PreciseInteger(b)) => PreciseInteger(a - b),
            (FastDecimal(a), FastDecimal(b)) => FastDecimal(a - b),
            (FastDecimal(a), FastInteger(b)) => FastDecimal(a - b as f32),
            (FastInteger(a), FastDecimal(b)) => FastDecimal(a as f32 - b),
            (FastInteger(a), FastInteger(b)) => FastInteger(a - b),
            (PreciseDecimal(a), FastDecimal(b)) => {
                let b_bd = BigDecimal::from_f64(b as f64).unwrap();
                PreciseDecimal(a - b_bd)
            }
            (FastDecimal(a), PreciseDecimal(b)) => {
                let a_bd = BigDecimal::from_f64(a as f64).unwrap();
                PreciseDecimal(a_bd - b)
            }
            (PreciseDecimal(a), FastInteger(b)) => {
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a - b_bd)
            }
            (FastInteger(a), PreciseDecimal(b)) => {
                let a_bd = BigDecimal::from(a);
                PreciseDecimal(a_bd - b)
            }
            (PreciseInteger(a), FastDecimal(b)) => {
                let a_bd = BigDecimal::from(a);
                let b_bd = BigDecimal::from_f64(b as f64).unwrap();
                PreciseDecimal(a_bd - b_bd)
            }
            (FastDecimal(a), PreciseInteger(b)) => {
                let a_bd = BigDecimal::from_f64(a as f64).unwrap();
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a_bd - b_bd)
            }
            (PreciseInteger(a), FastInteger(b)) => {
                let a_bd = BigDecimal::from(a);
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a_bd - b_bd)
            }
            (FastInteger(a), PreciseInteger(b)) => {
                let a_bd = BigDecimal::from(a);
                let b_bd = BigDecimal::from(b);
                PreciseDecimal(a_bd - b_bd)
            }
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        use Value::*;
        match (self, rhs) {
            (PreciseDecimal(a), PreciseDecimal(b)) => PreciseDecimal(a * b),
            (PreciseDecimal(a), PreciseInteger(b)) => PreciseDecimal(a * BigDecimal::from(b)),
            (PreciseInteger(a), PreciseDecimal(b)) => PreciseDecimal(BigDecimal::from(a) * b),
            (PreciseInteger(a), PreciseInteger(b)) => PreciseInteger(a * b),
            (FastDecimal(a), FastDecimal(b)) => FastDecimal(a * b),
            (FastDecimal(a), FastInteger(b)) => FastDecimal(a * b as f32),
            (FastInteger(a), FastDecimal(b)) => FastDecimal(a as f32 * b),
            (FastInteger(a), FastInteger(b)) => FastInteger(a * b),
            (PreciseDecimal(a), FastDecimal(b)) => {
                PreciseDecimal(a * BigDecimal::from_f64(b as f64).unwrap())
            }
            (FastDecimal(a), PreciseDecimal(b)) => {
                PreciseDecimal(BigDecimal::from_f64(a as f64).unwrap() * b)
            }
            (PreciseDecimal(a), FastInteger(b)) => PreciseDecimal(a * BigDecimal::from(b)),
            (FastInteger(a), PreciseDecimal(b)) => PreciseDecimal(BigDecimal::from(a) * b),
            (PreciseInteger(a), FastDecimal(b)) => {
                PreciseDecimal(BigDecimal::from(a) * BigDecimal::from_f64(b as f64).unwrap())
            }
            (FastDecimal(a), PreciseInteger(b)) => {
                PreciseDecimal(BigDecimal::from_f64(a as f64).unwrap() * BigDecimal::from(b))
            }
            (PreciseInteger(a), FastInteger(b)) => {
                PreciseDecimal(BigDecimal::from(a) * BigDecimal::from(b))
            }
            (FastInteger(a), PreciseInteger(b)) => {
                PreciseDecimal(BigDecimal::from(a) * BigDecimal::from(b))
            }
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Value {
        use Value::*;
        match (self, rhs) {
            (PreciseDecimal(a), PreciseDecimal(b)) => PreciseDecimal(a / b),
            (PreciseDecimal(a), PreciseInteger(b)) => PreciseDecimal(a / BigDecimal::from(b)),
            (PreciseInteger(a), PreciseDecimal(b)) => PreciseDecimal(BigDecimal::from(a) / b),
            (PreciseInteger(a), PreciseInteger(b)) => {
                PreciseDecimal(BigDecimal::from(a) / BigDecimal::from(b))
            }
            (FastDecimal(a), FastDecimal(b)) => FastDecimal(a / b),
            (FastDecimal(a), FastInteger(b)) => FastDecimal(a / b as f32),
            (FastInteger(a), FastDecimal(b)) => FastDecimal(a as f32 / b),
            (FastInteger(a), FastInteger(b)) => FastDecimal(a as f32 / b as f32),
            (PreciseDecimal(a), FastDecimal(b)) => {
                PreciseDecimal(a / BigDecimal::from_f64(b as f64).unwrap())
            }
            (FastDecimal(a), PreciseDecimal(b)) => {
                PreciseDecimal(BigDecimal::from_f64(a as f64).unwrap() / b)
            }
            (PreciseDecimal(a), FastInteger(b)) => PreciseDecimal(a / BigDecimal::from(b)),
            (FastInteger(a), PreciseDecimal(b)) => PreciseDecimal(BigDecimal::from(a) / b),
            (PreciseInteger(a), FastDecimal(b)) => {
                PreciseDecimal(BigDecimal::from(a) / BigDecimal::from_f64(b as f64).unwrap())
            }
            (FastDecimal(a), PreciseInteger(b)) => {
                PreciseDecimal(BigDecimal::from_f64(a as f64).unwrap() / BigDecimal::from(b))
            }
            (PreciseInteger(a), FastInteger(b)) => {
                PreciseDecimal(BigDecimal::from(a) / BigDecimal::from(b))
            }
            (FastInteger(a), PreciseInteger(b)) => {
                PreciseDecimal(BigDecimal::from(a) / BigDecimal::from(b))
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        // Helper: convert self to BigDecimal.
        let lhs_bd = match self {
            PreciseDecimal(d) => d.clone(),
            PreciseInteger(i) => BigDecimal::from(i.clone()),
            FastDecimal(f) => BigDecimal::from_f64(*f as f64).unwrap(),
            FastInteger(i) => BigDecimal::from(BigInt::from(*i)),
        };
        let rhs_bd = match other {
            PreciseDecimal(d) => d.clone(),
            PreciseInteger(i) => BigDecimal::from(i.clone()),
            FastDecimal(f) => BigDecimal::from_f64(*f as f64).unwrap(),
            FastInteger(i) => BigDecimal::from(BigInt::from(*i)),
        };
        lhs_bd == rhs_bd
    }
}
