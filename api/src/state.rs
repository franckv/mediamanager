use std::sync::{Arc, RwLock};

use crate::ripper::{DvdRipper, MockRipper};
use crate::Config;

use mediamanager_model::JobQueue;

pub type SharedState = Arc<RwLock<AppState>>;

pub struct AppState {
    pub config: Arc<Config>,
    pub queue: JobQueue,
    pub dvd_ripper: DvdRipper,
    pub mock_ripper: MockRipper,
}

impl AppState {
    pub fn new(config: Arc<Config>) -> Self {
        let dvd_ripper = DvdRipper::new(config.clone());
        let mock_ripper = MockRipper::new(config.clone());

        AppState {
            config,
            queue: JobQueue::new(),
            dvd_ripper,
            mock_ripper,
        }
    }
}
