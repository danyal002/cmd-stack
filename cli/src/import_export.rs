use crate::{args::ImportExportArgs, outputs::ErrorOutput};
use log::error;
use logic::import_export::ImportExportError;
use std::path::Path;

/// UI handler for export command
pub fn handle_export_command(args: ImportExportArgs) {
    let file_path = Path::new(&args.file);

    match logic::import_export::create_export_json(file_path) {
        Ok(_) => println!("\nCommands exported to {:?}", file_path),
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

    match logic::import_export::import_data(file_path) {
        Ok(num) => println!("\n{} commands imported from {:?}", num, file_path),
        Err(e) => {
            error!(target: "Import Cmd", "Failed to import command: {:?}", e);
            ErrorOutput::Import.print();
        }
    }
}
