use data::{dal::{Dal, SqlQueryError}, models::{Command, InternalParameter}};
use thiserror::Error;
use std::{fs, path::Path, collections::HashMap};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::Logic;

#[derive(Debug, Serialize, Deserialize)]
struct ExportFormat {
    commands: Vec<Command>,
    parameters: Vec<InternalParameter>
}

#[derive(Error, Debug)]
pub enum ImportExportError {
    #[error("database connection error")]
    DbConnection(#[from] data::dal::sqlite::SQliteDatabaseConnectionError),

    #[error("database query error")]
    DbQuery(#[from] SqlQueryError),

    #[error("could not serialize data")]
    SerdeError(#[from] serde_json::Error),

    #[error("could not read or write to file")]
    RwFile(#[from] std::io::Error),

    #[error("provided file is not a JSON file")]
    NotJson,

    #[error("provided file does not exist")]
    DoesNotExist,

    #[error("provided file path is invalid")]
    InvalidFilePath,

    #[error("import data is invalid")]
    InvalidData
}

/// Check if the file is a json file
fn is_file_json(file_path: &Path) -> Result<(), ImportExportError>  {
    // Ensure that the file is a JSON file
    if let Some(extension) = file_path.extension() {
        if extension != "json" {
            return Err(ImportExportError::NotJson)
        }
    } else {
        return Err(ImportExportError::InvalidFilePath);
    }
    Ok(())
}

impl Logic {
    

    #[tokio::main]
    /// Returns a JSON string containing all commands and parameters
    pub async fn create_export_json(&self, export_file_path: &Path) -> Result<(), ImportExportError> {
        is_file_json(export_file_path)?; 

        // Get all commands and parameters
        let export_data = ExportFormat {
            commands: self.dal.get_all_commands(false, false).await?,
            parameters: self.dal.get_all_internal_parameters().await?
        };

        let json_string = serde_json::to_string(&export_data)?;
        fs::write(export_file_path, json_string)?;

        Ok(())
    }


    #[tokio::main]
    pub async fn import_data(&self, import_file_path: &Path) -> Result<(), ImportExportError> {
        // Check if the file exists
        if !import_file_path.is_file() {
            return Err(ImportExportError::InvalidFilePath);
        }
        
        is_file_json(import_file_path)?; 

        // Deserialize the file 
        let data = fs::read_to_string(import_file_path)?;
        let import_data: ExportFormat = serde_json::from_str(&data)?;

        // Insert all records into the database
        //
        // We keep a map mapping command IDs in the json to their respective
        // ids in the new database. This is required when inserting the parameters
        // to ensure the foreign key references are consistent
        let mut import_cmd_id_to_db_id: HashMap<i64, i64> = HashMap::new();
        for command in import_data.commands {
            let db_id = self.dal.add_command(command.internal_command).await?;
            import_cmd_id_to_db_id.insert(command.id, db_id);
        }

        if import_data.parameters.len() > 0 {
            let mut insert_params: Vec<InternalParameter> = vec![];
            for param in import_data.parameters {
                let cmd_id = param.command_id;
                insert_params.push(InternalParameter {
                    command_id: match import_cmd_id_to_db_id.get(&cmd_id) {
                        Some(id) => *id,
                        None => return Err(ImportExportError::InvalidData)
                    },
                    symbol: param.symbol,
                    regex: param.regex,
                    note: param.note,
                })
            }
            self.dal.add_params(insert_params).await?;
        }

        Ok(())
    }
}