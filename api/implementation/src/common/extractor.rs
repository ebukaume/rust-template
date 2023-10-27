use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, FromRequestParts, Query},
    http::{request::Parts, Request},
    BoxError, Form, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::ApplicationError;

pub struct ValidatedBody<B>(pub B);
pub struct ValidatedQuery<Q>(pub Q);
pub struct ValidatedForm<F>(pub F);

#[async_trait]
impl<F, State, Body> FromRequest<State, Body> for ValidatedForm<F>
where
    F: DeserializeOwned + Validate,
    Body: HttpBody + Send + 'static,
    Body::Data: Send,
    Body::Error: Into<BoxError>,
    State: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request(request: Request<Body>, state: &State) -> Result<Self, Self::Rejection> {
        let Form(form_data) = Form::<F>::from_request(request, state).await?;

        form_data.validate()?;

        Ok(ValidatedForm(form_data))
    }
}

#[async_trait]
impl<Q, State> FromRequestParts<State> for ValidatedQuery<Q>
where
    Q: DeserializeOwned + Send + Validate,
    State: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let Query(query_data) = Query::<Q>::from_request_parts(parts, state).await?;

        query_data.validate()?;

        Ok(ValidatedQuery(query_data))
    }
}

#[async_trait]
impl<B, State, Body> FromRequest<State, Body> for ValidatedBody<B>
where
    B: DeserializeOwned + Validate,
    Body: HttpBody + Send + 'static,
    Body::Data: Send,
    Body::Error: Into<BoxError>,
    State: Send + Sync,
{
    type Rejection = ApplicationError;

    async fn from_request(request: Request<Body>, state: &State) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<B>::from_request(request, state).await?;

        data.validate()?;

        Ok(ValidatedBody(data))
    }
}
