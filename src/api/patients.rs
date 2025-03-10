use uuid::Uuid;
use actix_web::{get, post, put, delete, web, HttpResponse};
use crate::application::usecases::patient_use_cases::PatientUseCases;
use crate::domain::models::patient::Patient;
use crate::domain::repositories::patient_repository::PatientRepository;

#[post("/patients")]
pub async fn create_patient(
    patient: web::Json<Patient>,
    use_cases: web::Data<PatientUseCases<Box<dyn PatientRepository>>>,
) -> HttpResponse {
    match use_cases.create_patient(patient.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[get("/patients")]
pub async fn get_patients(
    use_cases: web::Data<PatientUseCases<Box<dyn PatientRepository>>>,
) -> HttpResponse {
    match use_cases.list_patients().await {
        Ok(patients) => HttpResponse::Ok().json(patients),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/patients/{id}")]
pub async fn get_patient(
    id: web::Path<Uuid>,
    use_cases: web::Data<PatientUseCases<Box<dyn PatientRepository>>>,
) -> HttpResponse {
    match use_cases.get_patient(id.into_inner()).await {
        Ok(Some(patient)) => HttpResponse::Ok().json(patient),
        Ok(None) => HttpResponse::NotFound().json("Patient not found"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[put("/patients")]
pub async fn update_patient(
    patient: web::Json<Patient>,
    use_cases: web::Data<PatientUseCases<Box<dyn PatientRepository>>>,
) -> HttpResponse {
    match use_cases.update_patient(patient.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[delete("/patients/{id}")]
pub async fn delete_patient(
    id: web::Path<Uuid>,
    use_cases: web::Data<PatientUseCases<Box<dyn PatientRepository>>>,
) -> HttpResponse {
    match use_cases.delete_patient(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/patients/hospital/{hospital_id}")]
pub async fn get_patients_by_hospital(
    hospital_id: web::Path<Uuid>,
    use_cases: web::Data<PatientUseCases<Box<dyn PatientRepository>>>,
) -> HttpResponse {
    match use_cases.get_patients_by_hospital(hospital_id.into_inner()).await {
        Ok(patients) => HttpResponse::Ok().json(patients),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/patients/doctor/{doctor_id}")]
pub async fn get_patients_by_doctor(
    doctor_id: web::Path<Uuid>,
    use_cases: web::Data<PatientUseCases<Box<dyn PatientRepository>>>,
) -> HttpResponse {
    match use_cases.get_patients_by_doctor(doctor_id.into_inner()).await {
        Ok(patients) => HttpResponse::Ok().json(patients),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}
