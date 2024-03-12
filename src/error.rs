#[derive(Debug)]
pub enum AppError {
    AlreadyGuessed,
    MongoDbError,
    NotFound,
}

impl From<mongodb::error::Error> for AppError {
    fn from(_mongo_error: mongodb::error::Error) -> Self {
        Self::MongoDbError
    }
}
