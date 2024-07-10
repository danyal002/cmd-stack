use logic::Logic;

use crate::args::ImportExportArgs;
use std::path::Path;

pub fn handle_export_command(logic_layer: Logic, args: ImportExportArgs) {
    let file_path = Path::new(&args.file);

    match logic_layer.create_export_json(file_path) {
        Ok(_) => println!("\nCommands exported to {:?}", file_path),
        Err(e) => println!("Export Cmd: Failed to export command: {:?}", e)
    }
}

pub fn handle_import_command(logic_layer: Logic, args: ImportExportArgs) {
    let file_path = Path::new(&args.file);

    match logic_layer.import_data(file_path) {
        Ok(_) => println!("\nCommands imported from {:?}", file_path),
        Err(e) => println!("Import Cmd: Failed to import command: {:?}", e)
    }
}


