use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApiError {
    #[error("network error: {0}")]
    Network(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("client too old: {0}")]
    ClientTooOld(String),

    #[error("api error: {code:?} {status:?}")]
    Api {
        status: u16,
        code: String,
        message: String,
    },

    #[error("bad response: {0}")]
    BadResponse(String),
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Deserialize, Default)]
struct ErrorBody {
    #[serde(default)]
    code: String,
    #[serde(default)]
    message: String,
}

impl ApiError {
    pub fn from_response(status: u16, raw_body: &str) -> Self {
        let body = serde_json::from_str::<ErrorBody>(raw_body).unwrap_or_default();

        let message: String = if body.message.is_empty() {
            format!("HTTP {status}")
        } else {
            body.message
        };

        match status {
            401 => ApiError::Unauthorized(message),
            426 => ApiError::ClientTooOld(message),
            _ => ApiError::Api {
                status,
                code: body.code,
                message,
            },
        }
    }

    pub fn user_message(&self) -> &str {
        match self {
            ApiError::Network(_) => {
                "Не удалось установить связь с сервером. Проверьте ваше соединение с Интернетом и попробуйте ещё раз."
            }
            ApiError::BadResponse(_) => "Сервер вернул некорректный ответ.",
            ApiError::Unauthorized(msg) | ApiError::ClientTooOld(msg) => msg,
            ApiError::Api {
                status,
                code,
                message,
            } => message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized() {
        let error = ApiError::from_response(
            401,
            r#"{"error":true,"code":"unauthorized","message":"Неверное имя пользователя или ключ"}"#,
        );
        assert!(!error.user_message().is_empty());
    }

    #[test]
    fn too_old() {
        let error = ApiError::from_response(
            426,
            r#"{"error":true,"code":"api_version_unsupported","message":"...","api_version":2,"min_api_version":2}"#,
        );
        assert!(!error.user_message().is_empty());
    }

    #[test]
    fn invalid_api() {
        let error = ApiError::from_response(
            400,
            r#"{"error":true,"code":"invalid_api_version","message":"Заголовок X-Api-Version должен быть целым числом.","api_version":2,"min_api_version":2}"#,
        );
        assert!(!error.user_message().is_empty());
    }

    #[test]
    fn nginx() {
        let error =
            ApiError::from_response(502, r#"<html><body><h1>502 Bad Gateway</h1></body></html>"#);
        assert!(!error.user_message().is_empty());
    }

    #[test]
    fn unauthorized_no_body() {
        let error = ApiError::from_response(401, r#""#);
        assert!(!error.user_message().is_empty());
    }
}
