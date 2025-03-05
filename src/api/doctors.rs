use actix_web::{web, HttpResponse, post, get};
use utoipa::OpenApi;
use crate::domain::models::doctor::Doctor;
use crate::domain::repositories::DoctorRepository;
use crate::application::use_cases::doctor_use_cases::DoctorUseCases;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/v1/doctors",
    tag = "doctors",
    request_body = Doctor,
    responses(
        (status = 201, description = "Doctor created successfully", body = Doctor),
        (status = 400, description = "Invalid input")
    )
)]
#[post("/doctors")]
pub async fn create_doctor(
    doctor: web::Json<Doctor>,
    use_cases: web::Data<DoctorUseCases<impl DoctorRepository>>,
) -> HttpResponse {
    match use_cases.create_doctor(doctor.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/doctors",
    tag = "doctors",
    responses(
        (status = 200, description = "List of doctors", body = Vec<Doctor>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/doctors")]
pub async fn get_doctors(
    use_cases: web::Data<DoctorUseCases<impl DoctorRepository>>,
) -> HttpResponse {
    match use_cases.list_doctors().await {
        Ok(doctors) => HttpResponse::Ok().json(doctors),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/doctors/{id}",
    tag = "doctors",
    responses(
        (status = 200, description = "Doctor found", body = Doctor),
        (status = 404, description = "Doctor not found")
    )
)]
#[get("/doctors/{id}")]
pub async fn get_doctor(
    id: web::Path<Uuid>,
    use_cases: web::Data<DoctorUseCases<impl DoctorRepository>>,
) -> HttpResponse {
    match use_cases.get_doctor(id.into_inner()).await {
        Ok(Some(doctor)) => HttpResponse::Ok().json(doctor),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
