use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T> ApiResponse<T> {
    pub fn _new(data: Option<T>, error: Option<ErrorResponse>) -> Self {
        let success = error.is_none();
        ApiResponse {
            success,
            data,
            error,
        }
    }

    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn _error(error: ErrorResponse) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}
