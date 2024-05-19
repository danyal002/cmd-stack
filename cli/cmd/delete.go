/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"errors"
	"fmt"
	"github.com/spf13/cobra"
	"log"
)

// deleteCmd represents the delete command
var deleteCmd = &cobra.Command{
	Use:   "delete",
	Short: "Search for and delete a command in your command stack",
	Long:  `missing_docs`,
	Run: func(cmd *cobra.Command, args []string) {
		id, _ := cmd.Flags().GetInt("id")

		if id == -1 {
			log.Fatal("Delete Cmd: You must provide an ID to delete a command")
			return
		}

		data_access_layer, err := dal.NewDataAccessLayer()
		if err != nil {
			log.Fatal("Delete Cmd: Failed to create data access layer:", err)
			return
		}
		defer data_access_layer.CloseDataAccessLayer()

		err = data_access_layer.DeleteCommandById(id)
		if errors.Is(err, dal.MissingCommandError) {
			fmt.Println("The command with ID", id, "does not exist")
			return
		} else if err != nil {
			log.Fatal("Delete Cmd: An error occurred while deleting the command with ID", id, ":", err)
			return
		} else {
			fmt.Println("Command with ID", id, "deleted successfully")
		}
	},
}

func init() {
	rootCmd.AddCommand(deleteCmd)
	deleteCmd.Flags().IntP("id", "i", -1, "ID of the command to delete")
}
