#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::OLPError;
    use actix_web::test;
    use serde_json::json;

    #[test]
    fn test_error_creation() {
        let error = OLPError::geometry_error("Invalid coordinates");
        assert_eq!(error.to_string(), "Geometry error: Invalid coordinates");

        let error = OLPError::validation_error("Invalid input", None);
        assert_eq!(error.to_string(), "Validation error: Invalid input");

        let error = OLPError::not_found("station");
        assert_eq!(error.to_string(), "Not found: station");
    }

    #[test]
    fn test_error_status_codes() {
        let validation_error = OLPError::validation_error("Invalid input", None);
        assert_eq!(validation_error.status_code(), actix_web::http::StatusCode::BAD_REQUEST);

        let not_found_error = OLPError::not_found("resource");
        assert_eq!(not_found_error.status_code(), actix_web::http::StatusCode::NOT_FOUND);

        let internal_error = OLPError::internal_error("Something went wrong");
        assert_eq!(internal_error.status_code(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_from_other_types() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let olp_error: OLPError = io_error.into();
        assert!(matches!(olp_error, OLPError::DataError { .. }));

        let anyhow_error = anyhow::anyhow!("Generic error");
        let olp_error: OLPError = anyhow_error.into();
        assert!(matches!(olp_error, OLPError::InternalError { .. }));
    }

    #[test]
    fn test_error_response_format() {
        let error = OLPError::bad_request("Invalid parameters");
        let response = error.error_response();
        
        assert_eq!(response.status(), actix_web::http::StatusCode::BAD_REQUEST);
    }
}