mod http_rest_request_exception {
    use std::fmt;

    use reqwest::{Error, Response};

    pub struct http_rest_request_exception {
        pub response: Option<Response>,
        pub message: String,
    }

    impl http_rest_request_exception {
        pub fn new() -> http_rest_request_exception {
            http_rest_request_exception {
                response: None,
                message: String::from(""),
            }
        }

        pub fn with_message(message: &str) -> http_rest_request_exception {
            http_rest_request_exception {
                response: None,
                message: String::from(message),
            }
        }

        pub fn with_response(response: Response) -> http_rest_request_exception {
            let message = format!("{}", response.status());
            http_rest_request_exception {
                response: Some(response),
                message,
            }
        }
    }

    impl fmt::Display for http_rest_request_exception {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self.response {
                Some(response) => write!(f, "{} - {}", response.url(), self.message),
                None => write!(f, "{}", self.message),
            }
        }
    }

    impl From<Error> for http_rest_request_exception {
        fn from(error: Error) -> Self {
            http_rest_request_exception::with_message(&error.to_string())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use reqwest::Response;
        use std::convert::TryFrom;

        #[test]
        fn test_new() {
            let e = http_rest_request_exception::new();
            assert!(e.response.is_none());
            assert_eq!(e.message, "");
        }
    }
    #[test]
    fn test_with_message() {
        let e = http_rest_request_exception::with_message("Test message");
        assert!(e.response.is_none());
        assert_eq!(e.message, "Test message");
    }

    // #[test]
    // fn test_with_response() {
    //     let response = Response::try_from(reqwest::Url::parse("http://example.com").unwrap()).unwrap();
    //     let e = http_rest_request_exception::with_response(response);
    //     assert!(e.response.is_some());
    //     assert_eq!(e.message, "200 OK");
    // }

    #[test]
    fn test_display() {
        let e = http_rest_request_exception::with_message("Test message");
        assert_eq!(format!("{}", e), "Test message");
    }

    // #[test]
    // fn test_from() {
    //     let error = reqwest::Error::new(reqwest::StatusCode::INTERNAL_SERVER_ERROR, "Test error");
    //     let e = http_rest_request_exception::from(error);
    //     assert_eq!(e.message, "500 Internal Server Error");
    // }
}
