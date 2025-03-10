use actix_web::{web, HttpResponse, post, get, put, delete};
use crate::domain::repositories::doctor_repository::DoctorRepository;
use crate::domain::models::doctor::{Doctor, DoctorStatus};
use crate::application::usecases::doctor_use_cases::DoctorUseCases;
use uuid::Uuid;

#[post("/doctors")]
pub async fn create_doctor(
    doctor: web::Json<Doctor>,
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.create_doctor(doctor.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[get("/doctors")]
pub async fn get_doctors(
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.list_doctors().await {
        Ok(doctors) => HttpResponse::Ok().json(doctors),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/doctors/{id}")]
pub async fn get_doctor(
    id: web::Path<Uuid>,
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.get_doctor(id.into_inner()).await {
        Ok(Some(doctor)) => HttpResponse::Ok().json(doctor),
        Ok(None) => HttpResponse::NotFound().json("Doctor not found"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[put("/doctors")]
pub async fn update_doctor(
    doctor: web::Json<Doctor>,
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.update_doctor(doctor.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[delete("/doctors/{id}")]
pub async fn delete_doctor(
    id: web::Path<Uuid>,
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.delete_doctor(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/doctors/hospital/{hospital_id}")]
pub async fn get_doctors_by_hospital(
    hospital_id: web::Path<Uuid>,
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.get_doctors_by_hospital(hospital_id.into_inner()).await {
        Ok(doctors) => HttpResponse::Ok().json(doctors),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/doctors/speciality/{speciality_id}")]
pub async fn get_doctors_by_speciality(
    speciality_id: web::Path<Uuid>,
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.get_doctors_by_speciality(speciality_id.into_inner()).await {
        Ok(doctors) => HttpResponse::Ok().json(doctors),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[put("/doctors/{id}/status")]
pub async fn update_doctor_status(
    id: web::Path<Uuid>,
    status: web::Json<DoctorStatus>,
    use_cases: web::Data<DoctorUseCases<Box<dyn DoctorRepository>>>,
) -> HttpResponse {
    match use_cases.update_doctor_status(id.into_inner(), status.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}