/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"fmt"
	"log"

	"github.com/spf13/cobra"
)

// searchCmd represents the search command
var searchCmd = &cobra.Command{
	Use:   "search",
	Short: "Search for a command in your command stack",
	Long:  `missing_docs`,
	Run: func(cmd *cobra.Command, args []string) {
		command, _ := cmd.Flags().GetString("command")

		data_access_layer, err := dal.NewDataAccessLayer()
		if err != nil {
			log.Fatal(err)
			return
		}
		defer data_access_layer.CloseDataAccessLayer()

		commands, err := data_access_layer.SearchByCommand(command)
		if err != nil {
			log.Fatal(err)
			return
		}

		if len(commands) > 0 {
			dal.PrintCommands(commands)
		} else {
			fmt.Println("No Commands Found...")
		}
	},
}

func init() {
	rootCmd.AddCommand(searchCmd)
	searchCmd.Flags().StringP("command", "c", "", "Search by command")
}
