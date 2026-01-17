use storage::DbConnection;
use uuid::Uuid;

pub async fn get_user_by_id(
    user_id: Uuid,
    db: &DbConnection,
) -> Result<storage::models::User, crate::setup::error::Error> {
    let user = storage::user::find_by_id(user_id, db).await.map_err(|e| crate::setup::error::Error::Database(format!("{:?}", e)))?;
    Ok(user)
}
