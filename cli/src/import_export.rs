use crate::{
    args::ImportExportArgs,
    outputs::{ErrorOutput, Output},
};
use log::error;
use logic::{import_export::ImportExportError, Logic};
use std::path::Path;

/// UI handler for export command
pub fn handle_export_command(args: ImportExportArgs) {
    let file_path = Path::new(&args.file);

    let logic = Logic::try_default();
    if logic.is_err() {
        error!(target: "Export Cmd", "Failed to initialize logic: {:?}", logic.err());
        ErrorOutput::Export.print();
        return;
    }

    match logic.as_ref().unwrap().create_export_json(file_path) {
        Ok(_) => Output::ExportCommandsSuccess(file_path).print(),
        Err(e) => {
            error!(target: "Export Cmd", "Failed to export command: {:?}", e);
            match e {
                ImportExportError::NotJson => ErrorOutput::NotJson.print(),
                _ => ErrorOutput::Export.print(),
            }
        }
    }
}

/// UI handler for import command
pub fn handle_import_command(args: ImportExportArgs) {
    let file_path = Path::new(&args.file);

    let logic = Logic::try_default();
    if logic.is_err() {
        error!(target: "Import Cmd", "Failed to initialize logic: {:?}", logic.err());
        ErrorOutput::Import.print();
        return;
    }

    match logic.as_ref().unwrap().import_data(file_path) {
        Ok(num) => Output::ImportCommandsSuccess(num, file_path).print(),
        Err(e) => {
            error!(target: "Import Cmd", "Failed to import command: {:?}", e);
            ErrorOutput::Import.print();
        }
    }
}
