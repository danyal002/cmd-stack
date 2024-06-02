package dal

import (
	"encoding/json"
	"github.com/lithammer/fuzzysearch/fuzzy"
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

func (c Command) String() string {
	const MaxCharsPrintedAlias = 25
	const MaxCharsPrintedCommand = 50
	const MaxCharsPrintedTags = 10
	const MaxCharsPrintedNote = 50

	var alias string
	var command string
	var tags string
	var note string

	if len(c.Alias) > MaxCharsPrintedAlias {
		alias = c.Alias[:MaxCharsPrintedAlias-3] + "..."
	} else {
		alias = c.Alias + strings.Repeat(" ", MaxCharsPrintedAlias-len(c.Alias))
	}

	if len(c.Command) > MaxCharsPrintedCommand {
		command = c.Command[:MaxCharsPrintedCommand-3] + "..."
	} else {
		command = c.Command + strings.Repeat(" ", MaxCharsPrintedCommand-len(c.Command))
	}

	if len(c.Tags) > MaxCharsPrintedTags {
		tags = c.Tags[:MaxCharsPrintedTags-3] + "..."
	} else {
		tags = c.Tags + strings.Repeat(" ", MaxCharsPrintedTags-len(c.Tags))
	}

	if len(c.Note) > MaxCharsPrintedNote {
		note = c.Note[:MaxCharsPrintedNote-3] + "..."
	} else {
		note = c.Note + strings.Repeat(" ", MaxCharsPrintedNote-len(c.Note))
	}

	return alias + " | " + command + " | " + tags + " | " + note
}

func (c Command) Serialize() ([]byte, error) {
	return json.Marshal(c)
}

// Returns the properties of a command that are printed based on the selected print option
func GetPrintedValues(printOption string) string {
	switch printOption {
	case "all":
		return "Alias | Command | Tags | Note"
	case "command":
		return "Command"
	case "alias":
		return "Alias"
	}
	return ""
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

// Filter a list of commands using the given search filters
func FilterCommandsBySearchFilters(commands []Command, searchFilters *SearchFilters) []Command {
	var filtered []Command
	for _, cmd := range commands {
		if (searchFilters.Command == "" || (strings.Contains(cmd.Command, searchFilters.Command) || fuzzy.MatchFold(searchFilters.Command, cmd.Command))) &&
			(searchFilters.Alias == "" || (strings.Contains(cmd.Alias, searchFilters.Alias) || fuzzy.MatchFold(searchFilters.Alias, cmd.Alias))) &&
			(searchFilters.Tag == "" || (strings.Contains(cmd.Tags, searchFilters.Tag) || fuzzy.MatchFold(searchFilters.Tag, cmd.Tags))) {
			filtered = append(filtered, cmd)
		}
	}
	return filtered
}
