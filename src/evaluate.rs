use crate::value::Value;
use std::collections::HashMap;

pub fn evaluate(
    expression: String,
    variables: HashMap<String, Value>,
) -> Result<Value, EvaluationError> {
    todo!()
}

pub struct EvaluationError {
    message: String,
}

impl EvaluationError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
