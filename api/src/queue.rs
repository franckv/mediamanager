use uuid::Uuid;

use mediamanager_model::{Job, JobStatus};

pub struct JobQueue {
    pub jobs: Vec<Job>,
}

impl JobQueue {
    pub fn new() -> Self {
        JobQueue { jobs: Vec::new() }
    }

    pub fn get(&mut self, id: Uuid) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }

    pub fn find(&mut self, device: &str) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.device == device)
    }

    pub fn push(&mut self, job: Job) -> (bool, &mut Job) {
        let exists = self.jobs.iter().any(|j| {
            j.device == job.device && j.status != JobStatus::Error && j.status != JobStatus::Stopped
        });

        let job = if exists {
            self.find(&job.device).unwrap()
        } else {
            self.jobs.push(job);
            self.jobs.iter_mut().last().unwrap()
        };

        (!exists, job)
    }

    pub fn clear(&mut self) {
        log::info!("Clear queue ({})", self.jobs.len());
        self.jobs.retain(|j| j.status != JobStatus::Stopped);
        log::info!("Queue cleared ({})", self.jobs.len());
    }
}
