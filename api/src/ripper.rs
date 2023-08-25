pub mod dvd;
pub mod mock;

use std::io::{Error, ErrorKind, Result};
use std::os::fd::AsRawFd;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;

use nix::ioctl_none_bad;

use uuid::Uuid;

use mediamanager_model::{Job, JobStatus};

use crate::Command;
use crate::Config;
use crate::SharedState;

pub use dvd::DvdRipper;
pub use mock::MockRipper;

#[derive(Debug)]
pub enum DriveStatus {
    None,
    Empty,
    Open,
    NotReady,
    Ready
}

ioctl_none_bad!(cdrom_drive_status, 0x5326);

pub trait Ripper {
    fn config(&self) -> Arc<Config>;
    fn read_label(&self, job: &Job) -> Result<String>;
    fn output(&self, job: &Job) -> Option<String>;
    fn rip(&self, job: &Job) -> Result<()>;

    fn process(&self, state: SharedState, job_id: Uuid) -> Result<()> {
        log::info!("Processing job [{}]", job_id);

        log::debug!("Get job [{}]", job_id);
        let mut job = get_job(state.clone(), job_id).ok_or(Error::from(ErrorKind::NotFound))?;

        log::debug!("Start Job [{}]", job_id);
        job.status = JobStatus::Running;
        update_job(state.clone(), &job)?;

        log::debug!("Get drive status [{}]", job_id);
        self.get_drive_status(&job)?;

        log::debug!("Read label [{}]", job_id);
        let label = self.read_label(&job)?;
        job.label = Some(label);
        update_job(state.clone(), &job)?;

        log::debug!("Get output [{}]", job_id);
        let output = self.output(&job).ok_or(Error::from(ErrorKind::NotFound))?;

        log::debug!("Set output to {} [{}]", &output, job_id);
        job.output = Some(output);
        update_job(state.clone(), &job)?;

        log::debug!("Create output directory [{}]", job_id);
        self.create_output(&job)?;

        log::debug!("Start ripping [{}]", job_id);
        self.rip(&job)?;

        log::debug!("Job completed [{}]", job_id);
        job.status = JobStatus::Stopped;
        update_job(state.clone(), &job)?;

        log::debug!("Eject [{}]", job_id);
        self.eject(&job)?;

        log::info!("Job processed [{}]", job_id);

        Ok(())
    }

    fn create_output(&self, job: &Job) -> Result<()> {
        log::debug!("create_output [{}]", job.id);

        Command::run(&self.config().ripper.create_dir_cmd, job)?;

        Ok(())
    }

    fn get_drive_status(&self, job: &Job) -> Result<DriveStatus> {
        log::debug!("Reading device status {} [{}]", &job.device, job.id);

        let device = format!("/dev/{}", job.device);

        let fd = OpenOptions::new()
            .read(true)
            .custom_flags(nix::libc::O_NONBLOCK)
            .open(&device)?;

        let res = unsafe { cdrom_drive_status(fd.as_raw_fd()).unwrap() };

        let status = match res {
            0 => DriveStatus::None,
            1 => DriveStatus::Empty,
            2 => DriveStatus::Open,
            3 => DriveStatus::NotReady,
            4 => DriveStatus::Ready,
            _ => DriveStatus::None
        };

        log::debug!("ioctl={}, status={:?} [{}]", res, status, job.id);

        Ok(status)
    }

    fn eject(&self, job: &Job) -> Result<()> {
        if self.config().ripper.eject {
            Command::run("eject %{device_f}", &job)?;
        }

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
