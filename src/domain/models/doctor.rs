use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Doctor {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: DoctorStatus,
    pub specialities: Vec<Uuid>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub situation: MaritalStatus,
    pub gender: Gender,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DoctorStatus {
    Present,
    NotPresent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MaritalStatus {
    Married,
    Divorced,
    Widow,
    Single,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
}
