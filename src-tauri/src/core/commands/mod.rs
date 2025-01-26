pub mod app_service;
pub mod auth;
pub mod catalog;

use crate::error::Result;
use app_service::AppService;

pub trait Command {
    type Output;
    fn exec(&self, service: &mut AppService) -> Result<Self::Output>;
}
