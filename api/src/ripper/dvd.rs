use std::io::Result;

use mediamanager_model::Job;

use crate::ripper::Ripper;
use crate::Command;
use crate::Config;

#[derive(Clone)]
pub struct DvdRipper {
    base_dir: String,
    create_dir_cmd: String,
    rip_cmd: String,
    label_cmd: String,
}

impl DvdRipper {
    pub fn new(config: &Config) -> Self {
        DvdRipper {
            base_dir: config.ripper.dvd.base_dir.clone(),
            create_dir_cmd: config.ripper.dvd.create_dir_cmd.clone(),
            rip_cmd: config.ripper.dvd.rip_cmd.clone(),
            label_cmd: config.ripper.dvd.label_cmd.clone(),
        }
    }
}

impl Ripper for DvdRipper {
    fn read_label(&self, job: &Job) -> Result<String> {
        log::debug!("Read label [{}]", job.id);

        let label = Command::run(&self.label_cmd, job)?;

        log::debug!("Label={} [{}]", &label, job.id);

        Ok(label)
    }

    fn output(&self, job: &Job) -> Option<String> {
        log::debug!("Get output [{}]", job.id);

        if let Some(label) = &job.label {
            let output = format!("{}/{}", self.base_dir, label);
            log::debug!("Output={}", &output);
            Some(output)
        } else {
            log::debug!("No output");
            None
        }
    }

    fn create_output(&self, job: &Job) -> Result<()> {
        log::debug!("create_output [{}]", job.id);

        Command::run(&self.create_dir_cmd, job)?;

        Ok(())
    }

    fn rip(&self, job: &Job) -> Result<()> {
        log::debug!("rip [{}]", job.id);

        if let Some(output) = &job.output {
            log::debug!("rip to {} [{}]", output, job.id);
            Command::run(&self.rip_cmd, job)?;
        }

        log::debug!("done ripping [{}]", job.id);

        Ok(())
    }
}
