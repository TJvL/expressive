use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser/grammar.pest"]
pub struct ExpressionParser;

#[cfg(test)]
mod tests {
    use crate::parser::{ExpressionParser, Rule};
    use pest::Parser;

    struct TestInput {
        pub expression: &'static str,
        pub expected_rules: Vec<Rule>,
    }

    #[test]
    fn test_arithmetic_expressions() {
        let test_cases = vec![
            TestInput {
                expression: "1 + 2",
                expected_rules: vec![Rule::integer, Rule::addition, Rule::integer],
            },
            TestInput {
                expression: "1 - 2",
                expected_rules: vec![Rule::integer, Rule::subtraction, Rule::integer],
            },
            TestInput {
                expression: "1 * 2",
                expected_rules: vec![Rule::integer, Rule::multiplication, Rule::integer],
            },
            TestInput {
                expression: "1 / 2",
                expected_rules: vec![Rule::integer, Rule::division, Rule::integer],
            },
            TestInput {
                expression: "1.0 + 2.5",
                expected_rules: vec![Rule::decimal, Rule::addition, Rule::decimal],
            },
            TestInput {
                expression: "1.0 - 2.5",
                expected_rules: vec![Rule::decimal, Rule::subtraction, Rule::decimal],
            },
            TestInput {
                expression: "1.0 * 2.5",
                expected_rules: vec![Rule::decimal, Rule::multiplication, Rule::decimal],
            },
            TestInput {
                expression: "1.0 / 2.5",
                expected_rules: vec![Rule::decimal, Rule::division, Rule::decimal],
            },
            TestInput {
                expression: "1 + 2.5",
                expected_rules: vec![Rule::integer, Rule::addition, Rule::decimal],
            },
            TestInput {
                expression: "1 - 2.5",
                expected_rules: vec![Rule::integer, Rule::subtraction, Rule::decimal],
            },
            TestInput {
                expression: "1 * 2.5",
                expected_rules: vec![Rule::integer, Rule::multiplication, Rule::decimal],
            },
            TestInput {
                expression: "1 / 2.5",
                expected_rules: vec![Rule::integer, Rule::division, Rule::decimal],
            },
            TestInput {
                expression: "1.0 + 2",
                expected_rules: vec![Rule::decimal, Rule::addition, Rule::integer],
            },
            TestInput {
                expression: "1.0 - 2",
                expected_rules: vec![Rule::decimal, Rule::subtraction, Rule::integer],
            },
            TestInput {
                expression: "1.0 * 2",
                expected_rules: vec![Rule::decimal, Rule::multiplication, Rule::integer],
            },
            TestInput {
                expression: "1.0 / 2",
                expected_rules: vec![Rule::decimal, Rule::division, Rule::integer],
            },
            TestInput {
                expression: "1+2", // Test no whitespace
                expected_rules: vec![Rule::integer, Rule::addition, Rule::integer],
            },
            TestInput {
                expression: "1-2", // Test no whitespace
                expected_rules: vec![Rule::integer, Rule::subtraction, Rule::integer],
            },
            TestInput {
                expression: "1*2", // Test no whitespace
                expected_rules: vec![Rule::integer, Rule::multiplication, Rule::integer],
            },
            TestInput {
                expression: "1/2", // Test no whitespace
                expected_rules: vec![Rule::integer, Rule::division, Rule::integer],
            },
        ];

        for test_case in test_cases {
            assert_arithmetic_expression(test_case.expression, &test_case.expected_rules);
        }
    }

    #[test]
    fn test_garbage_after_expression() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1 + 2 garbage");
        assert!(result.is_err());
    }

    #[test]
    fn test_garbage_before_expression() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "garbage 1 + 2");
        assert!(result.is_err());
    }

    fn assert_arithmetic_expression(expression: &str, expected_rules: &[Rule]) {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, expression);

        match result {
            Ok(pairs) => {
                let mut pairs_iter = pairs.flatten();
                for &expected_rule in expected_rules {
                    let pair = pairs_iter.next().unwrap();
                    assert_eq!(pair.as_rule(), expected_rule);
                    assert_eq!(pair.as_str().trim(), pair.as_str()); // Check no whitespace in token
                }
                assert!(pairs_iter.next().is_none()); // Check all tokens consumed
            }
            Err(error) => panic!("Parsing failed: {:?}", error),
        }
    }
}
