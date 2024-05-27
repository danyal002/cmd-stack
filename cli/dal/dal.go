package dal

import (
	"database/sql"
	"errors"
	"log"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

type DataAccessLayer struct {
	db *sql.DB
}

const DATABASE_NAME string = "cmdstack.db"
const DATABASE_CREATE_STRING string = `
CREATE TABLE IF NOT EXISTS command (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alias TEXT,
    command TEXT,
    tags TEXT,
    note TEXT,
    last_used INTEGER
);

CREATE TABLE IF NOT EXISTS param (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    command_id INTEGER,
    name TEXT,
    symbol TEXT,
    default_value TEXT,
    note TEXT
);
`

var MissingCommandError = errors.New("The command with the given ID does not exist")

// Create a new data access layer and initialize the database if required
func NewDataAccessLayer() (*DataAccessLayer, error) {
	db, err := sql.Open("sqlite3", DATABASE_NAME)
	if err != nil {
		return nil, err
	}

	// Initialize the database if required
	if _, err := db.Exec(DATABASE_CREATE_STRING); err != nil {
		return nil, err
	}
	return &DataAccessLayer{db: db}, nil
}

// Close the database connection
func (dal *DataAccessLayer) CloseDataAccessLayer() {
	dal.db.Close()
}

/****** CREATE ******/

// Add a command to the database with the given information
func (dal *DataAccessLayer) AddCommand(alias string, command string, tags string, note string) error {
	last_used := time.Now().Unix()
	_, err := dal.db.Exec("INSERT INTO command (alias, command, tags, note, last_used) VALUES (?, ?, ?, ?, ?)", alias, command, tags, note, last_used)
	return err
}

/****** READ ******/

// Extract list of commands from supplied rows
func (dal *DataAccessLayer) getCommandsFromRows(rows *sql.Rows) ([]Command, error) {
	var commands []Command
	for rows.Next() {
		var command Command
		if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.LastUsed); err != nil {
			log.Fatal("getCommandsFromRows: Failed to scan row:", err)
			return nil, err
		}
		commands = append(commands, command)
	}
	return commands, nil
}

// Get a command by the given id
func (dal *DataAccessLayer) GetCommandById(id int) (*Command, error) {
	stmt, err := dal.db.Prepare("SELECT * FROM command WHERE id = ?")
	if err != nil {
		log.Fatal("GetCommandById: Failed to prepare statement:", err)
		return nil, err
	}

	rows, err := stmt.Query(id)
	if err != nil {
		log.Fatal("GetCommandById: Failed to execute query:", err)
		return nil, err
	}
	defer rows.Close()

	var command Command
	if !rows.Next() {
		log.Println("The command with ID", id, "does not exist")
		return nil, MissingCommandError
	} else if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.LastUsed); err != nil {
		log.Fatal("GetCommandById: Failed to scan row:", err)
		return nil, err
	}
	return &command, nil
}

// Search for a command by the given tag text
func (dal *DataAccessLayer) SearchByTag(tag string) ([]Command, error) {
	stmt, err := dal.db.Prepare("SELECT * FROM command WHERE tags LIKE ?")
	if err != nil {
		log.Fatal("SearchByTag: Failed to create prepared statement:", err)
		return nil, err
	}

	rows, err := stmt.Query("%" + tag + "%")
	if err != nil {
		log.Fatal("SearchByTag: Failed to perform query:", err)
		return nil, err
	}
	defer rows.Close()

	commands, err := dal.getCommandsFromRows(rows)
	if err != nil {
		log.Fatal("SearchByTag: Failed to extract commands from rows", err)
		return nil, err
	}
	return commands, nil
}

// Search for a command by the given command text
func (dal *DataAccessLayer) SearchByCommand(command string) ([]Command, error) {
	stmt, err := dal.db.Prepare("SELECT * FROM command WHERE command LIKE ?")
	if err != nil {
		log.Fatal("SearchByCommand: Failed to create prepared statement:", err)
		return nil, err
	}

	rows, err := stmt.Query("%" + command + "%")
	if err != nil {
		log.Fatal("SearchByCommand: Failed to perform query:", err)
		return nil, err
	}
	defer rows.Close()

	commands, err := dal.getCommandsFromRows(rows)
	if err != nil {
		log.Fatal("SearchByCommand: Failed to extract commands from rows", err)
		return nil, err
	}
	return commands, nil
}

// Get all commands from the database, limiting and/or ordering results based on the supplied parameters
func (dal *DataAccessLayer) GetCommands(limit int, order_by_recent_usage bool) ([]Command, error) {
	var stmt *sql.Stmt
	var err error

	if order_by_recent_usage {
		stmt, err = dal.db.Prepare("SELECT * FROM command ORDER BY last_used DESC LIMIT ?")
	} else {
		stmt, err = dal.db.Prepare("SELECT * FROM command LIMIT ?")
	}
	if err != nil {
		log.Fatal("GetCommands: Failed to construct Prepared Statement: ", err)
		return nil, err
	}

	rows, err := stmt.Query(limit)
	if err != nil {
		log.Fatal("GetCommands: Failed to execute query: ", err)
		return nil, err
	}
	defer rows.Close()

	commands, err := dal.getCommandsFromRows(rows)
	if err != nil {
		log.Fatal("SearchByCommand: Failed to extract commands from rows", err)
		return nil, err
	}
	return commands, nil
}

// Search for a command by the given alias text
func (dal *DataAccessLayer) SearchByAlias(alias string) ([]Command, error) {
	stmt, err := dal.db.Prepare("SELECT * FROM command WHERE alias LIKE ?")
	if err != nil {
		log.Fatal("SearchByAlias: Failed to create prepared statement:", err)
		return nil, err
	}

	rows, err := stmt.Query("%" + alias + "%")
	if err != nil {
		log.Fatal("SearchByAlias: Failed to perform query:", err)
		return nil, err
	}
	defer rows.Close()

	commands, err := dal.getCommandsFromRows(rows)
	if err != nil {
		log.Fatal("SearchByAlias: Failed to extract commands from rows", err)
		return nil, err
	}
	return commands, nil
}

/****** UPDATE ******/

// Update the last used time of a command
func (dal *DataAccessLayer) UpdateCommandLastUsedById(id *uint64) error {
	if id == nil {
		log.Fatal("UpdateCommandLastUsedById: id cannot be nil")
		return errors.New("UpdateCommandLastUsedById: id cannot be nil")
	}

	stmt, err := dal.db.Prepare("UPDATE command SET last_used = ? WHERE id = ?")
	if err != nil {
		log.Fatal("UpdateCommandLastUsedById: Failed to prepare update statement:", err)
		return err
	}

	_, err = stmt.Exec(time.Now().Unix(), id)
	if err != nil {
		log.Fatal("UpdateCommandLastUsedById: failed to execute update statement:", err)
		return err
	}
	return nil
}

/****** DELETE ******/

func (dal *DataAccessLayer) DeleteCommandById(id int) error {
	// Determine if the id exists
	_, err := dal.GetCommandById(id)
	if err != nil {
		log.Fatal("DeleteCommandById: failed to get command by id:", err)
		return err
	}

	// Delete the id
	stmt, err := dal.db.Prepare("DELETE FROM command WHERE id = ?")
	if err != nil {
		log.Fatal("DeleteCommandById: Failed to prepare delete statement:", err)
		return err
	}

	_, err = stmt.Exec(id)
	if err != nil {
		log.Fatal("DeleteCommandById: failed to execute delete statement:", err)
		return err
	}
	return nil
}
