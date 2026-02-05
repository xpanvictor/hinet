use std::error::Error;

#[derive(Debug)]
pub enum DbError {
    InvalidDataType(anyhow::Error),
    DatabaseConnectionFailed,
}
