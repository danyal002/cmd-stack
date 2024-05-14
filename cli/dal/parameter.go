package dal

import "encoding/json"

type Parameter struct {
	Id           *uint64 // Unique identifier for the parameter in the parameter table
	CommandId    *uint64 // Id of the command that this parameter belongs to
	Name         string
	Symbol       string
	DefaultValue string
	Note         string
}

func (p Parameter) Serialize() ([]byte, error) {
	return json.Marshal(p)
}

func DeserializeParameter(data []byte) (Parameter, error) {
	var param Parameter
	err := json.Unmarshal(data, &param)
	return param, err
}
