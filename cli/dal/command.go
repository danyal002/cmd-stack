package dal

import (
	"encoding/json"
)

type Command struct {
	Id       *uint64
	Alias    string
	Command  string
	Tags     string
	Note     string
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
