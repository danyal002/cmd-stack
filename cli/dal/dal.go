package dal

import (
	"database/sql"
	"errors"
	"log"
	"time"

	sq "github.com/Masterminds/squirrel"
	_ "github.com/mattn/go-sqlite3"
)

type DataAccessLayer struct {
	db *sql.DB
}

type SearchFilters struct {
	Command string
	Alias   string
	Tag     string
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
var InvalidSearchFiltersError = errors.New("Provided search filters are invalid")

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
func (dal *DataAccessLayer) GetCommandById(id *uint64) (*Command, error) {
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

// Search for a command by the given search filters
func (dal *DataAccessLayer) SearchForCommand(searchFilters SearchFilters) ([]Command, error) {
	if searchFilters.Command == "" && searchFilters.Alias == "" && searchFilters.Tag == "" {
		return nil, InvalidSearchFiltersError
	}

	commands := sq.Select("*").From("command")
	if searchFilters.Command != "" {
		commands = commands.Where("command LIKE ?", "%"+searchFilters.Command+"%")
	}
	if searchFilters.Alias != "" {
		commands = commands.Where("alias LIKE ?", "%"+searchFilters.Alias+"%")
	}
	if searchFilters.Tag != "" {
		commands = commands.Where("tags LIKE ?", "%"+searchFilters.Tag+"%")
	}

	sql, args, err := commands.ToSql()
	if err != nil {
		log.Fatal("searchForCommand: Failed to construct SQL query:", err)
		return nil, err
	}
	stmt, err := dal.db.Prepare(sql)
	if err != nil {
		log.Fatal("searchForCommand: Failed to prepare statement:", err)
		return nil, err
	}

	rows, err := stmt.Query(args...)
	if err != nil {
		log.Fatal("searchForCommand: Failed to execute query:", err)
		return nil, err
	}

	commandsList, err := dal.getCommandsFromRows(rows)
	if err != nil {
		log.Fatal("searchForCommand: Failed to extract commands from rows", err)
		return nil, err
	}
	return commandsList, nil
}

// Get all commands from the database, limiting and/or ordering results based on the supplied parameters
func (dal *DataAccessLayer) GetCommands(limit int, orderByRecentUsage bool) ([]Command, error) {
	var stmt *sql.Stmt
	var err error

	if orderByRecentUsage {
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

/****** UPDATE ******/

// Update command by ID
func (dal *DataAccessLayer) UpdateCommandById(id *uint64, alias string, command string, tags string, note string) error {
	stmt, err := dal.db.Prepare("UPDATE command SET alias = ?, command = ?, tags = ?, note = ?, last_used = ? WHERE id = ?")
	if err != nil {
		log.Fatal("UpdateCommandById: Failed to prepare update statement:", err)
		return err
	}

	_, err = stmt.Exec(alias, command, tags, note, time.Now().Unix(), id)
	if err != nil {
		log.Fatal("UpdateCommandById: failed to execute update statement:", err)
		return err
	}

	return nil
}

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

func (dal *DataAccessLayer) DeleteCommandById(id *uint64) error {
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
