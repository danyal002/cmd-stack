use crate::args::ImportExportArgs;
use log::error;
use logic::Logic;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandleExportError {
    #[error("Failed to initialize logic")]
    LogicInit(#[from] logic::LogicInitError),
    #[error("Failed to export commands")]
    LogicExport(#[from] logic::import_export::ExportError),
}

#[derive(Error, Debug)]
pub enum HandleImportError {
    #[error("Failed to initialize logic")]
    LogicInit(#[from] logic::LogicInitError),
    #[error("Failed to import commands: {0}")]
    LogicImport(#[from] logic::import_export::ImportError),
}

/// UI handler for export command
pub fn handle_export_command(args: ImportExportArgs) -> Result<PathBuf, HandleExportError> {
    let file_path = Path::new(&args.file).to_path_buf();

    let logic = Logic::try_default()?;

    logic.create_export_json(&file_path)?;

    Ok(file_path)
}

/// UI handler for import command
pub fn handle_import_command(args: ImportExportArgs) -> Result<(u64, PathBuf), HandleImportError> {
    let file_path = Path::new(&args.file).to_path_buf();

    let logic = Logic::try_default()?;

    let num = logic.import_data(&file_path)?;

    Ok((num, file_path))
}
