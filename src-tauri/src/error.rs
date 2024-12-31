use derive_more::derive::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Display, From)]
pub enum Error {
    NotFoundError,
    UniqueConstraintError,
    HasChildrenError,
    SerdeJsonError(serde_json::Error),
}

// impl std::error::Error for Error {}
