use sqlx::{Connection, PgPool};
use crate::error::Error;

pub(crate) async fn ping(dbpool: &PgPool) -> Result<String, Error>  {
    let mut conn = dbpool.acquire().await?;
    conn.ping()
        .await
        .map(|_| "ok".to_string())
        .map_err(Into::into)
}