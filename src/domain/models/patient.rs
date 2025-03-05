use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::models::doctor::MaritalStatus;
use crate::domain::models::doctor::Gender;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Patient {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub profession: String,
    pub situation: MaritalStatus,
    pub gender: Gender,
    pub doctors: Vec<Uuid>,
    pub hospitals: Vec<Uuid>,
    pub history: Vec<MedicalHistory>,
    pub current_treatment: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalHistory {
    pub uuid: Uuid,
    pub date: DateTime<Utc>,
    pub description: String,
    pub treatment: String,
}
