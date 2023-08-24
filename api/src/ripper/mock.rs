use std::io::Result;

use crate::ripper::Ripper;
use crate::Config;

use mediamanager_model::Job;

#[derive(Clone)]
pub struct MockRipper {}

impl MockRipper {
    pub fn new(_: &Config) -> Self {
        MockRipper {}
    }
}

impl Ripper for MockRipper {
    fn read_label(&self, _: &Job) -> Result<String> {
        log::debug!("read_label");
        Ok("mock".to_string())
    }

    fn output(&self, _: &Job) -> Option<String> {
        log::debug!("output");
        Some("/tmp".to_string())
    }

    fn create_output(&self, _: &Job) -> Result<()> {
        log::debug!("create_output");

        Ok(())
    }

    fn rip(&self, _: &Job) -> Result<()> {
        log::debug!("rip");

        Ok(())
    }
}
