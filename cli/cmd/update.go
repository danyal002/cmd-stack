/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"fmt"
	"github.com/manifoldco/promptui"
	"github.com/spf13/cobra"
	"log"
)

// updateCmd represents the update command
var updateCmd = &cobra.Command{
	Use:   "update",
	Short: "Search for and update a command in your command stack",
	Long:  `missing_docs`,
	Run:   runUpdate,
}

func init() {
	rootCmd.AddCommand(updateCmd)
	updateCmd.Flags().StringP("command", "c", "", "Search by command")
	updateCmd.Flags().StringP("alias", "a", "", "Search by alias")
	updateCmd.Flags().StringP("tag", "t", "", "Search by tag")
	updateCmd.Flags().StringP("print", "p", "all", "Select how commands are presented to you (all, command, alias)")
	updateCmd.Flags().IntP("limit", "l", 5, "Limit the number of commands to display before scrolling (defaults to 10)")
}

func runUpdate(cmd *cobra.Command, args []string) {
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

	// Present the user with a form to update the command
	fmt.Println("Edit the fields you would like to update. Press Enter to keep the current value.")
	prompt := promptui.Prompt{
		Label:     "Command",
		Default:   command.Command,
		AllowEdit: true,
	}
	newCommand, err := prompt.Run()
	if err != nil {
		log.Fatal("Update Cmd: Failed to prompt for command", err)
		return
	}

	prompt = promptui.Prompt{
		Label:     "Alias",
		Default:   command.Alias,
		AllowEdit: true,
	}
	newAlias, err := prompt.Run()
	if err != nil {
		log.Fatal("Update Cmd: Failed to prompt for alias", err)
		return
	}

	prompt = promptui.Prompt{
		Label:     "Tags",
		Default:   command.Tags,
		AllowEdit: true,
	}
	newTags, err := prompt.Run()
	if err != nil {
		log.Fatal("Update Cmd: Failed to prompt for tags", err)
		return
	}

	prompt = promptui.Prompt{
		Label:     "Note",
		Default:   command.Note,
		AllowEdit: true,
	}
	newNote, err := prompt.Run()
	if err != nil {
		log.Fatal("Update Cmd: Failed to prompt for note", err)
		return
	}

	// Update the command in the database
	err = dataAccessLayer.UpdateCommandById(command.Id, newAlias, newCommand, newTags, newNote)
	if err != nil {
		log.Fatal("Update Cmd: Failed to update command", err)
		return
	}

	fmt.Println("Command updated successfully!")
}
