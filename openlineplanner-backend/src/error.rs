use std::{error::Error, fmt::Display};
use actix_web::{body::BoxBody, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug, Serialize)]
pub enum OLPError {
    #[error("Geometry error: {message}")]
    GeometryError { message: String },
    
    #[error("Validation error: {message}")]
    ValidationError { message: String, details: Option<ValidationErrors> },
    
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    
    #[error("Data loading error: {message}")]
    DataError { message: String },
    
    #[error("Network error: {message}")]
    NetworkError { message: String },
    
    #[error("Internal server error: {message}")]
    InternalError { message: String },
    
    #[error("Not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Bad request: {message}")]
    BadRequest { message: String },
}

impl OLPError {
    pub fn geometry_error<T: Display>(error: T) -> Self {
        Self::GeometryError {
            message: error.to_string(),
        }
    }

    pub fn validation_error<T: Display>(error: T, details: Option<ValidationErrors>) -> Self {
        Self::ValidationError {
            message: error.to_string(),
            details,
        }
    }

    pub fn config_error<T: Display>(error: T) -> Self {
        Self::ConfigError {
            message: error.to_string(),
        }
    }

    pub fn data_error<T: Display>(error: T) -> Self {
        Self::DataError {
            message: error.to_string(),
        }
    }

    pub fn network_error<T: Display>(error: T) -> Self {
        Self::NetworkError {
            message: error.to_string(),
        }
    }

    pub fn internal_error<T: Display>(error: T) -> Self {
        Self::InternalError {
            message: error.to_string(),
        }
    }

    pub fn not_found(resource: &str) -> Self {
        Self::NotFound {
            resource: resource.to_string(),
        }
    }

    pub fn bad_request<T: Display>(message: T) -> Self {
        Self::BadRequest {
            message: message.to_string(),
        }
    }

    pub fn from_error<T: Display>(error: T) -> Self {
        Self::InternalError {
            message: error.to_string(),
        }
    }
}

impl From<anyhow::Error> for OLPError {
    fn from(error: anyhow::Error) -> Self {
        Self::InternalError {
            message: error.to_string(),
        }
    }
}

impl From<ValidationErrors> for OLPError {
    fn from(errors: ValidationErrors) -> Self {
        Self::ValidationError {
            message: "Validation failed".to_string(),
            details: Some(errors),
        }
    }
}

impl From<config::ConfigError> for OLPError {
    fn from(error: config::ConfigError) -> Self {
        Self::ConfigError {
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for OLPError {
    fn from(error: std::io::Error) -> Self {
        Self::DataError {
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for OLPError {
    fn from(error: serde_json::Error) -> Self {
        Self::BadRequest {
            message: format!("JSON error: {}", error),
        }
    }
}

impl Error for OLPError {}

impl Display for OLPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OLPError::GeometryError { message } => write!(f, "Geometry error: {}", message),
            OLPError::ValidationError { message, .. } => write!(f, "Validation error: {}", message),
            OLPError::ConfigError { message } => write!(f, "Configuration error: {}", message),
            OLPError::DataError { message } => write!(f, "Data error: {}", message),
            OLPError::NetworkError { message } => write!(f, "Network error: {}", message),
            OLPError::InternalError { message } => write!(f, "Internal error: {}", message),
            OLPError::NotFound { resource } => write!(f, "Not found: {}", resource),
            OLPError::BadRequest { message } => write!(f, "Bad request: {}", message),
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

impl Responder for OLPError {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status_code = self.status_code();
        
        log::error!(
            "Request failed: {} {} - {}",
            req.method(),
            req.path(),
            self
        );

        let error_response = ErrorResponse {
            error: self.error_type(),
            message: self.to_string(),
            details: self.details(),
            code: Some(status_code.as_str().to_string()),
        };

        HttpResponse::build(status_code)
            .json(error_response)
    }
}

impl ResponseError for OLPError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            OLPError::ValidationError { .. } | OLPError::BadRequest { .. } => {
                actix_web::http::StatusCode::BAD_REQUEST
            }
            OLPError::NotFound { .. } => actix_web::http::StatusCode::NOT_FOUND,
            OLPError::ConfigError { .. } | OLPError::DataError { .. } => {
                actix_web::http::StatusCode::SERVICE_UNAVAILABLE
            }
            OLPError::NetworkError { .. } => actix_web::http::StatusCode::BAD_GATEWAY,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        self.respond_to(&actix_web::HttpRequest::default())
    }
}

impl OLPError {
    fn error_type(&self) -> String {
        match self {
            OLPError::GeometryError { .. } => "GEOMETRY_ERROR".to_string(),
            OLPError::ValidationError { .. } => "VALIDATION_ERROR".to_string(),
            OLPError::ConfigError { .. } => "CONFIG_ERROR".to_string(),
            OLPError::DataError { .. } => "DATA_ERROR".to_string(),
            OLPError::NetworkError { .. } => "NETWORK_ERROR".to_string(),
            OLPError::InternalError { .. } => "INTERNAL_ERROR".to_string(),
            OLPError::NotFound { .. } => "NOT_FOUND".to_string(),
            OLPError::BadRequest { .. } => "BAD_REQUEST".to_string(),
        }
    }

    fn details(&self) -> Option<serde_json::Value> {
        match self {
            OLPError::ValidationError { details, .. } => {
                details.as_ref().map(|d| serde_json::to_value(d).unwrap_or_default())
            }
            _ => None,
        }
    }
}
