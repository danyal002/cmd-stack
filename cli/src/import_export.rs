use crate::{args::ImportExportArgs, outputs::ErrorOutput};
use log::error;
use std::path::Path;

pub fn handle_export_command(args: ImportExportArgs) {
    let file_path = Path::new(&args.file);

    match logic::import_export::create_export_json(file_path) {
        Ok(_) => println!("\nCommands exported to {:?}", file_path),
        Err(e) => {
            error!(target: "Export Cmd", "Failed to export command: {:?}", e);
            ErrorOutput::Export.print();
        }
    }
}

pub fn handle_import_command(args: ImportExportArgs) {
    let file_path = Path::new(&args.file);

    match logic::import_export::import_data(file_path) {
        Ok(_) => println!("\nCommands imported from {:?}", file_path),
        Err(e) => {
            error!(target: "Import Cmd", "Failed to import command: {:?}", e);
            ErrorOutput::Import.print();
        }
    }
}
