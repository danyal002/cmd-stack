use crate::{args::ImportExportArgs, Cli};
use log::error;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleExportError {
    #[error("Failed to export commands")]
    LogicExport(#[from] logic::import_export::ExportError),
}

impl Cli {
    /// UI handler for export command
    pub fn handle_export_command(
        &self,
        args: ImportExportArgs,
    ) -> Result<PathBuf, HandleExportError> {
        let file_path = Path::new(&args.file).to_path_buf();
        self.logic.create_export_json(&file_path)?;
        Ok(file_path)
    }
}
