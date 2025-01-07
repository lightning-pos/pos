use derive_more::derive::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Display, From, Debug)]
pub enum Error {
    // Database Errors
    NotFoundError,
    UniqueConstraintError,
    HasChildrenError,

    // External Errors
    SerdeJsonError(serde_json::Error),
    SeaQueryError(sea_query::error::Error),
    SQLxError(sqlx::Error),
    TauriError(tauri::Error),
    IntoSeaError(modql::filter::IntoSeaError),
}

// impl std::error::Error for Error {}
