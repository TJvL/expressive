use crate::error::ExpressionError;
use crate::parser::{ExpressionParser, Rule};
use expressive_data::numeric::NumericValue;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::collections::HashMap;

pub fn evaluate(input: &str, variables: &HashMap<String, Value>) -> Result<Value, ExpressionError> {
    let result = ExpressionParser::parse(Rule::input, input);

    match result {
        Ok(pairs) => resolve(pairs, variables),
        Err(error) => Err(ExpressionError::ParseError {
            message: format!("{}", error),
        }),
    }
}

fn resolve(
    mut pairs: Pairs<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, ExpressionError> {
    let expression_pair = pairs
        .next()
        .ok_or(ExpressionError::UnexpectedAbstractSyntaxTree)?;
    resolve_branch(expression_pair, variables)
}

fn resolve_branch(
    pair: Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, ExpressionError> {
    match pair.as_rule() {
        Rule::integer => {
            let value = pair.as_str();
            Ok(NumericValue::)
        }
        Rule::decimal => {
            let val = pair.as_str();
            let n = val.parse::<f32>().unwrap_or_else(|_| {
                panic!("expected an decimal value to parse instead got: {}", val)
            });
            Ok(Value::FastDecimal(n))
        }
        Rule::variable_name => {
            let name = pair.as_str();
            variables
                .get(name)
                .cloned()
                .ok_or(ExpressionError::UndefinedVariable {
                    name: name.to_owned(),
                })
        }
        Rule::parenthesized => {
            let expression_pair = pair
                .into_inner()
                .next()
                .ok_or(ExpressionError::UnexpectedAbstractSyntaxTree)?;
            resolve_branch(expression_pair, variables)
        }
        Rule::expression => {
            let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
            resolve_terminals(&pairs, variables)
        }
        _ => Err(ExpressionError::UnexpectedAbstractSyntaxTree),
    }
}

fn resolve_terminals(
    pairs: &[Pair<Rule>],
    variables: &HashMap<String, Value>,
) -> Result<Value, ExpressionError> {
    let mut operand_stack: Vec<Value> = Vec::new();
    let mut operator_stack: Vec<Pair<Rule>> = Vec::new();

    // Operands will be pairs on uneven indices and operators on even.
    for (index, pair) in pairs.iter().enumerate() {
        if index % 2 == 0 {
            let operand_value = resolve_branch(pair.clone(), variables)?;
            operand_stack.push(operand_value);
        } else {
            while let Some(top_op) = operator_stack.last() {
                if get_precedence(top_op) >= get_precedence(pair) {
                    let op = operator_stack.pop().unwrap();
                    let right = operand_stack
                        .pop()
                        .ok_or(ExpressionError::UnexpectedAbstractSyntaxTree)?;
                    let left = operand_stack
                        .pop()
                        .ok_or(ExpressionError::UnexpectedAbstractSyntaxTree)?;
                    let result = apply_operator(left, op, right)?;
                    operand_stack.push(result);
                } else {
                    break;
                }
            }
            operator_stack.push(pair.clone());
        }
    }

    while let Some(op) = operator_stack.pop() {
        let right = operand_stack
            .pop()
            .ok_or(ExpressionError::UnexpectedAbstractSyntaxTree)?;
        let left = operand_stack
            .pop()
            .ok_or(ExpressionError::UnexpectedAbstractSyntaxTree)?;
        let result = apply_operator(left, op, right)?;
        operand_stack.push(result);
    }

    if operand_stack.len() == 1 {
        Ok(operand_stack.pop().unwrap())
    } else {
        Err(ExpressionError::UnexpectedAbstractSyntaxTree)
    }
}

fn get_precedence(operator: &Pair<Rule>) -> u32 {
    match operator.as_rule() {
        Rule::multiplication | Rule::division => 2,
        Rule::addition | Rule::subtraction => 1,
        _ => 0,
    }
}

fn apply_operator(
    left_operand: Value,
    operator: Pair<Rule>,
    right_operand: Value,
) -> Result<Value, ExpressionError> {
    match operator.as_rule() {
        Rule::addition => Ok(left_operand + right_operand),
        Rule::subtraction => Ok(left_operand - right_operand),
        Rule::multiplication => Ok(left_operand * right_operand),
        Rule::division => Ok(left_operand / right_operand),
        _ => Err(ExpressionError::UnexpectedAbstractSyntaxTree),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    #[test]
    fn test_evaluate() {
        let expression = "a / b";
        let mut variables = HashMap::new();
        variables.insert("a".to_string(), Value::FastInteger(3));
        variables.insert(
            "b".to_string(),
            Value::PreciseDecimal(BigDecimal::try_from(1.5).unwrap()),
        );
        let result = evaluate(expression, &variables);
        assert!(result.is_ok(), "result = {:?}", result);
        let value = result.unwrap();
        assert_eq!(
            value,
            Value::PreciseDecimal(BigDecimal::try_from(1.5).unwrap())
        );
    }
}
