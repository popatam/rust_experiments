use crate::dto::todo::{CreateTodo, Todo, UpdateTodo};
use crate::error::Error;
use sqlx::{query, query_as, PgPool};

pub async fn list(dbpool: &PgPool) -> Result<Vec<Todo>, Error> {
    query_as::<_, Todo>("SELECT * FROM todo ORDER BY id")
        .fetch_all(dbpool)
        .await
        .map_err(Into::into)
}

pub async fn read(dbpool: &PgPool, id: i64) -> Result<Todo, Error> {
    query_as::<_, Todo>("SELECT * FROM todo WHERE id = $1")
        .bind(id)
        .fetch_one(dbpool)
        .await
        .map_err(Into::into)
}

pub async fn create(dbpool: &PgPool, new_todo: CreateTodo) -> Result<Todo, Error> {
    query_as::<_, Todo>("INSERT INTO todo (body) VALUES ($1) RETURNING *")
        .bind(new_todo.body)
        .fetch_one(dbpool)
        .await
        .map_err(Into::into)
}

pub async fn update(dbpool: &PgPool, id: i64, update_todo: UpdateTodo) -> Result<Todo, Error> {
    query_as::<_, Todo>(
        "UPDATE todo
         SET
           body = COALESCE($1, body),
           done = COALESCE($2, done),
           updated_at = now()
         WHERE id = $3
         RETURNING *",
    )
        .bind(update_todo.body)
        .bind(update_todo.done)
        .bind(id)
        .fetch_one(dbpool)
        .await
        .map_err(Into::into)
}

pub async fn delete(dbpool: &PgPool, id: i64) -> Result<(), Error> {
    let deleted = query("DELETE FROM todo WHERE id = $1")
        .bind(id)
        .execute(dbpool)
        .await?;

    if deleted.rows_affected() == 0 {
        return Err(Error::NotFound);
    }

    Ok(())
}