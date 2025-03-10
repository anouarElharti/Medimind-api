use uuid::Uuid;
use actix_web::{web, HttpResponse, post, get, put, delete};
use crate::domain::models::hospital::Hospital;
use crate::domain::repositories::hospital_repository::HospitalRepository;
use crate::application::usecases::hospital_use_cases::HospitalUseCases;

#[post("/hospitals")]
pub async fn create_hospital(
    hospital: web::Json<Hospital>,
    use_cases: web::Data<HospitalUseCases<Box<dyn HospitalRepository>>>,
) -> HttpResponse {
    match use_cases.create_hospital(hospital.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[get("/hospitals")]
pub async fn get_hospitals(
    use_cases: web::Data<HospitalUseCases<Box<dyn HospitalRepository>>>,
) -> HttpResponse {
    match use_cases.list_hospitals().await {
        Ok(hospitals) => HttpResponse::Ok().json(hospitals),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/hospitals/{id}")]
pub async fn get_hospital(
    id: web::Path<Uuid>,
    use_cases: web::Data<HospitalUseCases<Box<dyn HospitalRepository>>>,
) -> HttpResponse {
    match use_cases.get_hospital(id.into_inner()).await {
        Ok(Some(hospital)) => HttpResponse::Ok().json(hospital),
        Ok(None) => HttpResponse::NotFound().json("Hospital not found"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[put("/hospitals")]
pub async fn update_hospital(
    hospital: web::Json<Hospital>,
    use_cases: web::Data<HospitalUseCases<Box<dyn HospitalRepository>>>,
) -> HttpResponse {
    match use_cases.update_hospital(hospital.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[delete("/hospitals/{id}")]
pub async fn delete_hospital(
    id: web::Path<Uuid>,
    use_cases: web::Data<HospitalUseCases<Box<dyn HospitalRepository>>>,
) -> HttpResponse {
    match use_cases.delete_hospital(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/hospitals/speciality/{speciality_id}")]
pub async fn get_hospitals_by_speciality(
    speciality_id: web::Path<Uuid>,
    use_cases: web::Data<HospitalUseCases<Box<dyn HospitalRepository>>>,
) -> HttpResponse {
    match use_cases.get_hospitals_by_speciality(speciality_id.into_inner()).await {
        Ok(hospitals) => HttpResponse::Ok().json(hospitals),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}
