use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::models::doctor::{Doctor, DoctorStatus};

#[async_trait]
pub trait DoctorRepository: Send + Sync {
    async fn create(&self, doctor: Doctor) -> Result<Doctor, Error>;
    async fn update(&self, doctor: Doctor) -> Result<Doctor, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Doctor>, Error>;
    async fn find_all(&self) -> Result<Vec<Doctor>, Error>;
    async fn find_by_speciality(&self, speciality_id: Uuid) -> Result<Vec<Doctor>, Error>;
    async fn find_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Doctor>, Error>;
    async fn find_by_status(&self, status: DoctorStatus) -> Result<Vec<Doctor>, Error>;
}

#[async_trait]
impl DoctorRepository for Box<dyn DoctorRepository> {
    async fn create(&self, doctor: Doctor) -> Result<Doctor, Error> {
        (**self).create(doctor).await
    }
    async fn update(&self, doctor: Doctor) -> Result<Doctor, Error> {
        (**self).update(doctor).await
    }
    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        (**self).delete(id).await
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Doctor>, Error> {
        (**self).find_by_id(id).await
    }
    async fn find_all(&self) -> Result<Vec<Doctor>, Error> {
        (**self).find_all().await
    }
    async fn find_by_speciality(&self, speciality_id: Uuid) -> Result<Vec<Doctor>, Error> {
        (**self).find_by_speciality(speciality_id).await
    }
    async fn find_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Doctor>, Error> {
        (**self).find_by_hospital(hospital_id).await
    }
    async fn find_by_status(&self, status: DoctorStatus) -> Result<Vec<Doctor>, Error> {
        (**self).find_by_status(status).await
    }
}
