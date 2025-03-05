use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Speciality {
    pub uuid: Uuid,
    pub name: String,
    pub hospitals: Vec<Uuid>,
    pub doctors: Vec<Uuid>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
