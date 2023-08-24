use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobType {
    CD,
    DVD,
    BR,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobStatus {
    Created,
    Running,
    Stopped,
    Error,
}

#[derive(Deserialize)]
pub struct CreateJob {
    pub typ: JobType,
    pub device: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Job {
    pub id: Uuid,
    pub status: JobStatus,
    pub label: Option<String>,
    pub typ: JobType,
    pub device: String,
    pub output: Option<String>,
}

impl Job {
    pub fn new(typ: JobType, device: &str) -> Self {
        Job {
            id: Uuid::new_v4(),
            status: JobStatus::Created,
            label: None,
            typ,
            device: device.to_owned(),
            output: None,
        }
    }
}
