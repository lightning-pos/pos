pub mod catalog;
pub mod mutations;
use juniper::{Context, EmptySubscription, RootNode};

use crate::AppState;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<AppState>>;

impl Context for AppState {}

pub struct Query;

pub struct Mutation;
