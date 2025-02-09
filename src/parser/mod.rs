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
        pub expected_tokens: Vec<(Rule, &'static str)>,
    }

    // This helper function parses the input using the top-level rule,
    // flattens the produced parse tree, and asserts that the tokens appear in the expected order.
    fn assert_arithmetic_expression(test_input: TestInput) {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, test_input.expression);
        match result {
            Ok(pairs) => {
                let mut flat_pairs = pairs.flatten();
                for expected_token in test_input.expected_tokens {
                    let pair = flat_pairs.next().expect("expected a token");
                    assert_eq!(
                        pair.as_rule(),
                        expected_token.0,
                        "For expression `{}`, expected rule {:?} but got {:?}",
                        test_input.expression,
                        expected_token.0,
                        pair.as_rule()
                    );
                    assert_eq!(
                        pair.as_str(),
                        expected_token.1,
                        "For expression `{}`, expected token text `{}` but got `{}`",
                        test_input.expression,
                        expected_token.1,
                        pair.as_str()
                    );
                }
                // Ensure that the next token is the EOI marker.
                assert_eq!(
                    flat_pairs.next().expect("expected a last token").as_rule(),
                    Rule::EOI,
                    "For expression `{}`, expected EOI at the end",
                    test_input.expression
                );
            }
            Err(error) => panic!(
                "parsing failed for expression `{}`: {:?}",
                test_input.expression, error
            ),
        }
    }

    // ===========================
    // Valid Expression Test Cases
    // ===========================

    #[test]
    fn test_one_plus_one() {
        let test_input = TestInput {
            expression: "1+1",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::addition, "+"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_leading_trailing_whitespace() {
        let test_input = TestInput {
            expression: " 1+1 ",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::addition, "+"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_extra_whitespace_between_tokens() {
        let test_input = TestInput {
            expression: "   1   +    1   ",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::addition, "+"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_negative_first_operand() {
        let test_input = TestInput {
            expression: "-1+1",
            expected_tokens: vec![
                (Rule::integer, "-1"),
                (Rule::addition, "+"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_negative_second_operand() {
        let test_input = TestInput {
            expression: "1+-1",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::addition, "+"),
                (Rule::integer, "-1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_both_negative_operands() {
        let test_input = TestInput {
            expression: "-1+-1",
            expected_tokens: vec![
                (Rule::integer, "-1"),
                (Rule::addition, "+"),
                (Rule::integer, "-1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_subtraction_operator() {
        let test_input = TestInput {
            expression: "1-1",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::subtraction, "-"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_multiplication_operator() {
        let test_input = TestInput {
            expression: "1*1",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::multiplication, "*"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_division_operator() {
        let test_input = TestInput {
            expression: "1/1",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::division, "/"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_large_integers() {
        let test_input = TestInput {
            expression: "123+456",
            expected_tokens: vec![
                (Rule::integer, "123"),
                (Rule::addition, "+"),
                (Rule::integer, "456"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_negative_large_first_operand() {
        let test_input = TestInput {
            expression: "-123+456",
            expected_tokens: vec![
                (Rule::integer, "-123"),
                (Rule::addition, "+"),
                (Rule::integer, "456"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_integer_subtraction() {
        let test_input = TestInput {
            expression: "123-456",
            expected_tokens: vec![
                (Rule::integer, "123"),
                (Rule::subtraction, "-"),
                (Rule::integer, "456"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_integer_multiplication() {
        let test_input = TestInput {
            expression: "123*456",
            expected_tokens: vec![
                (Rule::integer, "123"),
                (Rule::multiplication, "*"),
                (Rule::integer, "456"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_integer_division() {
        let test_input = TestInput {
            expression: "123/456",
            expected_tokens: vec![
                (Rule::integer, "123"),
                (Rule::division, "/"),
                (Rule::integer, "456"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_decimal_addition() {
        let test_input = TestInput {
            expression: "1.0+2.0",
            expected_tokens: vec![
                (Rule::decimal, "1.0"),
                (Rule::addition, "+"),
                (Rule::decimal, "2.0"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_negative_decimal_addition() {
        let test_input = TestInput {
            expression: "-1.0+2.0",
            expected_tokens: vec![
                (Rule::decimal, "-1.0"),
                (Rule::addition, "+"),
                (Rule::decimal, "2.0"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_decimal_subtraction() {
        let test_input = TestInput {
            expression: "1.0-2.0",
            expected_tokens: vec![
                (Rule::decimal, "1.0"),
                (Rule::subtraction, "-"),
                (Rule::decimal, "2.0"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_decimal_multiplication() {
        let test_input = TestInput {
            expression: "1.0*2.0",
            expected_tokens: vec![
                (Rule::decimal, "1.0"),
                (Rule::multiplication, "*"),
                (Rule::decimal, "2.0"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_decimal_division() {
        let test_input = TestInput {
            expression: "1.0/2.0",
            expected_tokens: vec![
                (Rule::decimal, "1.0"),
                (Rule::division, "/"),
                (Rule::decimal, "2.0"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_negative_decimal_subtraction() {
        let test_input = TestInput {
            expression: "-1.0-2.0",
            expected_tokens: vec![
                (Rule::decimal, "-1.0"),
                (Rule::subtraction, "-"),
                (Rule::decimal, "2.0"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_mixed_operand_first_decimal() {
        let test_input = TestInput {
            expression: "1.0+2",
            expected_tokens: vec![
                (Rule::decimal, "1.0"),
                (Rule::addition, "+"),
                (Rule::integer, "2"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_mixed_operand_second_decimal() {
        let test_input = TestInput {
            expression: "1+2.0",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::addition, "+"),
                (Rule::decimal, "2.0"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_tab_whitespace() {
        let test_input = TestInput {
            expression: "\t1\t+\t1\t",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::addition, "+"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    #[test]
    fn test_mixed_whitespace() {
        let test_input = TestInput {
            expression: " \t 1 \t +\t 1 \t",
            expected_tokens: vec![
                (Rule::integer, "1"),
                (Rule::addition, "+"),
                (Rule::integer, "1"),
            ],
        };
        assert_arithmetic_expression(test_input);
    }

    // ===========================
    // Invalid Expression Test Cases
    // ===========================

    #[test]
    fn test_invalid_garbage() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "garbage");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_double_plus() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1++1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_too_many_tokens() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1+1+1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_only_operators() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "++");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_missing_left_operand() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "+1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_missing_right_operand() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1+");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_missing_operator() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1 1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_extra_characters_in_second_operand() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1+1a");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_extra_characters_in_first_operand() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1a+1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_empty_input() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_whitespace_only() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_decimal_missing_fraction() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1.+1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_decimal_missing_integer_part() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, ".1+1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_multiple_decimal_points_in_first_operand() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1.1.1+2");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_multiple_decimal_points_in_second_operand() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1+2.2.2");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_plus_sign_in_operand() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1+ +2");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_missing_operand_after_operator() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1*");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_missing_left_operand_multiplication() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "*1");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_missing_right_operand_division() {
        let result = ExpressionParser::parse(Rule::arithmetic_expression, "1/");
        assert!(result.is_err());
    }
}
