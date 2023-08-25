use std::io::Result;
use std::sync::Arc;

use crate::ripper::Ripper;
use crate::Config;

use mediamanager_model::Job;

#[derive(Clone)]
pub struct MockRipper {
    config: Arc<Config>
}

impl MockRipper {
    pub fn new(config: Arc<Config>) -> Self {
        MockRipper {
            config
        }
    }
}

impl Ripper for MockRipper {
    fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    fn read_label(&self, _: &Job) -> Result<String> {
        log::debug!("read_label");
        Ok("mock".to_string())
    }

    fn output(&self, _: &Job) -> Option<String> {
        log::debug!("output");
        Some("/tmp".to_string())
    }

    fn rip(&self, _: &Job) -> Result<()> {
        log::debug!("rip");

        Ok(())
    }
}
