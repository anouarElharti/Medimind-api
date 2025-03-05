use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::models::speciality::Speciality;

#[async_trait]
pub trait SpecialityRepository {
    async fn create(&self, speciality: Speciality) -> Result<Speciality, Error>;
    async fn update(&self, speciality: Speciality) -> Result<Speciality, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Speciality>, Error>;
    async fn find_all(&self) -> Result<Vec<Speciality>, Error>;
    async fn find_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Speciality>, Error>;
    async fn find_by_doctor(&self, doctor_id: Uuid) -> Result<Vec<Speciality>, Error>;
}
