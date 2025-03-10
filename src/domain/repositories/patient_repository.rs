use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::models::patient::Patient;

#[async_trait]
pub trait PatientRepository: Send + Sync {
    async fn create(&self, patient: Patient) -> Result<Patient, Error>;
    async fn update(&self, patient: Patient) -> Result<Patient, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Patient>, Error>;
    async fn find_all(&self) -> Result<Vec<Patient>, Error>;
    async fn find_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Patient>, Error>;
    async fn find_by_doctor(&self, doctor_id: Uuid) -> Result<Vec<Patient>, Error>;
}

#[async_trait]
impl PatientRepository for Box<dyn PatientRepository> {
    async fn create(&self, patient: Patient) -> Result<Patient, Error> {
        (**self).create(patient).await
    }
    async fn update(&self, patient: Patient) -> Result<Patient, Error> {
        (**self).update(patient).await
    }
    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        (**self).delete(id).await
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Patient>, Error> {
        (**self).find_by_id(id).await
    }
    async fn find_all(&self) -> Result<Vec<Patient>, Error> {
        (**self).find_all().await
    }
    async fn find_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Patient>, Error> {
        (**self).find_by_hospital(hospital_id).await
    }
    async fn find_by_doctor(&self, doctor_id: Uuid) -> Result<Vec<Patient>, Error> {
        (**self).find_by_doctor(doctor_id).await
    }
}
