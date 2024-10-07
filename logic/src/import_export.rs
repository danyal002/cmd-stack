use data::{
    dal::{Dal, SqlQueryError, SqlTxError},
    models::{Command, InternalParameter},
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, fs, path::Path};
use thiserror::Error;

use crate::{DatabaseConnectionError, Logic};

#[derive(Debug, Serialize, Deserialize)]
struct ExportFormat {
    commands: Vec<Command>,
    parameters: Vec<InternalParameter>,
}

#[derive(Error, Debug)]
pub enum ImportExportError {
    #[error("database connection error")]
    DbConnection(#[from] DatabaseConnectionError),

    #[error("error executing database query")]
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
    InvalidData,

    #[error("transaction error")]
    Transaction(#[from] SqlTxError),
}

/// Check if the file is a json file
fn is_file_json(file_path: &Path) -> Result<(), ImportExportError> {
    if let Some(extension) = file_path.extension() {
        if extension != "json" {
            return Err(ImportExportError::NotJson);
        }
    } else {
        return Err(ImportExportError::InvalidFilePath);
    }
    Ok(())
}

impl Logic {
    #[tokio::main]
    /// Handle the export request by writing all data in the database to the requested JSON file
    pub async fn create_export_json(
        &self,
        export_file_path: &Path,
    ) -> Result<(), ImportExportError> {
        is_file_json(export_file_path)?;

        // Get all commands and parameters
        let export_data = ExportFormat {
            commands: self
                .db_connection
                .get_all_commands(false, false, None)
                .await?,
            parameters: self.db_connection.get_all_internal_parameters(None).await?,
        };

        let json_string = serde_json::to_string(&export_data)?;
        fs::write(export_file_path, json_string)?;

        Ok(())
    }

    #[tokio::main]
    /// Handle the import request by importing all data in the given JSON file
    pub async fn import_data(&self, import_file_path: &Path) -> Result<u32, ImportExportError> {
        // Check if the file exists
        if !import_file_path.is_file() {
            return Err(ImportExportError::InvalidFilePath);
        }

        is_file_json(import_file_path)?;

        // Deserialize the file
        let data = fs::read_to_string(import_file_path)?;
        let import_data: ExportFormat = serde_json::from_str(&data)?;

        let mut tx = self.db_connection.begin().await?;

        // Insert all records into the database
        //
        // We keep a map linking command IDs in the json to their respective
        // ids in the database. This is required when inserting the parameters
        // to ensure the foreign key references are correct
        let num_commands = import_data.commands.len() as u32;
        let mut import_cmd_id_to_db_id: HashMap<i64, i64> = HashMap::new();
        for command in import_data.commands {
            let db_id = match self
                .db_connection
                .add_command(command.internal_command, Some(&mut tx))
                .await
            {
                Ok(id) => id,
                Err(e) => {
                    self.db_connection.rollback(tx).await?;
                    return Err(ImportExportError::DbQuery(e));
                }
            };
            import_cmd_id_to_db_id.insert(command.id, db_id);
        }

        if !import_data.parameters.is_empty() {
            let mut insert_params: Vec<InternalParameter> = vec![];
            for param in import_data.parameters {
                let cmd_id = param.command_id;
                insert_params.push(InternalParameter {
                    command_id: match import_cmd_id_to_db_id.get(&cmd_id) {
                        Some(id) => *id,
                        None => {
                            self.db_connection.rollback(tx).await?;
                            return Err(ImportExportError::InvalidData);
                        }
                    },
                    symbol: param.symbol,
                    regex: param.regex,
                    note: param.note,
                })
            }
            if let Err(e) = self
                .db_connection
                .add_params(insert_params, Some(&mut tx))
                .await
            {
                self.db_connection.rollback(tx).await?;
                return Err(ImportExportError::DbQuery(e));
            }
        }

        self.db_connection.commit(tx).await?;
        Ok(num_commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn is_file_json_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        let result = is_file_json(&file_path);
        assert!(result.is_ok());
    }

    #[test]
    fn is_file_json_wrong_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        let result = is_file_json(&file_path);
        assert!(matches!(result, Err(ImportExportError::NotJson)));
    }

    #[test]
    fn is_file_json_no_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test");

        let result = is_file_json(&file_path);
        assert!(matches!(result, Err(ImportExportError::InvalidFilePath)));
    }
}
