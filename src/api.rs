use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use super::databasing;

#[derive(Deserialize)]
pub struct Token {
    token: String,
}

#[derive(Serialize)]
pub struct TokenValidation {
    valid: bool,
}

#[post("/validate_token", format = "json", data = "<token>")]
pub fn validate_token(token: Json<Token>) -> Json<TokenValidation> {
    println!("[API] Reiceived token to validate: {}", token.token);
    Json(TokenValidation { valid: databasing::is_token_valid(&token.token) })
}
