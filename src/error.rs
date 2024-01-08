use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum LineLoginError {
    ApiError(LineApiError),
    HttpError(LineHttpError),
    SystemError(LineSystemError),
}

impl LineLoginError {
    pub fn status(&self) -> u16 {
        match self {
            LineLoginError::ApiError(e) => e.status,
            LineLoginError::HttpError(e) => e.status,
            LineLoginError::SystemError(_) => 0,
        }
    }
    pub fn api_error(&self) -> Option<&LineApiErrorResponse> {
        match self {
            LineLoginError::ApiError(e) => Some(&e.error),
            LineLoginError::HttpError(_) => None,
            LineLoginError::SystemError(_) => None,
        }
    }
}

impl From<LineApiError> for LineLoginError {
    fn from(value: LineApiError) -> Self {
        LineLoginError::ApiError(value)
    }
}

impl From<LineHttpError> for LineLoginError {
    fn from(value: LineHttpError) -> Self {
        LineLoginError::HttpError(value)
    }
}

impl From<LineSystemError> for LineLoginError {
    fn from(value: LineSystemError) -> Self {
        LineLoginError::SystemError(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineApiError {
    pub status: u16,
    pub error: LineApiErrorResponse,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

/// https://developers.line.biz/ja/reference/messaging-api/#error-responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineApiErrorResponse {
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Clone)]
pub struct LineHttpError {
    pub status: u16,
    pub http_response_body: Option<String>,
}

impl LineHttpError {
    pub fn new(status: u16, http_response_body: String) -> LineHttpError {
        LineHttpError {
            status,
            http_response_body: Some(http_response_body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineSystemError {
    pub message: Option<String>,
}

impl LineSystemError {
    pub fn new(message: String) -> LineSystemError {
        LineSystemError {
            message: Some(message),
        }
    }
}
