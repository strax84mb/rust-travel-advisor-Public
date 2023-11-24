use actix_web::{HttpRequest, http::header::ToStrError};

pub fn string_to_id(str: String) -> Result<i64, String> {
    let n = match str.parse::<i64>() {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    if n <= 0 {
        return Err("must be a positive number".to_string());
    }

    Ok(n)
}

pub fn extract_auth(req: &HttpRequest) -> Option<Result<&str, ToStrError>> {
    match req.headers().get(actix_web::http::header::AUTHORIZATION) {
        Some(header) => Some(header.to_str()),
        None => None,
    }
}