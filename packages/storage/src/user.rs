use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{Entity, UserActiveModel};

pub async fn find_by_id(
    user_id: Uuid,
    db: &DatabaseConnection,
) -> Result<crate::models::User, sea_orm::DbErr> {
    Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("User not found".to_string()))
}

pub async fn find_by_email(
    email: &str,
    db: &DatabaseConnection,
) -> Result<crate::models::User, sea_orm::DbErr> {
    Entity::find()
        .filter(crate::models::Column::Email.eq(email))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("User not found".to_string()))
}

pub async fn create_user(
    email: &str,
    password_hash: &str,
    name: &str,
    db: &DatabaseConnection,
) -> Result<crate::models::User, sea_orm::DbErr> {
    let user = UserActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(email.to_string()),
        password_hash: Set(password_hash.to_string()),
        name: Set(name.to_string()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
    };
    user.insert(db).await
}
