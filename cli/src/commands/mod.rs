mod check;
mod init;
mod projects;
mod services;

pub use check::*;
pub use init::*;
pub use projects::*;
pub use services::*;

use crate::error::AppError;

pub enum OutputFormat {
  Pretty,
  Json,
  Flat,
}

impl OutputFormat {
  pub fn from_string(format: &str) -> Result<OutputFormat, AppError> {
    match format {
      "pretty" => Ok(OutputFormat::Pretty),
      "json" => Ok(OutputFormat::Json),
      "flat" => Ok(OutputFormat::Flat),
      format => Err(AppError::UserError(format!("Invalid output format: {}", format))),
    }
  }
}
