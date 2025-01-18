pub mod catalog;
use juniper::{Context, EmptyMutation, EmptySubscription, RootNode};

use crate::AppState;

pub type Schema = RootNode<'static, Query, EmptyMutation<AppState>, EmptySubscription<AppState>>;

impl Context for AppState {}

pub struct Query;
