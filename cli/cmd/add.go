/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"log"
	"strconv"

	"github.com/spf13/cobra"
)

// addCmd represents the add command
var addCmd = &cobra.Command{
	Use:   "add",
	Short: "missing_docs",
	Long:  `missing_docs`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		cmdText := args[0]
		tag, _ := cmd.Flags().GetString("tag")
		note, _ := cmd.Flags().GetString("note")
		alias, _ := cmd.Flags().GetString("alias")

		command := Command{
			Command: cmdText,
			Tag:     tag,
			Note:    note,
			Alias:   alias,
			Id:      nil,
		}

		command_id := dal.GenerateId()
		command.Id = &command_id

		command_str, err := command.Serialize()
		if err != nil {
			log.Fatal(err)
		}

		err = dal.Add(strconv.FormatUint(*command.Id, 10), string(command_str))
		if err != nil {
			log.Fatal(err)
		}
	},
}

func init() {
	rootCmd.AddCommand(addCmd)
	addCmd.Flags().StringP("tag", "t", "", "Tag for the command")
	addCmd.Flags().StringP("note", "n", "", "Note for the command")
	addCmd.Flags().StringP("alias", "a", "", "Alias for the command")
}
