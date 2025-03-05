use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hospital {
    pub uuid: Uuid,
    pub name: String,
    pub logo: Option<String>,
    pub address: String,
    pub postal_code: u32,
    pub city: String,
    pub country: String,
    pub doctors: Vec<Uuid>,
    pub patients: Vec<Uuid>,
    pub specialities: Vec<Uuid>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
