use std::process::ExitCode;

pub type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub struct CliError {
    pub exit_code: ExitCode,
    pub message: String,
}
