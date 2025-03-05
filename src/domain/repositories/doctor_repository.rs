use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::models::Doctor;

#[async_trait]
pub trait DoctorRepository {
    async fn create(&self, doctor: Doctor) -> Result<Doctor, Error>;
    async fn update(&self, doctor: Doctor) -> Result<Doctor, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Doctor>, Error>;
    async fn find_all(&self) -> Result<Vec<Doctor>, Error>;
    async fn find_by_speciality(&self, speciality_id: Uuid) -> Result<Vec<Doctor>, Error>;
    async fn find_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Doctor>, Error>;
    async fn find_by_status(&self, status: DoctorStatus) -> Result<Vec<Doctor>, Error>;
}
