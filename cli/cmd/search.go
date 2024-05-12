/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"github.com/spf13/cobra"
	"log"
)

// searchCmd represents the search command
var searchCmd = &cobra.Command{
	Use:   "search",
	Short: "Search for a command in your command stack",
	Long:  `missing_docs`,
	Args:  cobra.ExactArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		//tags, _ := cmd.Flags().GetString("tags")
		command, _ := cmd.Flags().GetString("command")
		//alias, _ := cmd.Flags().GetString("alias")
		//note, _ := cmd.Flags().GetString("note")

		data_access_layer, err := dal.NewDataAccessLayer()
		if err != nil {
			log.Fatal(err)
		}
		defer data_access_layer.CloseDataAccessLayer()

		commands, err := data_access_layer.SearchCommandByCommand(command)
		if err != nil {
			log.Fatal(err)
		}

		// Print success log
		log.Printf("Search successful. Got %d command(s)", len(commands))

	},
}

func init() {
	rootCmd.AddCommand(searchCmd)
	searchCmd.Flags().StringP("tag", "t", "", "Tag for the command")
	searchCmd.Flags().StringP("command", "c", "", "Command to search for")
	searchCmd.Flags().StringP("alias", "a", "", "Name for the command")
	searchCmd.Flags().StringP("note", "n", "", "Note for the command")
}
