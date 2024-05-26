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

func extractAndValidateArgs(cmd *cobra.Command, args []string) (string, string, string, string, error) {
	command, _ := cmd.Flags().GetString("command")
	alias, _ := cmd.Flags().GetString("alias")
	tag, _ := cmd.Flags().GetString("tag")
	printOption, _ := cmd.Flags().GetString("print")

	if !slices.Contains(dal.CmdPrintingOptions, printOption) {
		fmt.Println("Invalid print argument")
		log.Fatal("Search Cmd: Invalid print argument")
		return "", "", "", "", errors.New("Invalid print argument")
	}

	return command, alias, tag, printOption, nil
}

// Get an initial set of commands from the database based on the CLI command arguments
func getInitialCommands(command string, alias string, tag string, data_access_layer *dal.DataAccessLayer) ([]dal.Command, error) {
	/*
		We search in the following order:
		1. Search by tag
		2. Search by command
		3. Search by alias
	*/
	commands := []dal.Command{}
	var err error
	if tag != "" {
		commands, err = data_access_layer.SearchByTag(tag)
		if err != nil {
			log.Fatal("Search Cmd: Failed to search for command by tag", err)
			return nil, err
		}
	}

	if len(commands) > 0 && command != "" {
		commands = dal.FilterCommandsByCommand(commands, command)
	} else if command != "" {
		commands, err = data_access_layer.SearchByCommand(command)
		if err != nil {
			log.Fatal("Search Cmd: Failed to search for command by command", err)
			return nil, err
		}
	}

	if len(commands) > 0 && alias != "" {
		commands = dal.FilterCommandsByAlias(commands, alias)
	} else if alias != "" {
		commands, err = data_access_layer.SearchByAlias(alias)
		if err != nil {
			log.Fatal("Search Cmd: failed to search for command by alias", err)
			return nil, err
		}
	}
	return commands, nil
}

func runSearch(cmd *cobra.Command, args []string) {
	data_access_layer, err := dal.NewDataAccessLayer()
	if err != nil {
		log.Fatal("Search Cmd: Failed to create dal", err)
		return
	}
	defer data_access_layer.CloseDataAccessLayer()

	command, alias, tag, printOption, err := extractAndValidateArgs(cmd, args)
	if err != nil {
		log.Fatal("Search Cmd: Invalid args", err)
		return
	}

	// Get the initial set of commands
	commands, err := getInitialCommands(command, alias, tag, data_access_layer)
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
	data_access_layer.UpdateCommandLastUsedById(commands[item].Id)
}
