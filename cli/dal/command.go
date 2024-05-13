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
	UserId   *uint64
	LastUsed uint64
}

func (c Command) String() string {
	return c.Alias + " | " + c.Command
}

func (c Command) Serialize() ([]byte, error) {
	return json.Marshal(c)
}

func DeserializeCommand(data []byte) (Command, error) {
	var cmd Command
	err := json.Unmarshal(data, &cmd)
	return cmd, err
}

func PrintCommands(commands []Command) {
	for _, command := range commands {
		println(command.String())
	}
}

func FilterCommandsByCommand(commands []Command, command string) []Command {
	var filteredCommands []Command
	for _, cmd := range commands {
		if strings.Contains(cmd.Command, command) {
			filteredCommands = append(filteredCommands, cmd)
		}
	}
	return filteredCommands
}

func FilterCommandsByAlias(commands []Command, alias string) []Command {
	var filteredCommands []Command
	for _, cmd := range commands {
		if strings.Contains(cmd.Alias, alias) {
			filteredCommands = append(filteredCommands, cmd)
		}
	}
	return filteredCommands
}
