package dal

import (
	"encoding/json"
	"strconv"
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
	return c.Alias + " | " + c.Command + " | " + c.Tags + " | " + c.Note + " | " + strconv.FormatUint(c.LastUsed, 10)
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
