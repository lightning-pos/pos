use chrono::Utc;
use sea_query::{DeleteStatement, Expr, InsertStatement, SelectStatement, UpdateStatement};
use uuid::Uuid;

use crate::{adapters::outgoing::database::DatabaseAdapter, core::{commands::AppService, models::auth::user_model::{User, UserNewInput, UserUpdateInput, Users}, types::db_uuid::DbUuid}, error::Result};

pub async fn insert_user(service: &AppService, user: UserNewInput) -> Result<User> {
    let id: DbUuid = Uuid::now_v7().into();
    let now = Utc::now().naive_utc();
    let insert_query = InsertStatement::new()
        .into_table(Users::Table)
        .columns(Users::all_columns())
        .values_panic([
            id.into(),
            user.username.into(),
            user.pin_hash.into(),
            user.full_name.into(),
            user.state.into(),
            user.last_login_at.into(),
            now.into(),
            now.into(),
        ])
        .to_owned();
    service.db_adapter.insert_one::<User>(&insert_query).await
}

pub async fn get_user_by_id(service: &AppService, id: DbUuid) -> Result<Option<User>> {
    let select_query = SelectStatement::new()
        .columns(Users::all_columns())
        .from(Users::Table)
        .and_where(Expr::col(Users::Id).eq(id))
        .to_owned();

    service.db_adapter.query_optional::<User>(&select_query).await
}


pub async fn get_user_by_username(service: &AppService, username: &str) -> Result<Option<User>> {
    let select_query = SelectStatement::new()
        .columns(Users::all_columns())
        .from(Users::Table)
        .and_where(Expr::col(Users::Username).eq(username))
        .to_owned();

    service.db_adapter.query_optional::<User>(&select_query).await
}

pub async fn update_user(service: &AppService, user: UserUpdateInput) -> Result<User> {
    let mut update_query = UpdateStatement::new();
    let now = Utc::now().naive_utc();

    update_query.table(Users::Table);

    if let Some(username) = user.username {
        update_query.value(Users::Username, username);
    }

    if let Some(pin_hash) = user.pin_hash {
        update_query.value(Users::PinHash, pin_hash);
    }

    if let Some(full_name) = user.full_name {
        update_query.value(Users::FullName, full_name);
    }

    if let Some(state) = user.state {
        update_query.value(Users::State, state);
    }

    if let Some(last_login_at) = user.last_login_at {
        update_query.value(Users::LastLoginAt, last_login_at);
    }

    let _ = update_query
        .value(Users::UpdatedAt, now)
        .and_where(Expr::col(Users::Id).eq(user.id))
        .to_owned();

    service.db_adapter.update_one::<User>(&update_query).await
}

pub async fn delete_user(service: &AppService, id: DbUuid) -> Result<u64> {
    let delete_query = DeleteStatement::new()
        .from_table(Users::Table)
        .and_where(Expr::col(Users::Id).eq(id))
        .to_owned();

    service.db_adapter.delete(&delete_query).await
}


#[cfg(test)]
mod test {
    use crate::core::{
            commands::{tests::setup_service, AppService},
            models::auth::user_model::{User, UserNewInput, UserState, UserUpdateInput},
            repositories::user_repository::{delete_user, get_user_by_id, get_user_by_username, insert_user, update_user},
        };

    async fn create_user_helper(service: &AppService) -> User {
        let user = UserNewInput {
            username: "testuser".to_string(),
            pin_hash: "pinhash".to_string(),
            full_name: "Full Name".to_string(),
            state: UserState::Active,
            last_login_at: None,
        };

        insert_user(service, user).await.unwrap()
    }

    #[tokio::test]
    async fn test_insert_user() {
        let service = setup_service().await;

        let user = create_user_helper(&service).await;

        assert_eq!(user.username, "testuser");
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let mut service = setup_service().await;

        let user = create_user_helper(&mut service).await;

        let user = get_user_by_id(&mut service, user.id).await.unwrap();
        assert_eq!(user.unwrap().username, "testuser");
    }

    #[tokio::test]
    async fn test_get_user_by_username() {
        let mut service = setup_service().await;

        create_user_helper(&mut service).await;

        let user = get_user_by_username(&mut service, "testuser").await.unwrap();
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "testuser");
    }

    #[tokio::test]
    async fn test_update_user() {
        let mut service = setup_service().await;

        let user = create_user_helper(&service).await;

        let user = update_user(&mut service, UserUpdateInput {
            id: user.id,
            username: Some("testuser2".to_string()),
            pin_hash: None,
            full_name: None,
            state: None,
            last_login_at: None,
        }).await.unwrap();
        assert_eq!(user.username, "testuser2");
    }

    #[tokio::test]
    async fn test_delete_user() {
        let mut service = setup_service().await;

        let user = create_user_helper(&service).await;

        let deleted_users = delete_user(&mut service, user.id).await.unwrap();
        assert_eq!(deleted_users, 1);

        let user = get_user_by_id(&mut service, user.id).await.unwrap();
        assert!(user.is_none());
    }
}
