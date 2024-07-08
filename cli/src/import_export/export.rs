use crate::args::ImportExportArgs;

pub fn handle_export_command(args: ImportExportArgs) {
    let file = args.file;

    // Ensure that the file is a JSON file
    if !file.ends_with(".json") {
        println!("Export Cmd: File must be a JSON file");
        return;
    }
    
    println!("Exporting to file: {}", file);
}