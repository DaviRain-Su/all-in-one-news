use axum::body::Body;
use axum::http::Response;
use axum::http::{request::Parts, StatusCode};
use axum::{
    async_trait,
    extract::{Form, FromRef, FromRequestParts, State},
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Acquire, PgConnection};
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use crate::email_client::EmailClient;

#[derive(Deserialize, Debug)]
pub struct FormData {
    name: String,
    email: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = anyhow::Error;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(&value.name)?;
        let email = SubscriberEmail::parse(&value.email)?;
        Ok(Self::new(email, name))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),
    // Transparent delegates both `Display`'s and `source`'s implementation
    // to the type wrapped by `UnexpectedError`.
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

// we can extract the connection pool with `State`
pub async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub struct WrapEmailClient(pub EmailClient);

impl FromRef<EmailClient> for WrapEmailClient {
    fn from_ref(state: &EmailClient) -> Self {
        Self(state.clone())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for WrapEmailClient
where
    EmailClient: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let email_client = EmailClient::from_ref(state);

        Ok(Self(email_client))
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, conn_pool),
    fields(request_id = %Uuid::new_v4(),subscriber = ?form)
)]
pub async fn subscribe(
    DatabaseConnection(mut conn_pool): DatabaseConnection,
    WrapEmailClient(email_client): WrapEmailClient,
    form: Option<Form<FormData>>,
) -> impl IntoResponse {
    let connection_pool = conn_pool
        .acquire()
        .await
        .expect("Failed to acquire connection");
    match form {
        Some(form) => {
            let new_subscriber = match NewSubscriber::try_from(form.0) {
                Ok(from) => from,
                Err(_) => {
                    let error_text = "invalid form data";
                    let mut resonse = Response::new(Body::from(error_text));
                    *resonse.status_mut() = StatusCode::BAD_REQUEST;
                    return resonse;
                }
            };

            match insert_subscriber(connection_pool, &new_subscriber).await {
                Ok(_) => {
                    // Send a (useless) email to the new subscriber.
                    // We are ignoring email delivery errors for now.
                    if let Err(e) = send_confirmation_email(&email_client, new_subscriber).await {
                        let error_text = format!("invalid form data: {e:?}");
                        let mut resonse = Response::new(Body::from(error_text));
                        *resonse.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                        return resonse;
                    }

                    let response_text = "Received subscription".to_string();
                    Response::new(Body::from(response_text))
                }
                Err(e) => {
                    tracing::error!("Failed to execute query: {:?}", e);
                    let error_text = format!("Database error: {}", e);
                    let mut response = Response::new(Body::from(error_text));
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    response
                }
            }
        }
        None => {
            let error_text = "Missing fields";
            let mut response = Response::new(Body::from(error_text));
            *response.status_mut() = StatusCode::BAD_REQUEST;
            response
        }
    }
    // `_request_span_guard` is dropped at the end of `subscribe`
    // That's when we "exit" the span
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &mut PgConnection,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at, status)
VALUES ($1, $2, $3, $4, 'pending_confirmation')
"#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
) -> anyhow::Result<()> {
    let confirmation_link = "https://my-api.com/subscriptions/confirm";
    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );
    let html_body = format!(
        "Welcome to our newsletter!<br />\
Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );
    email_client
        .send_email(new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}
