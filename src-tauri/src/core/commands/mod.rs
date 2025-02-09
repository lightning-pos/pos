pub mod app_service;
pub mod auth;
pub mod catalog;
pub mod common;
pub mod sales;

pub use app_service::*;
pub use auth::*;
pub use catalog::*;
pub use common::*;
pub use sales::*;

use crate::error::Result;
use app_service::AppService;

pub trait Command {
    type Output;
    fn exec(&self, service: &mut AppService) -> Result<Self::Output>;
}
