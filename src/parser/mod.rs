use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./parser/grammar.pest"]
pub struct ExpressionParser;

#[cfg(test)]
mod tests {
    use crate::parser::{ExpressionParser, Rule};
    use pest::Parser;
    use std::fmt::{Display, Formatter};

    struct ExpectedPair {
        pub rule: Rule,
        pub value: &'static str,
    }

    impl Display for ExpectedPair {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "rule: {:?}, value: {}", self.rule, self.value)
        }
    }

    fn assert_expression_is_valid(input: &str, expected_pairs: &[ExpectedPair]) {
        let result = ExpressionParser::parse(Rule::input, input);
        assert!(
            result.is_ok(),
            "the parse result for input '{}' was expected to succeed instead got error: {}",
            input,
            result.unwrap_err(),
        );
        let pairs = result.unwrap().flatten();
        assert_eq!(
            pairs.len(),
            expected_pairs.len(),
            "the parsed result was expected to have {} pairs instead got {}",
            expected_pairs.len(),
            pairs.len()
        );
        for comparison_pair in pairs.zip(expected_pairs.iter()) {
            let actual = comparison_pair.0;
            let expected = comparison_pair.1;
            assert_eq!(
                actual.as_rule(),
                expected.rule,
                "expected a pair to have rule '{:?}' instead got '{:?}'",
                expected.rule,
                actual.as_rule()
            );
            assert_eq!(
                actual.as_str(),
                expected.value,
                "expected a pair to have value '{}' instead got '{}'",
                expected.value,
                actual.as_str()
            );
        }
    }

    fn assert_expression_is_invalid(input: &str) {
        let result = ExpressionParser::parse(Rule::input, input);
        assert!(result.is_err(), "expected the expression to be invalid");
    }

    #[test]
    fn test_valid_expression() {
        let input = "1 + 1 + (2 - 2)";
        let expected_pairs = vec![
            ExpectedPair {
                rule: Rule::expression,
                value: input,
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "1",
            },
            ExpectedPair {
                rule: Rule::addition,
                value: "+",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "1",
            },
            ExpectedPair {
                rule: Rule::addition,
                value: "+",
            },
            ExpectedPair {
                rule: Rule::parenthesized,
                value: "(2 - 2)",
            },
            ExpectedPair {
                rule: Rule::expression,
                value: "2 - 2",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "2",
            },
            ExpectedPair {
                rule: Rule::subtraction,
                value: "-",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "2",
            },
            ExpectedPair {
                rule: Rule::EOI,
                value: "",
            },
        ];
        assert_expression_is_valid(input, &expected_pairs);
    }

    #[test]
    fn test_invalid_expression() {
        let input = "garbage";
        assert_expression_is_invalid(input);
    }

    #[test]
    fn test_simple_variable_expression() {
        let input = "a + 2";
        let expected_pairs = vec![
            ExpectedPair {
                rule: Rule::expression,
                value: "a + 2",
            },
            ExpectedPair {
                rule: Rule::variable_name,
                value: "a",
            },
            ExpectedPair {
                rule: Rule::addition,
                value: "+",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "2",
            },
            ExpectedPair {
                rule: Rule::EOI,
                value: "",
            },
        ];
        assert_expression_is_valid(input, &expected_pairs);
    }

    #[test]
    fn test_simple_decimal_expression() {
        let input = "3.14 * 2";
        let expected_pairs = vec![
            ExpectedPair {
                rule: Rule::expression,
                value: "3.14 * 2",
            },
            ExpectedPair {
                rule: Rule::decimal,
                value: "3.14",
            },
            ExpectedPair {
                rule: Rule::multiplication,
                value: "*",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "2",
            },
            ExpectedPair {
                rule: Rule::EOI,
                value: "",
            },
        ];
        assert_expression_is_valid(input, &expected_pairs);
    }

    #[test]
    fn test_complex_expression() {
        let input = "a * (b + c) - d / e";
        let expected_pairs = vec![
            ExpectedPair {
                rule: Rule::expression,
                value: "a * (b + c) - d / e",
            },
            ExpectedPair {
                rule: Rule::variable_name,
                value: "a",
            },
            ExpectedPair {
                rule: Rule::multiplication,
                value: "*",
            },
            ExpectedPair {
                rule: Rule::parenthesized,
                value: "(b + c)",
            },
            ExpectedPair {
                rule: Rule::expression,
                value: "b + c",
            },
            ExpectedPair {
                rule: Rule::variable_name,
                value: "b",
            },
            ExpectedPair {
                rule: Rule::addition,
                value: "+",
            },
            ExpectedPair {
                rule: Rule::variable_name,
                value: "c",
            },
            ExpectedPair {
                rule: Rule::subtraction,
                value: "-",
            },
            ExpectedPair {
                rule: Rule::variable_name,
                value: "d",
            },
            ExpectedPair {
                rule: Rule::division,
                value: "/",
            },
            ExpectedPair {
                rule: Rule::variable_name,
                value: "e",
            },
            ExpectedPair {
                rule: Rule::EOI,
                value: "",
            },
        ];
        assert_expression_is_valid(input, &expected_pairs);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "  1+  1 ";
        let expected_pairs = vec![
            ExpectedPair {
                rule: Rule::expression,
                value: "1+  1 ",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "1",
            },
            ExpectedPair {
                rule: Rule::addition,
                value: "+",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "1",
            },
            ExpectedPair {
                rule: Rule::EOI,
                value: "",
            },
        ];
        assert_expression_is_valid(input, &expected_pairs);
    }

    #[test]
    fn test_nested_expression() {
        let input = "((1+2)*3) - 4.5/var";
        let expected_pairs = vec![
            ExpectedPair {
                rule: Rule::expression,
                value: "((1+2)*3) - 4.5/var",
            },
            ExpectedPair {
                rule: Rule::parenthesized,
                value: "((1+2)*3)",
            },
            ExpectedPair {
                rule: Rule::expression,
                value: "(1+2)*3",
            },
            ExpectedPair {
                rule: Rule::parenthesized,
                value: "(1+2)",
            },
            ExpectedPair {
                rule: Rule::expression,
                value: "1+2",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "1",
            },
            ExpectedPair {
                rule: Rule::addition,
                value: "+",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "2",
            },
            ExpectedPair {
                rule: Rule::multiplication,
                value: "*",
            },
            ExpectedPair {
                rule: Rule::integer,
                value: "3",
            },
            ExpectedPair {
                rule: Rule::subtraction,
                value: "-",
            },
            ExpectedPair {
                rule: Rule::decimal,
                value: "4.5",
            },
            ExpectedPair {
                rule: Rule::division,
                value: "/",
            },
            ExpectedPair {
                rule: Rule::variable_name,
                value: "var",
            },
            ExpectedPair {
                rule: Rule::EOI,
                value: "",
            },
        ];
        assert_expression_is_valid(input, &expected_pairs);
    }

    #[test]
    fn test_invalid_newline() {
        let input = "1 +\n2";
        assert_expression_is_invalid(input);
    }

    #[test]
    fn test_invalid_operator_sequence() {
        let input = "1 + * 2";
        assert_expression_is_invalid(input);
    }

    #[test]
    fn test_invalid_empty_input() {
        let input = "";
        assert_expression_is_invalid(input);
    }

    #[test]
    fn test_invalid_decimal_format() {
        let input = "1.";
        assert_expression_is_invalid(input);
    }
}
