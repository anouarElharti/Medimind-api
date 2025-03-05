#[derive(Debug)]
pub enum Error {
    NotFound,
    Database(String),
    Validation(String),
}