use mediamanager_model::{Job, JobQueue, JobStatus, JobType, QueryJob};

fn generate_queue() -> JobQueue {
    let mut queue = JobQueue::new();

    let job1 = Job::new(JobType::DVD, "/dev/sr0");
    queue.push(job1.clone());

    let mut job2 = Job::new(JobType::DVD, "/dev/sr1");
    job2.status = JobStatus::Running;
    queue.push(job2.clone());

    let job3 = Job::new(JobType::CD, "/dev/sr2");
    queue.push(job3.clone());

    queue
}

#[test]
fn test_find() {
    let mut queue = generate_queue();

    let job = queue.jobs[1].clone();
    let j = queue.find("/dev/sr1");

    assert!(j.is_some());
    assert_eq!(job.id, j.unwrap().id);
}

#[test]
fn test_query() {
    let queue = generate_queue();

    let q = QueryJob {
        id: Some(queue.jobs[0].id),
        status: None,
        typ: None,
        device: None,
    };
    let v = queue.query(q);
    let _: Vec<Job> = v.inspect(|j| assert_eq!(queue.jobs[0].id, j.id)).collect();

    let q = QueryJob {
        id: None,
        status: Some(JobStatus::Running),
        typ: None,
        device: None,
    };
    let v: Vec<Job> = queue.query(q).collect();
    assert_eq!(1, v.len());
    assert_eq!(queue.jobs[1].id, v[0].id);

    let q = QueryJob {
        id: None,
        status: None,
        typ: Some(JobType::CD),
        device: None,
    };
    let v: Vec<Job> = queue.query(q).collect();
    assert_eq!(1, v.len());
    assert_eq!(queue.jobs[2].id, v[0].id);

    let q = QueryJob {
        id: None,
        status: None,
        typ: Some(JobType::DVD),
        device: None,
    };
    let v: Vec<Job> = queue.query(q).collect();
    assert_eq!(2, v.len());

    let q = QueryJob {
        id: None,
        status: None,
        typ: None,
        device: Some("/dev/sr0".to_owned()),
    };
    let v: Vec<Job> = queue.query(q).collect();
    assert_eq!(1, v.len());
    assert_eq!(queue.jobs[0].id, v[0].id);

    let q = QueryJob {
        id: None,
        status: Some(JobStatus::Created),
        typ: Some(JobType::DVD),
        device: None,
    };
    let v: Vec<Job> = queue.query(q).collect();
    assert_eq!(1, v.len());
    assert_eq!(queue.jobs[0].id, v[0].id);

    let q = QueryJob {
        id: None,
        status: Some(JobStatus::Running),
        typ: Some(JobType::CD),
        device: None,
    };
    let v: Vec<Job> = queue.query(q).collect();
    assert_eq!(0, v.len());
}
