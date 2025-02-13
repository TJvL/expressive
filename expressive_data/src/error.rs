use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("could not convert the value '{value:}' to type '{type_name:}'")]
    StringConversionError{ value: String, type_name: String },
}
