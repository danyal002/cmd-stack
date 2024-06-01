package cmd

import (
	"cmdstack/dal"
	"errors"
	"fmt"
	"github.com/spf13/cobra"
	"log"
	"slices"

	"github.com/manifoldco/promptui"
)

func SearchArgsWizard() (*string, *string, *string, error) {
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

func ExtractAndValidateSearchArgs(cmd *cobra.Command) (*dal.SearchFilters, *string, *int, error) {
	tag, _ := cmd.Flags().GetString("tag")
	command, _ := cmd.Flags().GetString("command")
	alias, _ := cmd.Flags().GetString("alias")
	printOption, _ := cmd.Flags().GetString("print")
	limit, _ := cmd.Flags().GetInt("limit")

	// If no arguments are provided, we will present the user with a form
	if tag == "" && command == "" && alias == "" {
		newTag, newCommand, newAlias, err := SearchArgsWizard()
		if err != nil {
			log.Fatal("Search Cmd: Failed to prompt for search args", err)
			return nil, nil, nil, err
		}
		tag = *newTag
		command = *newCommand
		alias = *newAlias
	}

	if !slices.Contains(dal.CmdPrintingOptions, printOption) {
		fmt.Println("Invalid print argument")
		log.Fatal("Search Cmd: Invalid print argument")
		return nil, nil, nil, errors.New("Invalid print argument")
	} else if limit < 5 || limit > 200 {
		fmt.Println("Invalid limit argument")
		log.Fatal("Search Cmd: Invalid limit argument")
		return nil, nil, nil, errors.New("Invalid limit argument")
	}

	return &dal.SearchFilters{
		Tag:     tag,
		Command: command,
		Alias:   alias,
	}, &printOption, &limit, nil
}
