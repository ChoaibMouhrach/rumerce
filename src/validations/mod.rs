use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

pub mod auth;
pub mod cart;
pub mod category;
pub mod magic_tokens;
pub mod product;
pub mod role;
pub mod unit;
pub mod user;
pub mod warehouse;

#[derive(Default, Clone, Debug, Copy)]
pub struct ValidatedForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(err) => {
                (StatusCode::BAD_REQUEST, Json(err.field_errors())).into_response()
            }
            ServerError::JsonRejection(_) => {
                (StatusCode::BAD_REQUEST, self.to_string()).into_response()
            }
        }
        .into_response()
    }
}
