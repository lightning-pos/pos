use chrono::NaiveDateTime;
use juniper::graphql_object;

use crate::{
    core::{
        entities::auth::user::{User, UserState},
        types::db_uuid::DbUuid,
    },
    AppState,
};

#[graphql_object(context = AppState)]
impl User {
    pub fn id(&self) -> DbUuid {
        self.id
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn full_name(&self) -> String {
        self.full_name.clone()
    }

    pub fn state(&self) -> UserState {
        self.state.clone()
    }

    pub fn last_login_at(&self) -> Option<NaiveDateTime> {
        self.last_login_at
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
