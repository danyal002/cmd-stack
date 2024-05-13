/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"fmt"
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
		tag, _ := cmd.Flags().GetString("tag")
		command, _ := cmd.Flags().GetString("command")
		alias, _ := cmd.Flags().GetString("alias")

		// We process searches in the following order:
		// 1. Search by tags
		// 2. Search by command
		// 3. Search by alias

		data_access_layer, err := dal.NewDataAccessLayer()
		if err != nil {
			log.Fatal(err)
		}
		defer data_access_layer.CloseDataAccessLayer()

		commands := []dal.Command{}
		var get_from_database = true
		if tag != "" {
			log.Println("Getting db from tags", tag)
			get_from_database = false
			commands, err = data_access_layer.SearchCommandsByTag(tag)
			if err != nil {
				log.Fatal(err)
			}
		}

		if command != "" && get_from_database {
			log.Println("Getting db from command", command)
			get_from_database = false
			commands, err = data_access_layer.SearchCommandsByCommand(command)
			if err != nil {
				log.Fatal(err)
			}
		} else if command != "" {
			log.Println("Getting from command", command)
			commands = dal.FilterCommandsByCommand(commands, command)
		}

		if alias != "" && get_from_database {
			log.Println("Getting db from alias", alias)
			get_from_database = false
			commands, err = data_access_layer.SearchCommandsByAlias(alias)
			if err != nil {
				log.Fatal(err)
			}
		} else if alias != "" {
			log.Println("Getting from alias", alias)
			commands = dal.FilterCommandsByAlias(commands, alias)
		}

		if len(commands) > 0 {
			for _, com := range commands {
				fmt.Println(com.String())
			}
		} else {
			fmt.Println("No commands found")
		}
	},
}

func init() {
	rootCmd.AddCommand(searchCmd)
	searchCmd.Flags().StringP("tag", "t", "", "Tag for the command")
	searchCmd.Flags().StringP("command", "c", "", "Command to search for")
	searchCmd.Flags().StringP("alias", "a", "", "Name for the command")
}
