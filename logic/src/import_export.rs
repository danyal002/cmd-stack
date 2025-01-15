use data::{
    dal::{InsertCommandError, SelectAllCommandsError},
    models::InternalCommand,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{self},
    path::Path,
};
use thiserror::Error;

use crate::Logic;

#[derive(Debug, Serialize, Deserialize)]
struct ImportExportFormat {
    commands: Vec<InternalCommand>,
}

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("Failed to serialize commands")]
    Deserialize(#[from] serde_json::Error),
    #[error("Failed to write commands to file")]
    Write(String),
    #[error("Failed to fetch commands from the database")]
    Database(#[from] SelectAllCommandsError),
}

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("Failed to deserialize commands: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Failed to read commands from file")]
    Read(String),
    #[error("Failed to insert commands to the database")]
    Database(#[from] InsertCommandError),
    #[error("File not found at specified path")]
    InvalidFilePath,
    #[error("Specified file does not have the correct extension")]
    IncorrectFileExtension,
}

impl Logic {
    #[tokio::main]
    /// Handle the export request by writing all data in the database to the requested JSON file
    pub async fn create_export_json(&self, export_file_path: &Path) -> Result<(), ExportError> {
        let commands = self.dal.get_all_commands(false, false).await?;
        let export_data = ImportExportFormat {
            commands: commands
                .into_iter()
                .map(|command| command.internal_command)
                .collect(),
        };
        let json_string = serde_json::to_string(&export_data)?;
        fs::write(export_file_path, json_string).map_err(|e| ExportError::Write(e.to_string()))?;

        Ok(())
    }

    #[tokio::main]
    /// Handle the import request by importing all data in the given JSON file
    pub async fn import_data(&self, import_file_path: &Path) -> Result<u64, ImportError> {
        let json_string =
            fs::read_to_string(import_file_path).map_err(|e| ImportError::Read(e.to_string()))?;
        let import_data: ImportExportFormat = serde_json::from_str(&json_string)?;

        let num_commands = self
            .dal
            .insert_mulitple_commands(import_data.commands)
            .await?;

        Ok(num_commands)
    }
}
