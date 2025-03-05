use actix_web::{web, HttpResponse, post, get};
use utoipa::OpenApi;
use crate::domain::models::hospital::Hospital;
use crate::domain::repositories::HospitalRepository;
use crate::application::use_cases::hospital_use_cases::HospitalUseCases;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/v1/hospitals",
    tag = "hospitals",
    request_body = Hospital,
    responses(
        (status = 201, description = "Hospital created successfully", body = Hospital),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/hospitals")]
pub async fn create_hospital(
    hospital: web::Json<Hospital>,
    use_cases: web::Data<HospitalUseCases<impl HospitalRepository>>,
) -> HttpResponse {
    match use_cases.create_hospital(hospital.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/hospitals",
    tag = "hospitals",
    responses(
        (status = 200, description = "List of hospitals", body = Vec<Hospital>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/hospitals")]
pub async fn get_hospitals(
    use_cases: web::Data<HospitalUseCases<impl HospitalRepository>>,
) -> HttpResponse {
    match use_cases.list_hospitals().await {
        Ok(hospitals) => HttpResponse::Ok().json(hospitals),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/hospitals/{id}",
    tag = "hospitals",
    responses(
        (status = 200, description = "Hospital found", body = Hospital),
        (status = 404, description = "Hospital not found")
    )
)]
#[get("/hospitals/{id}")]
pub async fn get_hospital(
    id: web::Path<Uuid>,
    use_cases: web::Data<HospitalUseCases<impl HospitalRepository>>,
) -> HttpResponse {
    match use_cases.get_hospital(id.into_inner()).await {
        Ok(Some(hospital)) => HttpResponse::Ok().json(hospital),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
