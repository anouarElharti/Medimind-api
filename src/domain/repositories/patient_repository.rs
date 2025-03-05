use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::models::patient::Patient;

#[async_trait]
pub trait PatientRepository {
    async fn create(&self, patient: Patient) -> Result<Patient, Error>;
    async fn update(&self, patient: Patient) -> Result<Patient, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Patient>, Error>;
    async fn find_all(&self) -> Result<Vec<Patient>, Error>;
    async fn find_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Patient>, Error>;
    async fn find_by_doctor(&self, doctor_id: Uuid) -> Result<Vec<Patient>, Error>;
    async fn find_by_treatment(&self, treatment: &str) -> Result<Vec<Patient>, Error>;
}
