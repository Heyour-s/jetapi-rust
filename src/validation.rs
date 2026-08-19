use validator::Validate;
use axum::{extract::FromRequest, response::{IntoResponse, Response}, Json, http::StatusCode};
use serde::de::DeserializeOwned;
use std::ops::Deref;

pub struct Valid<T>(pub T);

impl<T: DeserializeOwned + Validate, S> FromRequest<S> for Valid<T>
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|e| e.into_response())?;
        data.validate().map_err(|e| {
            let msg = e.to_string();
            let status = StatusCode::UNPROCESSABLE_ENTITY;
            (status, Json(serde_json::json!({ "errors": msg }))).into_response()
        })?;
        Ok(Valid(data))
    }
}

impl<T> Deref for Valid<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}