use crate::{args::ImportExportArgs, outputs::Output};
use log::error;
use logic::Logic;
use std::path::Path;
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
    #[error("Failed to export commands")]
    LogicImport(#[from] logic::import_export::ImportError),
}

/// UI handler for export command
pub fn handle_export_command(args: ImportExportArgs) -> Result<(), HandleExportError> {
    let file_path = Path::new(&args.file);

    let logic = Logic::try_default()?;

    logic.create_export_json(file_path)?;

    Output::ExportCommandsSuccess(file_path).print();
    Ok(())
}

/// UI handler for import command
pub fn handle_import_command(args: ImportExportArgs) -> Result<(), HandleImportError> {
    let file_path = Path::new(&args.file);

    let logic = Logic::try_default()?;

    let num = logic.import_data(file_path)?;

    Output::ImportCommandsSuccess(num, file_path).print();
    Ok(())
}
