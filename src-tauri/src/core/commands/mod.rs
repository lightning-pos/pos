pub mod app_service;
pub mod auth;
pub mod catalog;
pub mod common;
pub mod purchases;
pub mod sales;

pub use app_service::*;
pub use auth::*;
pub use catalog::*;
pub use common::*;
pub use purchases::*;
pub use sales::*;

use crate::error::Result;

pub trait Command {
    type Output;
    fn exec(&self, service: &mut AppService) -> Result<Self::Output>;
}
