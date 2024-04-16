mod http_response_message_extensions {
    use reqwest::{blocking::Response, header::HeaderMap};
    use serde_json::Value;
    use std::time::SystemTime;

    pub fn add_call_time_headers(response: &mut Response, start: SystemTime, end: SystemTime) {
        let duration = end
            .duration_since(start)
            .expect("end should be after start");
        response.headers_mut().insert(
            "X-CallSent-Timestamp",
            start
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .into(),
        );
        response.headers_mut().insert(
            "X-CallReceived-Timestamp",
            end.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .into(),
        );
        response.headers_mut().insert(
            "X-CallLength-Milliseconds",
            duration.as_millis().to_string().parse().unwrap(),
        );
    }

    pub fn get_call_length(response: &Response) -> Option<String> {
        response
            .headers()
            .get("X-CallLength-Milliseconds")
            .and_then(|value| value.to_str().ok())
            .map(|value| format!("{} ms", value))
    }

    pub fn get_header_value(response: &Response, name: &str) -> Option<String> {
        response
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(String::from)
    }

    pub async fn process_string_response(
        response: Response,
        throw_exception_on_failure: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if response.status().is_success() {
            let result = response.text()?;
            // Log the successful request here using your preferred logging framework.
            Ok(result)
        } else if throw_exception_on_failure {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "HTTP request failed",
            )))
        } else {
            Ok(String::new())
        }
    }

    // Example usage for calling the above functions can be placed in a main function or tests.
    #[cfg(test)]
    mod tests {
        use super::*;
        use reqwest::blocking::Response;
        use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
        use std::collections::HashMap;
        use std::time::{SystemTime, UNIX_EPOCH};

        fn setup_mock_response() -> Response {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert("X-CallLength-Milliseconds", HeaderValue::from_static("500"));

            Response::builder()
                .status(200)
                .headers(headers)
                .body("response body".to_string())
                .unwrap()
                .try_into()
                .unwrap()
        }

        #[test]
        fn test_get_call_length() {
            let response = setup_mock_response();
            let call_length = get_call_length(&response);
            assert_eq!(call_length, Some("500 ms".to_string()));
        }

        #[test]
        fn test_get_header_value() {
            let response = setup_mock_response();
            let content_type = get_header_value(&response, "content-type");
            assert_eq!(content_type, Some("application/json".to_string()));
        }

        #[tokio::test]
        async fn test_process_string_response_success() {
            let response = setup_mock_response();
            let result = process_string_response(response, false).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "response body");
        }

        #[tokio::test]
        async fn test_process_string_response_failure() {
            let mut response = setup_mock_response();
            *response.status_mut() = reqwest::StatusCode::INTERNAL_SERVER_ERROR;
            let result = process_string_response(response, true).await;
            assert!(result.is_err());
        }
    }
}
