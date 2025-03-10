use uuid::Uuid;
use crate::domain::repositories::DoctorRepository;
use crate::domain::models::doctor::{Doctor, DoctorStatus};
use crate::domain::error::Error;

pub struct DoctorUseCases<R: DoctorRepository> {
    repository: R,
}

impl<R: DoctorRepository> DoctorUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_doctor(&self, doctor: Doctor) -> Result<Doctor, Error> {
        self.repository.create(doctor).await
    }

    pub async fn update_doctor(&self, doctor: Doctor) -> Result<Doctor, Error> {
        self.repository.update(doctor).await
    }

    pub async fn delete_doctor(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete(id).await
    }

    pub async fn get_doctor(&self, id: Uuid) -> Result<Option<Doctor>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn list_doctors(&self) -> Result<Vec<Doctor>, Error> {
        self.repository.find_all().await
    }

    pub async fn get_doctors_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Doctor>, Error> {
        self.repository.find_by_hospital(hospital_id).await
    }
    
    pub async fn get_doctors_by_speciality(&self, speciality_id: Uuid) -> Result<Vec<Doctor>, Error> {
        self.repository.find_by_speciality(speciality_id).await
    }
    
    pub async fn update_doctor_status(&self, id: Uuid, status: DoctorStatus) -> Result<Doctor, Error> {
        let doctor = self.get_doctor(id).await?.ok_or(Error::NotFound)?;
        let updated_doctor = Doctor {
            status,
            ..doctor
        };
        self.update_doctor(updated_doctor).await
    }
}
