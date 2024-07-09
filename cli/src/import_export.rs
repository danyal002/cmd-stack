use crate::args::ImportExportArgs;

pub fn handle_export_command(args: ImportExportArgs) {
    let file = args.file;

    // Ensure that the file is a JSON file
    if !file.ends_with(".json") {
        println!("Export Cmd: File must be a JSON file");
        return;
    }

    match logic::import_export::create_export_json(file.clone()) {
        Ok(_) => println!("\nCommands exported to {}", file),
        Err(e) => println!("Failed to export command: {:?}", e)
    }
}


