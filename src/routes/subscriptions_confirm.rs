use axum::{extract::Query, response::IntoResponse};

#[derive(serde::Deserialize, Debug)]
pub struct Parameters {
    pub subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
pub async fn confirm(_parameters: Query<Parameters>) -> impl IntoResponse {
    "comfirmed!"
}
