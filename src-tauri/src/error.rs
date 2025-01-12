use derive_more::derive::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Display, From, Debug)]
pub enum Error {
    // Database Errors
    NotFoundError,
    UniqueConstraintError,
    HasChildrenError,

    // External Errors
    DieselError(diesel::result::Error),
    SerdeJsonError(serde_json::Error),
    TauriError(tauri::Error),
}

// impl std::error::Error for Error {}
