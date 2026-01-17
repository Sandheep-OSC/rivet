use storage::DbConnection;

pub async fn login(
    email: &str,
    password: &str,
    db: &DbConnection,
) -> Result<String, crate::setup::error::Error> {
    let user = storage::user::find_by_email(email, db)
        .await
        .map_err(|e| crate::setup::error::Error::Database(format!("{:?}", e)))?;
    if storage::auth::verify_password(password, &user.password_hash)
        .map_err(crate::setup::error::Error::Bcrypt)?
    {
        let token =
            storage::auth::generate_jwt(user.id).map_err(crate::setup::error::Error::Jwt)?;
        Ok(token)
    } else {
        Err(crate::setup::error::Error::Validation(
            "Invalid credentials".to_string(),
        ))
    }
}

pub async fn register(
    email: &str,
    password: &str,
    name: &str,
    db: &DbConnection,
) -> Result<(), crate::setup::error::Error> {
    let password_hash =
        storage::auth::hash_password(password).map_err(crate::setup::error::Error::Bcrypt)?;
    storage::user::create_user(email, &password_hash, name, db)
        .await
        .map_err(|e| crate::setup::error::Error::Database(format!("{:?}", e)))?;
    Ok(())
}
