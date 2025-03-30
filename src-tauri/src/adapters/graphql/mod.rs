pub mod analytics;
pub mod auth;
pub mod catalog;
pub mod common;
pub mod finance;
pub mod purchases;
pub mod sales;

pub mod mutations;
pub mod queries;
use juniper::{Context, EmptySubscription, RootNode};

use crate::AppState;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<AppState>>;

impl Context for AppState {}

pub struct Query;
pub struct Mutation;
