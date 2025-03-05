use actix_web::web;
use utoipa::OpenApi;
use crate::domain::models::{
    hospital::Hospital,
    patient::Patient,
    speciality::Speciality,
    doctor::Doctor,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::hospitals::create_hospital,
        crate::api::hospitals::get_hospitals,
        crate::api::hospitals::get_hospital,
        crate::api::patients::create_patient,
        crate::api::patients::get_patients,
        crate::api::patients::get_patient,
        crate::api::specialities::create_speciality,
        crate::api::specialities::get_specialities,
        crate::api::specialities::get_speciality,
        crate::api::doctors::create_doctor,
        crate::api::doctors::get_doctors,
        crate::api::doctors::get_doctor
    ),
    components(
        schemas(Hospital, Patient, Speciality, Doctor)
    ),
    tags(
        (name = "Medimind API", description = "Medical Management System API")
    )
)]
pub struct ApiDoc;

pub mod hospitals;
pub mod patients;
pub mod specialities;
pub mod doctors;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(hospitals::create_hospital)
            .service(hospitals::get_hospitals)
            .service(hospitals::get_hospital)
            .service(patients::create_patient)
            .service(patients::get_patients)
            .service(patients::get_patient)
            .service(specialities::create_speciality)
            .service(specialities::get_specialities)
            .service(specialities::get_speciality)
            .service(doctors::create_doctor)
            .service(doctors::get_doctors)
            .service(doctors::get_doctor)
    );
}
