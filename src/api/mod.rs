use actix_web::web;
use crate::api::{hospitals::*, patients::*, specialities::*, doctors::*};

pub mod hospitals;
pub mod patients;
pub mod specialities;
pub mod doctors;
pub mod auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Hospitals
            .service(create_hospital)
            .service(get_hospitals)
            .service(get_hospital)
            .service(update_hospital)
            .service(delete_hospital)
            .service(get_hospitals_by_speciality)
            // Patients
            .service(create_patient)
            .service(get_patients)
            .service(get_patient)
            .service(update_patient)
            .service(delete_patient)
            .service(get_patients_by_hospital)
            .service(get_patients_by_doctor)
            // Specialities
            .service(create_speciality)
            .service(get_specialities)
            .service(get_speciality)
            .service(update_speciality)
            .service(delete_speciality)
            // Doctors
            .service(create_doctor)
            .service(get_doctors)
            .service(get_doctor)
            .service(update_doctor)
            .service(delete_doctor)
            .service(get_doctors_by_hospital)
            .service(get_doctors_by_speciality)
            .service(update_doctor_status)
            // Auth
            .service(auth::login)
    );
}