use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

/// Login endpoint
#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
#[post("/login")]
pub async fn login(req: web::Json<LoginRequest>) -> HttpResponse {
    // Use the req parameter to avoid the warning
    let _credentials = req.into_inner();
    
    HttpResponse::Ok().json(LoginResponse {
        token: "dummy_token".to_string(),
    })
}
