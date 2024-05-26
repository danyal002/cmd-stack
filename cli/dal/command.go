package dal

import (
	"encoding/json"
	"strings"
)

type Command struct {
	Id       *uint64
	Alias    string
	Command  string
	Tags     string
	Note     string
	LastUsed uint64
}

var CmdPrintingOptions = []string{"all", "command", "alias"}

const MaxCharsPrinted = 50

func (c Command) String() string {
	var command string
	var note string
	if len(c.Command) > MaxCharsPrinted {
		command = c.Command[:MaxCharsPrinted] + "..."
	} else {
		command = c.Command
	}

	if len(c.Note) > MaxCharsPrinted {
		note = c.Note[:MaxCharsPrinted] + "..."
	} else {
		note = c.Note
	}

	return c.Alias + " | " + command + " | " + c.Tags + " | " + note
}

func (c Command) Serialize() ([]byte, error) {
	return json.Marshal(c)
}

func FormatCommands(commands []Command, printOption string) []string {
	var formattedCommands []string
	for _, command := range commands {
		switch printOption {
		case "all":
			formattedCommands = append(formattedCommands, command.String())
		case "command":
			formattedCommands = append(formattedCommands, command.Command)
		case "alias":
			formattedCommands = append(formattedCommands, command.Alias)
		}
	}
	return formattedCommands
}

// Filter a list of commands using the given command string
func FilterCommandsByCommand(commands []Command, command string) []Command {
	var filtered []Command
	for _, cmd := range commands {
		if strings.Contains(cmd.Command, command) {
			filtered = append(filtered, cmd)
		}
	}
	return filtered
}

// Filter a list of commands using the given alias string
func FilterCommandsByAlias(commands []Command, alias string) []Command {
	var filtered []Command
	for _, cmd := range commands {
		if strings.Contains(cmd.Alias, alias) {
			filtered = append(filtered, cmd)
		}
	}
	return filtered
}
