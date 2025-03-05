use actix_web::{web, HttpResponse, post, get};
use utoipa::OpenApi;
use crate::domain::models::speciality::Speciality;
use crate::domain::repositories::SpecialityRepository;
use crate::application::use_cases::speciality_use_cases::SpecialityUseCases;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/v1/specialities",
    tag = "specialities",
    request_body = Speciality,
    responses(
        (status = 201, description = "Speciality created successfully", body = Speciality),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/specialities")]
pub async fn create_speciality(
    speciality: web::Json<Speciality>,
    use_cases: web::Data<SpecialityUseCases<impl SpecialityRepository>>,
) -> HttpResponse {
    match use_cases.create_speciality(speciality.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/specialities",
    tag = "specialities",
    responses(
        (status = 200, description = "List of specialities", body = Vec<Speciality>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/specialities")]
pub async fn get_specialities(
    use_cases: web::Data<SpecialityUseCases<impl SpecialityRepository>>,
) -> HttpResponse {
    match use_cases.list_specialities().await {
        Ok(specialities) => HttpResponse::Ok().json(specialities),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/specialities/{id}",
    tag = "specialities",
    responses(
        (status = 200, description = "Speciality found", body = Speciality),
        (status = 404, description = "Speciality not found")
    )
)]
#[get("/specialities/{id}")]
pub async fn get_speciality(
    id: web::Path<Uuid>,
    use_cases: web::Data<SpecialityUseCases<impl SpecialityRepository>>,
) -> HttpResponse {
    match use_cases.get_speciality(id.into_inner()).await {
        Ok(Some(speciality)) => HttpResponse::Ok().json(speciality),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
