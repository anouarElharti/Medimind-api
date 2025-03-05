use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::models::hospital::Hospital;

#[async_trait]
pub trait HospitalRepository {
    async fn create(&self, hospital: Hospital) -> Result<Hospital, Error>;
    async fn update(&self, hospital: Hospital) -> Result<Hospital, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Hospital>, Error>;
    async fn find_all(&self) -> Result<Vec<Hospital>, Error>;
    async fn find_by_speciality(&self, speciality_id: Uuid) -> Result<Vec<Hospital>, Error>;
    async fn find_by_doctor(&self, doctor_id: Uuid) -> Result<Vec<Hospital>, Error>;
    async fn find_by_patient(&self, patient_id: Uuid) -> Result<Vec<Hospital>, Error>;
}
