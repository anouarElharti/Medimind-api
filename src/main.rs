mod api;

use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::auth::login
    ),
    components(
        schemas(api::auth::LoginRequest, api::auth::LoginResponse)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    
    HttpServer::new(move || {
        App::new()
            .configure(api::config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}