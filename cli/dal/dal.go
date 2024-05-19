package dal

import (
	"database/sql"
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

// Add a command to the database with the given information
func (dal *DataAccessLayer) AddCommand(alias string, command string, tags string, note string) error {
	last_used := time.Now().Unix()
	_, err := dal.db.Exec("INSERT INTO command (alias, command, tags, note, last_used) VALUES (?, ?, ?, ?, ?)", alias, command, tags, note, last_used)
	return err
}

// Search for a command by the given text
func (dal *DataAccessLayer) SearchByCommand(command string) ([]Command, error) {
	stmt, err := dal.db.Prepare("SELECT * FROM command WHERE command LIKE ?")
	if err != nil {
		log.Fatal("SearchByCommand: Failed to construct Prepared Statement: ", err)
		return nil, err
	}

	rows, err := stmt.Query("%" + command + "%")
	if err != nil {
		log.Fatal("SearchByCommand: Failed to execute query: ", err)
		return nil, err
	}
	defer rows.Close()

	var commands []Command
	for rows.Next() {
		var command Command
		if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.LastUsed); err != nil {
			log.Fatal("SearchByCommand: Failed to scan row: ", err)
			return nil, err
		}
		commands = append(commands, command)
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

	var commands []Command
	for rows.Next() {
		var command Command
		if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.LastUsed); err != nil {
			log.Fatal("GetCommands: Failed to extract Command: ", err)
			return nil, err
		}
		commands = append(commands, command)
	}
	return commands, nil
}
