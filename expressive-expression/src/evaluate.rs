use std::collections::HashMap;
use pest::Parser;
use crate::parser::{ExpressionParser, Rule};

pub fn evaluate_expression(input: &str, variables: HashMap<String, String>) -> Result<String, String> {
    let result = ExpressionParser::parse(Rule::input, input);
    
    if let Ok(pairs) = result {
        Ok(format!("{:?}", pairs))
    } else {
        Err(format!("Error: {:?}", result))
    }
}
