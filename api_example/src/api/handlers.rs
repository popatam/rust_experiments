use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::dto::todo::{CreateTodo, Todo, UpdateTodo};
use crate::error::Error;
use crate::repo;

pub async fn ping(State(dbpool): State<sqlx::PgPool>) -> Result<String, Error> {
    repo::system::ping(&dbpool).await
}

#[utoipa::path(
    get,
    path = "/v1/todos",
    responses(
        (status = 200, description = "List todos", body = [Todo])
    )
)]
pub async fn todo_list(State(dbpool): State<sqlx::PgPool>) -> Result<Json<Vec<Todo>>, Error> {
    repo::todo::list(&dbpool).await.map(Json::from)
}

#[utoipa::path(
    get,
    path = "/v1/todos/{id}",
    params(
        ("id" = i64, Path, description = "Todo id")
    ),
    responses(
        (status = 200, body = Todo),
        (status = 404, description = "Not found")
    )
)]
pub async fn todo_read(
    State(dbpool): State<sqlx::PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, Error> {
    repo::todo::read(&dbpool, id).await.map(Json::from)
}

#[utoipa::path(
    patch,
    path = "/v1/todos/{id}",
    params(
        ("id" = i64, Path)
    ),
    request_body = UpdateTodo,
    responses(
        (status = 200, body = Todo),
        (status = 404)
    )
)]
pub async fn todo_update(
    State(dbpool): State<sqlx::PgPool>,
    Path(id): Path<i64>,
    Json(update_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, Error> {
    update_todo.validate()?;
    repo::todo::update(&dbpool, id, update_todo).await.map(Json::from)
}

#[utoipa::path(
    post,
    path = "/v1/todos",
    request_body = CreateTodo,
    responses(
        (status = 200, body = Todo)
    )
)]
pub async fn todo_create(
    State(dbpool): State<sqlx::PgPool>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<Json<Todo>, Error> {
    repo::todo::create(&dbpool, new_todo).await.map(Json::from)
}

#[utoipa::path(
    delete,
    path = "/v1/todos/{id}",
    params(
        ("id" = i64, Path)
    ),
    responses(
        (status = 204),
        (status = 404)
    )
)]
pub async fn todo_delete(
    State(dbpool): State<sqlx::PgPool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, Error> {
    repo::todo::delete(&dbpool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}