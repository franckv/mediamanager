use std::sync::{Arc, RwLock};

use crate::ripper::{DvdRipper, MockRipper, Ripper};
use crate::Config;

use mediamanager_model::JobQueue;

pub type SharedState = Arc<RwLock<AppState>>;

pub struct AppState {
    pub config: Arc<Config>,
    pub queue: JobQueue,
    pub ripper: Arc<dyn Ripper>,
}

impl AppState {
    pub fn new(config: Arc<Config>) -> SharedState {
        let ripper = if config.ripper.mock {
            MockRipper::new(config.clone())
        } else {
            DvdRipper::new(config.clone())
        };

        Arc::new(RwLock::new(AppState {
            config,
            queue: JobQueue::new(),
            ripper,
        }))
    }
}
