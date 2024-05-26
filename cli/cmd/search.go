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
}

func searchArgsWizard(cur_tag string, cur_command string, cur_alias string) (string, string, string, string, error) {
	fmt.Println("Specify the tags, commands, and/or aliases you'd like to see:")

	prompt := promptui.Prompt{
		Label:     "Tag",
		AllowEdit: true,
		Default:   cur_tag,
	}
	tag, err := prompt.Run()
	if err != nil {
		log.Fatal("Search Cmd: Failed to prompt for tag", err)
		return "", "", "", "", err
	}

	prompt = promptui.Prompt{
		Label:     "Command",
		AllowEdit: true,
		Default:   cur_command,
	}
	command, err := prompt.Run()
	if err != nil {
		log.Fatal("Search Cmd: Failed to prompt for command", err)
		return "", "", "", "", err
	}

	prompt = promptui.Prompt{
		Label:     "Alias",
		AllowEdit: true,
		Default:   cur_alias,
	}
	alias, err := prompt.Run()
	if err != nil {
		log.Fatal("Search Cmd: Failed to prompt for alias", err)
		return "", "", "", "", err
	}

	printOption := "all"
	// TODO: Decide if we want to show this option
	//sel := promptui.Select{
	//	Label: "Select printing style",
	//	Items: dal.CmdPrintingOptions,
	//}
	//_, printOption, err := sel.Run()
	//if err != nil {
	//	log.Fatal("Search Cmd: Failed to prompt for print option", err)
	//	return "", "", "", "", err
	//}

	return tag, command, alias, printOption, nil
}

func extractAndValidateArgs(cmd *cobra.Command) (string, string, string, string, error) {
	tag, _ := cmd.Flags().GetString("tag")
	command, _ := cmd.Flags().GetString("command")
	alias, _ := cmd.Flags().GetString("alias")
	printOption, _ := cmd.Flags().GetString("print")

	// If no arguments are provided, we will present the user with a form
	var err error
	if tag == "" && command == "" && alias == "" {
		tag, command, alias, printOption, err = searchArgsWizard(tag, command, alias)
		if err != nil {
			log.Fatal("Search Cmd: Failed to prompt for search args", err)
			return "", "", "", "", err
		}
	}

	if !slices.Contains(dal.CmdPrintingOptions, printOption) {
		fmt.Println("Invalid print argument")
		log.Fatal("Search Cmd: Invalid print argument")
		return "", "", "", "", errors.New("Invalid print argument")
	}

	return tag, command, alias, printOption, nil
}

// Get an initial set of commands from the database based on the CLI command arguments
func getInitialCommands(command string, alias string, tag string, dataAccessLayer *dal.DataAccessLayer) ([]dal.Command, error) {
	/*
		We search in the following order:
		1. Search by tag
		2. Search by command
		3. Search by alias
	*/
	commands := []dal.Command{}
	var err error
	if tag != "" {
		commands, err = dataAccessLayer.SearchByTag(tag)
		if err != nil {
			log.Fatal("Search Cmd: Failed to search for command by tag", err)
			return nil, err
		}
	}

	if len(commands) > 0 && command != "" {
		commands = dal.FilterCommandsByCommand(commands, command)
	} else if command != "" {
		commands, err = dataAccessLayer.SearchByCommand(command)
		if err != nil {
			log.Fatal("Search Cmd: Failed to search for command by command", err)
			return nil, err
		}
	}

	if len(commands) > 0 && alias != "" {
		commands = dal.FilterCommandsByAlias(commands, alias)
	} else if alias != "" {
		commands, err = dataAccessLayer.SearchByAlias(alias)
		if err != nil {
			log.Fatal("Search Cmd: failed to search for command by alias", err)
			return nil, err
		}
	}
	return commands, nil
}

func runSearch(cmd *cobra.Command, args []string) {
	dataAccessLayer, err := dal.NewDataAccessLayer()
	if err != nil {
		log.Fatal("Search Cmd: Failed to create dal", err)
		return
	}
	defer dataAccessLayer.CloseDataAccessLayer()

	tag, command, alias, printOption, err := extractAndValidateArgs(cmd)
	if err != nil {
		log.Fatal("Search Cmd: Invalid args", err)
		return
	}

	// Get the initial set of commands
	commands, err := getInitialCommands(command, alias, tag, dataAccessLayer)
	if err != nil {
		log.Fatal("Search Cmd:", err)
		return
	} else if len(commands) == 0 {
		fmt.Println("No Commands Found...")
		return
	}

	// Format the commands for printing
	formattedCommands := dal.FormatCommands(commands, printOption)

	// Prompt the user to select a command
	prompt := promptui.Select{
		Label: "Select Command (" + dal.GetPrintedValues(printOption) + ")",
		Items: formattedCommands,
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
