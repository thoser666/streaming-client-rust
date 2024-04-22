mod http_rest_request_exception {
    use reqwest::{Response, Error as ReqwestError};
    use thiserror::Error;
    use std::fmt;

    #[derive(Error, Debug)]
    pub enum HttpError {
        #[error("Network error: {0}")]
        NetworkError(#[from] ReqwestError),

        #[error("HTTP request failed: {0}")]
        HttpRequestFailed { url: String, status: reqwest::StatusCode, body: String },

        #[error("Inner error: {0}")]
        InnerError(String),
    }

    impl HttpError {
        pub async fn new(response: Response) -> Self {
            let status = response.status();
            let url = response.url().to_string();
            let body = response.text().await.unwrap_or_default();

            HttpError::HttpRequestFailed {
                url,
                status,
                body,
            }
        }

        pub fn with_inner(message: &str, inner: &str) -> Self {
            HttpError::InnerError(format!("{} - Inner error: {}", message, inner))
        }
    }

    impl fmt::Display for HttpError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                HttpError::NetworkError(err) => write!(f, "Network error: {}", err),
                HttpError::HttpRequestFailed { url, status, body } => {
                    write!(f, "HTTP request failed: URL {} - Status {} - Body {}", url, status, body)
                },
                HttpError::InnerError(msg) => write!(f, "{}", msg),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use reqwest::{Response, Error as ReqwestError};
        use thiserror::Error;
        use std::fmt;

        #[derive(Error, Debug)]
        pub enum HttpError {
            #[error("Network error: {0}")]
            NetworkError(#[from] ReqwestError),

            #[error("HTTP request failed: {0}")]
            HttpRequestFailed { url: String, status: reqwest::StatusCode, body: String },

            #[error("Inner error: {0}")]
            InnerError(String),
        }

        impl HttpError {
            pub async fn new(response: Response) -> Self {
                let status = response.status();
                let url = response.url().to_string();
                let body = response.text().await.unwrap_or_default();

                HttpError::HttpRequestFailed {
                    url,
                    status,
                    body,
                }
            }

            pub fn with_inner(message: &str, inner: &str) -> Self {
                HttpError::InnerError(format!("{} - Inner error: {}", message, inner))
            }
        }

        impl fmt::Display for HttpError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    HttpError::NetworkError(err) => write!(f, "Network error: {}", err),
                    HttpError::HttpRequestFailed { url, status, body } => {
                        write!(f, "HTTP request failed: URL {} - Status {} - Body {}", url, status, body)
                    },
                    HttpError::InnerError(msg) => write!(f, "{}", msg),
                }
            }
        }

    }

        //   #[tokio::main]
    async fn main() {
        // let client = reqwest::Client::new();
        // let response = client.get("https://httpbin.org/status/500").send().await.unwrap();
        // 
        // if !response.status().is_success() {
        //     let error = HttpError::new(response).await;
        //     println!("{}", error);
        // }
    }



}