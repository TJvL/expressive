use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExpressionError {
    #[error("could not parse expression: {message:}")]
    ParseError { message: String },
    #[error("could not evaluate expression: '{expression:?}'")]
    EvaluationError { expression: String },
    #[error("the variable with name '{name:?}' is undefined")]
    UndefinedVariable { name: String },
    #[error("unexpected abstract syntax tree structure")]
    UnexpectedAbstractSyntaxTree,
}
