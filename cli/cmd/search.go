/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"fmt"
	"github.com/manifoldco/promptui"
	"github.com/spf13/cobra"
	"golang.design/x/clipboard"
	"log"
)

// searchCmd represents the search command
var searchCmd = &cobra.Command{
	Use:   "search",
	Short: "Search for a command in your command stack",
	Long:  `missing_docs`,
	Run:   runSearch,
}

func init() {
	rootCmd.AddCommand(searchCmd)
	searchCmd.Flags().StringP("command", "c", "", "Search by command")
	searchCmd.Flags().StringP("alias", "a", "", "Search by alias")
	searchCmd.Flags().StringP("tag", "t", "", "Search by tag")
	searchCmd.Flags().StringP("print", "p", "all", "Select how commands are presented to you (all, command, alias)")
	searchCmd.Flags().IntP("limit", "l", 5, "Limit the number of commands to display before scrolling (defaults to 10)")
}

func runSearch(cmd *cobra.Command, args []string) {
	dataAccessLayer, err := dal.NewDataAccessLayer()
	if err != nil {
		log.Fatal("Search Cmd: Failed to create dal", err)
		return
	}
	defer dataAccessLayer.CloseDataAccessLayer()

	searchFilters, printOption, limit, err := ExtractAndValidateSearchArgs(cmd)
	if err != nil {
		log.Fatal("Search Cmd: Invalid args", err)
		return
	}

	commands, err := dataAccessLayer.SearchForCommand(*searchFilters)
	if err == dal.InvalidSearchFiltersError {
		fmt.Println("Invalid search filters provided")
		return
	} else if err != nil {
		log.Fatal("Search Cmd: Failed to search for command", err)
		return
	}

	// Format the commands for printing
	formattedCommands := dal.FormatCommands(commands, *printOption)

	// Prompt the user to select a command
	prompt := promptui.Select{
		Label: "Select Command (" + dal.GetPrintedValues(*printOption) + ")",
		Items: formattedCommands,
		Size:  *limit,
	}
	item, _, err := prompt.Run()
	if err != nil {
		fmt.Printf("Search Comd: Prompt failed %v\n", err)
		return
	}

	// Copy the selected command to the clipboard
	if err = clipboard.Init(); err != nil {
		panic(err)
	}
	clipboard.Write(clipboard.FmtText, []byte(commands[item].Command))
	fmt.Println("Command added to clipboard!")

	// Update the command's usage statistics
	dataAccessLayer.UpdateCommandLastUsedById(commands[item].Id)
}
