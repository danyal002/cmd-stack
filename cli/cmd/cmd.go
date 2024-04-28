package cmd

import (
	"encoding/json"
)

type Command struct {
	Id      *uint64
	Alias   string
	Command string
	Tag     string
	Note    string
}

func (c Command) Serialize() ([]byte, error) {
	return json.Marshal(c)
}

func Deserialize(data []byte) (Command, error) {
	var cmd Command
	err := json.Unmarshal(data, &cmd)
	return cmd, err
}
