package dal

import (
	"database/sql"
	"errors"
	_ "github.com/mattn/go-sqlite3"
	"log"
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

// Add a command to the database with the given information
func (dal *DataAccessLayer) AddCommand(alias string, command string, tags string, note string) error {
	last_used := time.Now().Unix()
	_, err := dal.db.Exec("INSERT INTO command (alias, command, tags, note, last_used) VALUES (?, ?, ?, ?, ?)", alias, command, tags, note, last_used)
	return err
}

// Get a command by the given id
func (dal *DataAccessLayer) GetCommandById(id int) (*Command, error) {
	stmt, err := dal.db.Prepare("SELECT * FROM command WHERE id = ?")
	if err != nil {
		log.Fatal(err)
		return nil, err
	}

	rows, err := stmt.Query(id)
	if err != nil {
		log.Fatal(err)
		return nil, err
	}
	defer rows.Close()

	var command Command
	if !rows.Next() {
		log.Println("The command with ID", id, "does not exist")
		return nil, MissingCommandError
	} else if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.LastUsed); err != nil {
		return nil, err
	}
	return &command, nil
}

// Search for a command by the given text
func (dal *DataAccessLayer) SearchByCommand(command string) ([]Command, error) {
	stmt, err := dal.db.Prepare("SELECT * FROM command WHERE command LIKE ?")
	if err != nil {
		log.Fatal(err)
		return nil, err
	}

	rows, err := stmt.Query("%" + command + "%")
	if err != nil {
		log.Fatal(err)
		return nil, err
	}
	defer rows.Close()

	var commands []Command
	for rows.Next() {
		var command Command
		if err := rows.Scan(&command.Id, &command.Alias, &command.Command, &command.Tags, &command.Note, &command.LastUsed); err != nil {
			return nil, err
		}
		commands = append(commands, command)
	}
	return commands, nil
}

func (dal *DataAccessLayer) DeleteCommandById(id int) error {
	// Determine if the id exists
	_, err := dal.GetCommandById(id)
	if err != nil {
		return err
	}

	// Delete the id
	stmt, err := dal.db.Prepare("DELETE FROM command WHERE id = ?")
	if err != nil {
		log.Fatal(err)
		return err
	}

	_, err = stmt.Exec(id)
	if err != nil {
		log.Fatal(err)
		return err
	}
	return nil
}
