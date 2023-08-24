pub mod dvd;
pub mod mock;

use std::fs;
use std::io::{Error, ErrorKind, Result};

use uuid::Uuid;

use mediamanager_model::{Job, JobStatus};

use crate::Command;
use crate::SharedState;

pub use dvd::DvdRipper;
pub use mock::MockRipper;

pub trait Ripper {
    fn read_label(&self, job: &Job) -> Result<String>;
    fn output(&self, job: &Job) -> Option<String>;
    fn rip(&self, job: &Job) -> Result<()>;

    fn process(&self, state: SharedState, job_id: Uuid) -> Result<()> {
        log::debug!("Get job [{}]", job_id);
        let mut job = get_job(state.clone(), job_id).ok_or(Error::from(ErrorKind::NotFound))?;

        log::debug!("Start Job [{}]", job_id);
        job.status = JobStatus::Running;
        update_job(state.clone(), &job)?;

        log::debug!("Read label [{}]", job_id);
        let label = self.read_label(&job)?;
        job.label = Some(label);
        update_job(state.clone(), &job)?;

        log::debug!("Get output [{}]", job_id);
        let output = self.output(&job).ok_or(Error::from(ErrorKind::NotFound))?;

        log::debug!("Create output directory [{}]", job_id);
        fs::create_dir_all(&output)?;

        log::debug!("Set output to {} [{}]", &output, job_id);
        job.output = Some(output);
        update_job(state.clone(), &job)?;

        log::debug!("Start ripping [{}]", job_id);
        self.rip(&job)?;

        log::debug!("Job completed [{}]", job_id);
        job.status = JobStatus::Stopped;
        update_job(state.clone(), &job)?;

        log::debug!("Eject [{}]", job_id);
        self.eject(&job)?;

        Ok(())
    }

    fn eject(&self, job: &Job) -> Result<()> {
        Command::run("eject %{device_f}", &job)?;

        Ok(())
    }
}

fn get_job(state: SharedState, job_id: Uuid) -> Option<Job> {
    let queue = &mut state.write().unwrap().queue;
    if let Some(job) = queue.get(job_id) {
        Some(job.clone())
    } else {
        None
    }
}

fn update_job(state: SharedState, update: &Job) -> Result<()> {
    let queue = &mut state.write().unwrap().queue;
    let job = queue
        .get(update.id)
        .ok_or(Error::from(ErrorKind::NotFound))?;

    job.status = update.status;
    job.label = update.label.clone();
    job.output = update.output.clone();

    Ok(())
}
