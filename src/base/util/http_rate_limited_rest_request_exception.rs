mod http_rate_limited_rest_request_exception {

    use reqwest::header::HeaderMap;
    use reqwest::{Error as ReqwestError, Response};
    use thiserror::Error;

    // Define custom error types using thiserror for convenience.
    #[derive(Error, Debug)]
    pub enum HttpError {
        #[error("Network error: {0}")]
        NetworkError(#[from] ReqwestError),

        #[error("Rate limited: bucket {rate_limit_bucket}, partial data: {partial_data:?}")]
        RateLimited {
            rate_limit_bucket: String,
            partial_data: Option<String>, // Simplifying by assuming partial data is string-typed.
        },
    }

    // A hypothetical function that processes an HTTP response and potentially returns a rate-limited error.
    async fn process_response(response: Response) -> Result<(), HttpError> {
        if response.status().as_u16() == 429 {
            // HTTP 429 Too Many Requests
            let rate_limit_bucket = response
                .headers()
                .get("X-RateLimit-Bucket")
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_string();

            let partial_data = response
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|json| json.get("data").map(|data| data.to_string()));

            Err(HttpError::RateLimited {
                rate_limit_bucket,
                partial_data,
            })
        } else {
            Ok(())
        }
    }

    #[tokio::main]
    async fn main() {
        // Example usage, assuming 'client' and 'request' are set up correctly.
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.example.com/data")
            .send()
            .await
            .unwrap();

        match process_response(response).await {
            Ok(_) => println!("Request successful."),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use reqwest::header::{HeaderMap, HeaderValue};
        use reqwest::{Response, StatusCode};
        use serde_json::json;
        use std::str::FromStr;
        use tokio_test::block_on;

        fn mock_response(status_code: StatusCode, headers: HeaderMap, body: String) -> Response {
            Response::builder()
                .status(status_code)
                .headers(headers)
                .body(reqwest::blocking::Body::from(body))
                .unwrap()
                .try_into()
                .unwrap()
        }
        
        #[tokio::test]
        async fn test_process_response_rate_limited() {
            let mut headers = HeaderMap::new();
            headers.insert("X-RateLimit-Bucket", HeaderValue::from_static("bucket123"));
        
            let body = json!({
            "data": "Partial information"
        }).to_string();
        
            let response = mock_response(StatusCode::TOO_MANY_REQUESTS, headers, body);
        
            match process_response(response).await {
                Err(HttpError::RateLimited { rate_limit_bucket, partial_data }) => {
                    assert_eq!(rate_limit_bucket, "bucket123");
                    assert_eq!(partial_data, Some("Partial information".to_string()));
                },
                _ => panic!("Expected rate limited error"),
            }
        }
        
        #[tokio::test]
        async fn test_process_response_success() {
            let headers = HeaderMap::new(); // No special headers needed for a normal response.
            let body = json!({}).to_string();
            let response = mock_response(StatusCode::OK, headers, body);
        
            assert!(process_response(response).await.is_ok());
        }
    }

    //     TODO Tests
}
