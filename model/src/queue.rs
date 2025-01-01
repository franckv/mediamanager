use uuid::Uuid;

use crate::{Job, JobStatus, QueryJob};

#[derive(Debug)]
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

    pub fn query(&self, query: QueryJob) -> impl Iterator<Item = Job> + '_ {
        self.jobs
            .iter()
            .filter(move |&j| {
                if let Some(typ) = query.typ {
                    if j.typ != typ {
                        return false;
                    }
                }
                if let Some(status) = query.status {
                    if j.status != status {
                        return false;
                    }
                }
                if let Some(id) = query.id {
                    if j.id != id {
                        return false;
                    }
                }
                if let Some(device) = &query.device {
                    if j.device != *device {
                        return false;
                    }
                }
                true
            })
            .cloned()
    }

    pub fn clear(&mut self) {
        self.jobs.retain(|j| j.status != JobStatus::Stopped);
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}
