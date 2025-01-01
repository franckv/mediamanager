use std::io::{BufRead, BufReader, Result};
use std::process;
use std::str;

use mediamanager_model::Job;

pub struct Command;

impl Command {
    pub fn run(cmd: &str, job: &Job) -> Result<String> {
        let command = &Self::process_params(cmd, job);
        log::debug!("Run cmd: {} [{}]", &command, job.id);

        let mut output = process::Command::new("sh")
            .args(["-c", command])
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()?;

        let mut stdout = Vec::new();

        let stdout = {
            let reader = BufReader::new(output.stdout.as_mut().unwrap());

            for line in reader.lines() {
                let line = line?;
                log::debug!("Cmd stdout={} [{}]", &line, job.id);
                stdout.push(line);
            }

            stdout.join("\n")
        };

        {
            let reader = BufReader::new(output.stderr.as_mut().unwrap());

            for line in reader.lines() {
                log::debug!("Cmd stderr={} [{}]", &line?, job.id);
            }
        }

        output.wait()?;

        Ok(stdout)
    }

    fn process_params(cmd: &str, job: &Job) -> String {
        let cmd = cmd
            .replace("%{device}", &job.device)
            .replace("%{device_f}", &format!("/dev/{}", &job.device));

        if let Some(output) = &job.output {
            cmd.replace("%{output}", output)
        } else {
            cmd
        }
    }
}
