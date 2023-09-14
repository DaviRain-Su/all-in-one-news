use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    ""
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;

    use super::health_check;

    #[tokio::test]
    async fn health_check_success() {
        let response = health_check().await;
        assert!(response.into_response().status().is_success());
    }
}
