use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use log::info;

pub mod prelude {
    pub use super::auth_middleware;
}

pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = match get_token(&headers) {
        Some(token) => token,
        None => {
            info!("No token provided");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // TODO This is obvously not best practice, just didn't wanna leave this wide open.
    let can_pass = token == "your_api_key";

    if can_pass {
        let response = next.run(request).await;

        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            let parts: Vec<&str> = value.split_whitespace().collect();
            if parts.len() == 2 && parts[0].eq_ignore_ascii_case("Bearer") {
                Some(parts[1])
            } else {
                None
            }
        })
}
