use uuid::Uuid;
use crate::domain::models::speciality::Speciality;
use crate::domain::repositories::Repository;

pub struct SpecialityUseCases<R: Repository<Speciality>> {
    repository: R,
}

impl<R: Repository<Speciality>> SpecialityUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_speciality(&self, speciality: Speciality) -> Result<Speciality, Error> {
        self.repository.create(speciality).await
    }

    pub async fn update_speciality(&self, speciality: Speciality) -> Result<Speciality, Error> {
        self.repository.update(speciality).await
    }

    pub async fn delete_speciality(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete(id).await
    }

    pub async fn get_speciality(&self, id: Uuid) -> Result<Option<Speciality>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn list_specialities(&self) -> Result<Vec<Speciality>, Error> {
        self.repository.find_all().await
    }

    pub async fn get_specialities_by_hospital(&self, hospital_id: Uuid) -> Result<Vec<Speciality>, Error> {
        self.repository.find_by_hospital(hospital_id).await
    }

    pub async fn get_specialities_by_doctor(&self, doctor_id: Uuid) -> Result<Vec<Speciality>, Error> {
        self.repository.find_by_doctor(doctor_id).await
    }
}
