use crate::api::handlers;
use crate::dto::todo::CreateTodo;
use crate::dto::todo::Todo;
use crate::dto::todo::UpdateTodo;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::todo_list,
        handlers::todo_read,
        handlers::todo_create,
        handlers::todo_update,
        handlers::todo_delete
    ),
    components(
        schemas(Todo, CreateTodo, UpdateTodo)
    ),
    tags(
        (name = "todo", description = "Todo API")
    )
)]
struct ApiDoc;

pub fn create_router(dbpool: sqlx::PgPool) -> axum::Router {
    use axum::{Router, routing::get};
    use tower_http::cors::{Any, CorsLayer};
    use tower_http::trace::TraceLayer;

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(|| async { "Ok" }))
        .route("/ready", get(handlers::ping))
        .nest(
            "/v1",
            Router::new()
                .route("/todos", get(handlers::todo_list).post(handlers::todo_create))
                .route(
                    "/todos/{id}",
                    get(handlers::todo_read).patch(handlers::todo_update).delete(handlers::todo_delete),
                ),
        )
        .with_state(dbpool)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .layer(TraceLayer::new_for_http())
}
