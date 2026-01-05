use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::error::Error;

#[derive(Serialize, Clone, sqlx::FromRow, ToSchema)]
pub struct Todo {
    id: i64,
    body: String,
    done: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTodo {
    pub(crate) body: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTodo {
    pub(crate) body: Option<String>,
    pub(crate) done: Option<bool>,
}

impl UpdateTodo {
    pub fn validate(&self) -> Result<(), Error> {
        if self.body.is_none() && self.done.is_none() {
            return Err(Error::Validation(
                StatusCode::UNPROCESSABLE_ENTITY,
                "At least one of 'body' or 'done' must be provided".to_string(),
            ));
        }
        Ok(())
    }
}