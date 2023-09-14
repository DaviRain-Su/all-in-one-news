#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};

    use secrecy::Secret;
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use zero2prod::domain::SubscriberEmail;
    use zero2prod::email_client::EmailClient;

    use wiremock::Request;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, _request: &Request) -> bool {
            // try to parse the body as as JSON value
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&_request.body);

            if let Ok(body) = result {
                // check if the body has the expected structure
                // check that all the mandatory fields are populated
                // without inspecting the field values.
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                // if parsing fails, do not match the request
                false
            }
        }
    }

    /// generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    /// generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }

    /// generate a random subscriber email
    fn email() -> SubscriberEmail {
        let email: String = SafeEmail().fake();
        SubscriberEmail::parse(&email).unwrap()
    }

    /// create a new email client with a random sender
    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url,
            email(),
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(200),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let response = ResponseTemplate::new(500).set_delay(std::time::Duration::from_secs(180));
        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            // use ur custom matcher
            .and(SendEmailBodyMatcher)
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let ret = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        assert!(ret.is_err())
        // Assert
    }
}
