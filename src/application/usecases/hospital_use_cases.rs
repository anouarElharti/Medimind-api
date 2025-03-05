use uuid::Uuid;
use crate::domain::models::hospital::Hospital;
use crate::domain::repositories::Repository;

pub struct HospitalUseCases<R: Repository<Hospital>> {
    repository: R,
}

impl<R: Repository<Hospital>> HospitalUseCases<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_hospital(&self, hospital: Hospital) -> Result<Hospital, Error> {
        self.repository.create(hospital).await
    }

    pub async fn update_hospital(&self, hospital: Hospital) -> Result<Hospital, Error> {
        self.repository.update(hospital).await
    }

    pub async fn delete_hospital(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete(id).await
    }

    pub async fn get_hospital(&self, id: Uuid) -> Result<Option<Hospital>, Error> {
        self.repository.find_by_id(id).await
    }

    pub async fn list_hospitals(&self) -> Result<Vec<Hospital>, Error> {
        self.repository.find_all().await
    }

    pub async fn get_hospitals_by_speciality(&self, speciality_id: Uuid) -> Result<Vec<Hospital>, Error> {
        self.repository.find_by_speciality(speciality_id).await
    }
}
