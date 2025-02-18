use crate::{args::ImportExportArgs, Cli};
use log::error;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleImportError {
    #[error("Failed to import commands: {0}")]
    LogicImport(#[from] logic::import_export::ImportError),
}

impl Cli {
    /// UI handler for import command
    pub fn handle_import_command(
        &self,
        args: ImportExportArgs,
    ) -> Result<(u64, PathBuf), HandleImportError> {
        let file_path = Path::new(&args.file).to_path_buf();
        let num = self.logic.import_data(&file_path)?;
        Ok((num, file_path))
    }
}
