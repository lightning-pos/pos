use derive_more::derive::{Display, From};
use tauri::ipc::InvokeError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Display, From, Debug)]
pub enum Error {
    // Database Errors
    NotFoundError,
    ForeignKeyConstraintError,
    UniqueConstraintError,
    HasChildrenError,
    AlreadyExistsError,
    DatabaseError(String),

    // External Errors
    GraphQLError(juniper::GraphQLError),
    ParseError(juniper::ParseError),
    SerdeJsonError(serde_json::Error),
    TauriError(tauri::Error),
    UuidError(uuid::Error),
    LibsqlError(libsql::Error),
}

impl From<Error> for InvokeError {
    fn from(err: Error) -> Self {
        InvokeError::from(err.to_string())
    }
}
