/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"errors"
	"fmt"
	"log"
	"slices"

	"github.com/manifoldco/promptui"
	"github.com/spf13/cobra"
	"golang.design/x/clipboard"
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

func searchArgsWizard() (*string, *string, *string, error) {
	fmt.Println("Specify the tags, commands, and/or aliases you'd like to see:")

	prompt := promptui.Prompt{
		Label:     "Tag",
		AllowEdit: true,
	}
	tag, err := prompt.Run()
	if err != nil {
		log.Fatal("Search Cmd: Failed to prompt for tag", err)
		return nil, nil, nil, err
	}

	prompt = promptui.Prompt{
		Label:     "Command",
		AllowEdit: true,
	}
	command, err := prompt.Run()
	if err != nil {
		log.Fatal("Search Cmd: Failed to prompt for command", err)
		return nil, nil, nil, err
	}

	prompt = promptui.Prompt{
		Label:     "Alias",
		AllowEdit: true,
	}
	alias, err := prompt.Run()
	if err != nil {
		log.Fatal("Search Cmd: Failed to prompt for alias", err)
		return nil, nil, nil, err
	}

	return &tag, &command, &alias, nil
}

func extractAndValidateArgs(cmd *cobra.Command) (*string, *string, *string, *string, *int, error) {
	tag, _ := cmd.Flags().GetString("tag")
	command, _ := cmd.Flags().GetString("command")
	alias, _ := cmd.Flags().GetString("alias")
	printOption, _ := cmd.Flags().GetString("print")
	limit, _ := cmd.Flags().GetInt("limit")

	// If no arguments are provided, we will present the user with a form
	if tag == "" && command == "" && alias == "" {
		newTag, newCommand, newAlias, err := searchArgsWizard()
		if err != nil {
			log.Fatal("Search Cmd: Failed to prompt for search args", err)
			return nil, nil, nil, nil, nil, err
		}
		tag = *newTag
		command = *newCommand
		alias = *newAlias
	}

	if !slices.Contains(dal.CmdPrintingOptions, printOption) {
		fmt.Println("Invalid print argument")
		log.Fatal("Search Cmd: Invalid print argument")
		return nil, nil, nil, nil, nil, errors.New("Invalid print argument")
	} else if limit < 5 || limit > 200 {
		fmt.Println("Invalid limit argument")
		log.Fatal("Search Cmd: Invalid limit argument")
		return nil, nil, nil, nil, nil, errors.New("Invalid limit argument")
	}

	return &tag, &command, &alias, &printOption, &limit, nil
}

func runSearch(cmd *cobra.Command, args []string) {
	dataAccessLayer, err := dal.NewDataAccessLayer()
	if err != nil {
		log.Fatal("Search Cmd: Failed to create dal", err)
		return
	}
	defer dataAccessLayer.CloseDataAccessLayer()

	tag, command, alias, printOption, limit, err := extractAndValidateArgs(cmd)
	if err != nil {
		log.Fatal("Search Cmd: Invalid args", err)
		return
	}

	commands, err := dataAccessLayer.SearchForCommand(dal.SearchFilters{
		Command: *command,
		Alias:   *alias,
		Tag:     *tag,
	})
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
