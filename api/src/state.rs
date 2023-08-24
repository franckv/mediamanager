use std::sync::{Arc, RwLock};

use crate::ripper::{DvdRipper, MockRipper};
use crate::Config;
use crate::JobQueue;

pub type SharedState = Arc<RwLock<AppState>>;

pub struct AppState {
    pub config: Config,
    pub queue: JobQueue,
    pub dvd_ripper: DvdRipper,
    pub mock_ripper: MockRipper,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let dvd_ripper = DvdRipper::new(&config);
        let mock_ripper = MockRipper::new(&config);

        AppState {
            config,
            queue: JobQueue::new(),
            dvd_ripper,
            mock_ripper,
        }
    }
}
