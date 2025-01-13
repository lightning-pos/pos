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

    // External Errors
    DieselError(diesel::result::Error),
    SerdeJsonError(serde_json::Error),
    TauriError(tauri::Error),
}

impl From<Error> for InvokeError {
    fn from(err: Error) -> Self {
        InvokeError::from(err.to_string())
    }
}
