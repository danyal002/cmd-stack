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
	Run:   runDelete,
}

func init() {
	rootCmd.AddCommand(deleteCmd)
	deleteCmd.Flags().StringP("command", "c", "", "Search by command")
	deleteCmd.Flags().StringP("alias", "a", "", "Search by alias")
	deleteCmd.Flags().StringP("tag", "t", "", "Search by tag")
	deleteCmd.Flags().StringP("print", "p", "all", "Select how commands are presented to you (all, command, alias)")
	deleteCmd.Flags().IntP("limit", "l", 5, "Limit the number of commands to display before scrolling (defaults to 10)")
}

func runDelete(cmd *cobra.Command, args []string) {
	dataAccessLayer, err := dal.NewDataAccessLayer()
	if err != nil {
		log.Fatal("Update Cmd: Failed to create dal", err)
		return
	}
	defer dataAccessLayer.CloseDataAccessLayer()

	searchFilters, printOption, limit, err := ExtractAndValidateSearchArgs(cmd)
	if err != nil {
		log.Fatal("Update Cmd: Invalid args", err)
		return
	}

	command, err := GetSelectedItemFromUser(dataAccessLayer, searchFilters, printOption, limit)
	if err != nil {
		log.Fatal("Update Cmd: Failed to get selected item from user", err)
		return
	}

	err = dataAccessLayer.DeleteCommandById(command.Id)
	if errors.Is(err, dal.MissingCommandError) {
		fmt.Println("The command does not exist")
		return
	} else if err != nil {
		log.Fatal("Delete Cmd: An error occurred while deleting the command with ID", *command.Id, ":", err)
		return
	} else {
		fmt.Println("Command deleted successfully")
	}
}
