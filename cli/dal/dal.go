package dal

import (
	"database/sql"
	_ "github.com/mattn/go-sqlite3"
	"time"
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
    user_id INTEGER,
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
func (dal *DataAccessLayer) AddCommand(alias string, command string, tags string, note string, user_id uint64) error {
	last_used := time.Now().Unix()
	_, err := dal.db.Exec("INSERT INTO command (alias, command, tags, note, user_id, last_used) VALUES (?, ?, ?, ?, ?, ?)", alias, command, tags, note, user_id, last_used)
	return err
}

// Get all commands in the database
func (dal *DataAccessLayer) getAllCommands() ([]Command, error) {
	rows, err := dal.db.Query("SELECT * FROM command")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var commands []Command
	for rows.Next() {
		var command Command
		if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.UserId, &command.LastUsed); err != nil {
			return nil, err
		}
		commands = append(commands, command)
	}
	return commands, nil
}

// Find all commands that have a name that matches the given alias
func (dal *DataAccessLayer) SearchCommandsByAlias(alias string) ([]Command, error) {
	commands, err := dal.getAllCommands()
	if err != nil {
		return nil, err
	}
	return FilterCommandsByAlias(commands, alias), nil
}

// Find all commands that match the given command
func (dal *DataAccessLayer) SearchCommandsByCommand(command string) ([]Command, error) {
	commands, err := dal.getAllCommands()
	if err != nil {
		return nil, err
	}
	return FilterCommandsByCommand(commands, command), nil
}

// Find all commands that have a tag that matches the given tag
func (dal *DataAccessLayer) SearchCommandsByTag(tag string) ([]Command, error) {
	rows, err := dal.db.Query("SELECT * FROM command WHERE tags LIKE ?", "%"+tag+"%")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var commands []Command
	for rows.Next() {
		var command Command
		if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.UserId, &command.LastUsed); err != nil {
			return nil, err
		}
		commands = append(commands, command)
	}
	return commands, nil
}
