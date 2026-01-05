pub(crate) async fn init_dbpool() -> Result<sqlx::PgPool, sqlx::Error> {
    use sqlx::postgres::PgConnectOptions;
    use sqlx::postgres::PgPoolOptions;
    use std::str::FromStr;

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/todo_db".to_string());

    let dbpool = PgPoolOptions::new()
        .connect_with(PgConnectOptions::from_str(&db_connection_str)?)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&dbpool)
        .await
        .expect("DB migrations failed");
    Ok(dbpool)
}