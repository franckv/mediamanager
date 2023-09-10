use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use log;

use mediamanager_model::{CreateJob, QueryJob, Job, JobStatus, JobType};

use crate::ripper::Ripper;
use crate::SharedState;

pub async fn root() -> StatusCode {
    log::debug!("root");

    StatusCode::OK
}

pub async fn get_jobs(Query(query): Query<QueryJob>, State(state): State<SharedState>) -> (StatusCode, Json<Vec<Job>>) {
    log::debug!("get_jobs");

    let jobs: Vec<Job> = state.read().unwrap().queue.query(query).collect();

    (StatusCode::CREATED, Json(jobs))
}

pub async fn create_job(
    State(state): State<SharedState>,
    Json(payload): Json<CreateJob>,
) -> (StatusCode, Json<Job>) {
    log::debug!("create_job");

    let (created, job) = {
        let job = Job::new(payload.typ, &payload.device);

        let queue = &mut state.write().unwrap().queue;
        let (created, job) = queue.push(job);

        (created, job.clone())
    };

    if created {
        log::debug!("Job created: {:?}", job);

        let job_id = job.id;
        let ripper = match job.typ {
            JobType::DVD => state.read().unwrap().dvd_ripper.clone(),
            _ => state.read().unwrap().dvd_ripper.clone(),
        };

        tokio::task::spawn_blocking(move || {
            if let Err(err) = ripper.process(state.clone(), job_id) {
                log::error!("Ripping failed: {} [{}]", err, job_id);
                if let Some(job) = &mut state.write().unwrap().queue.get(job_id) {
                    job.status = JobStatus::Error;
                }
            };
        });

        log::debug!("Job running [{}]", job_id);
        (StatusCode::CREATED, Json(job.clone()))
    } else {
        log::debug!("Job already exists: {:?}", job);
        (StatusCode::CONFLICT, Json(job))
    }
}

pub async fn clear_jobs(State(state): State<SharedState>) -> StatusCode {
    log::debug!("clear_jobs");

    let queue = &mut state.write().unwrap().queue;
    queue.clear();

    StatusCode::OK
}
