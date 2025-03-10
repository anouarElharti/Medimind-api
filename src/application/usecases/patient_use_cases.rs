use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::models::patient::Patient;
use crate::domain::repositories::PatientRepository;

pub struct PatientUseCases<R: PatientRepository> {
    repository: R,
}

impl<R: PatientRepository> PatientUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_patient(&self, patient: Patient) -> Result<Patient, Error> {
        self.repository.create(patient).await
    }

    pub async fn update_patient(&self, patient: Patient) -> Result<Patient, Error> {
        self.repository.update(patient).await
    }

    pub async fn delete_patient(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete(id).await
    }

    pub async fn get_patient(&self, id: Uuid) -> Result<Option<Patient>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn list_patients(&self) -> Result<Vec<Patient>, Error> {
        self.repository.find_all().await
    }

    pub async fn get_patients_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Patient>, Error> {
        self.repository.find_by_hospital(hospital_id).await
    }

    pub async fn get_patients_by_doctor(&self, doctor_id: Uuid) -> Result<Vec<Patient>, Error> {
        self.repository.find_by_doctor(doctor_id).await
    }
}
