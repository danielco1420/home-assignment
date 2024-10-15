use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize)]
pub struct WinnerResponse {
    pub win: bool,
}

#[derive(serde::Serialize)]
pub struct TotalWinsResponse {
    pub total_wins: i64,
}

const EMAIL_REGEX: &str = r"^[^\s@]+@[^\s@]+\.[^\s@]+$";
const SYSTEM_PASSWORD: &str = "r2isthebest";

pub fn generate_new_token(request: LoginRequest) -> Option<String> {
    let email = request.email.trim();
    let password = request.password.trim();

    if regex::Regex::new(EMAIL_REGEX).unwrap().is_match(email) && password == SYSTEM_PASSWORD {
        Some((rand::random::<u32>() % 1_000_000_000).to_string())
    } else {
        None
    }
}

pub fn get_user_token(req: &actix_web::HttpRequest) -> String {
    req.headers().get("Authorization").and_then(|h| h.to_str().ok())
        .filter(|h| h.starts_with("Bearer "))
        .map(|h| h.split_whitespace().nth(1).unwrap().to_string())
        .unwrap_or_default()
}
