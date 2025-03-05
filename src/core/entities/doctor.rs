use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Doctor {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: DoctorStatus,
    pub specialities: Vec<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub situation: MaritalStatus,
    pub gender: Gender,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DoctorStatus {
    Present,
    NotPresent,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaritalStatus {
    Married,
    Divorced,
    Widow,
    Single,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}