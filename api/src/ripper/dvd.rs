use std::io::{Error, ErrorKind, Result};
use std::sync::Arc;

use mediamanager_model::Job;

use crate::ripper::Ripper;
use crate::Command;
use crate::Config;

#[derive(Clone)]
pub struct DvdRipper {
    config: Arc<Config>
}

impl DvdRipper {
    pub fn new(config: Arc<Config>) -> Self {
        DvdRipper {
            config
        }
    }
}

impl Ripper for DvdRipper {
    fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    fn read_label(&self, job: &Job) -> Result<String> {
        log::debug!("Read label [{}]", job.id);

        let label = Command::run(&self.config.ripper.dvd.label_cmd, job)?;

        log::debug!("Label={} [{}]", &label, job.id);

        if label.is_empty() {
            Err(Error::from(ErrorKind::NotFound))
        } else {
            Ok(label)
        }
    }

    fn output(&self, job: &Job) -> Option<String> {
        log::debug!("Get output [{}]", job.id);

        if let Some(label) = &job.label {
            let output = format!("{}/{}", self.config.library.base_dir, label);
            log::debug!("Output={}", &output);
            Some(output)
        } else {
            log::debug!("No output");
            None
        }
    }

    fn rip(&self, job: &Job) -> Result<()> {
        log::debug!("rip [{}]", job.id);

        if let Some(output) = &job.output {
            log::debug!("rip to {} [{}]", output, job.id);
            Command::run(&self.config.ripper.dvd.rip_cmd, job)?;
        }

        log::debug!("done ripping [{}]", job.id);

        Ok(())
    }
}
