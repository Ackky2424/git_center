use serde_json::json;

pub struct ApiError {
    code: i32,
    description: String
}

pub struct Api {
    token: String
}

impl Api {
    /// 指定したURIでAPIコールを行う
    fn call(uri: &String) -> Result<serde_json, ApiError> {
        Ok(json!(""))
    }

    ///
    fn auth() -> Result<serde_json, ApiError> {
        Ok(json!(""))
    }

    /// APIコール時に使用するトークンを設定する
    pub fn set_token(token: String) {
        Self.token = token;
    }
}