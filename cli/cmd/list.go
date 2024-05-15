/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"github.com/spf13/cobra"
	"log"
)

// listCmd represents the list command
var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List your most recently used commands in command stack",
	Long:  `missing_docs`,
	Run: func(cmd *cobra.Command, args []string) {
		use_recent, _ := cmd.Flags().GetBool("recent")
		limit, _ := cmd.Flags().GetInt("limit")

		data_access_layer, err := dal.NewDataAccessLayer()
		if err != nil {
			log.Fatal("List Command: Failed to get Data Access Layer: ", err)
			return
		}
		defer data_access_layer.CloseDataAccessLayer()

		commands, err := data_access_layer.GetAllCommands(limit, use_recent)
		if err != nil {
			log.Fatal("List Command: Failed to retrieve commands from the database: ", err)
			return
		}

		dal.PrintCommands(commands)
	},
}

func init() {
	rootCmd.AddCommand(listCmd)
	listCmd.Flags().BoolP("recent", "r", false, "List most recently used commands")
	listCmd.Flags().IntP("limit", "l", 25, "Limit the number of commands listed")
}
