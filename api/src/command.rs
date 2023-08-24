use std::io::Result;
use std::process;
use std::str;

use regex::Regex;

use mediamanager_model::Job;

pub struct Command;

impl Command {
    pub fn run(cmd: &str, job: &Job) -> Result<String> {
        let command = &Self::process_params(cmd, job);
        log::debug!("Run cmd: {} [{}]", &command, job.id);

        let output = process::Command::new("sh").args(["-c", command]).output()?;

        let stdout = str::from_utf8(&output.stdout).unwrap();
        let stderr = str::from_utf8(&output.stderr).unwrap();

        log::debug!("Cmd stout={}, stderr={}", stdout, stderr);

        Ok(stdout.into())
    }

    fn process_params(cmd: &str, job: &Job) -> String {
        let re = Regex::new(r"^sr([0-9])$").unwrap();

        let idx = {
            if let Some(cap) = re.captures(&job.device) {
                cap[1].to_owned()
            } else {
                "0".to_owned()
            }
        };

        log::debug!("idx={} [{}]", &idx, job.id);

        let cmd = cmd
            .replace("%{idx}", &idx)
            .replace("%{device}", &job.device)
            .replace("%{device_f}", &format!("/dev/{}", &job.device));

        if let Some(output) = &job.output {
            cmd.replace("%{output}", output)
        } else {
            cmd
        }
    }
}
