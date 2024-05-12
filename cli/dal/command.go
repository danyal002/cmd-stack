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
	UserId   *uint64
	LastUsed uint64
}

func (c Command) Serialize() ([]byte, error) {
	return json.Marshal(c)
}

func DeserializeCommand(data []byte) (Command, error) {
	var cmd Command
	err := json.Unmarshal(data, &cmd)
	return cmd, err
}
