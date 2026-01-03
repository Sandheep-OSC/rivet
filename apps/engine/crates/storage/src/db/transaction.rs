use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, TransactionTrait};

pub async fn transaction<T, F, Fut>(db: &DatabaseConnection, f: F) -> Result<T, DbErr>
where
    F: for<'a> FnOnce(&'a dyn ConnectionTrait) -> Fut,
    Fut: std::future::Future<Output = Result<T, DbErr>>,
{
    let txn = db.begin().await?;
    let result = f(&txn).await;

    match result {
        Ok(val) => {
            txn.commit().await?;
            Ok(val)
        }
        Err(e) => {
            txn.rollback().await?;
            Err(e)
        }
    }
}
