#[derive(Debug)]
pub enum Error {
    NotFound,
    InvalidInput,
    DatabaseError(String),
}
