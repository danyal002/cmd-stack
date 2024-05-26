/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"log"

	"github.com/manifoldco/promptui"
	"github.com/spf13/cobra"
)

// addCmd represents the add command
var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Save a command to your command stack",
	Long:  `missing_docs`,
	Args:  cobra.ExactArgs(1),
	Run:   addCommand,
}

func init() {
	rootCmd.AddCommand(addCmd)
	addCmd.Flags().StringP("alias", "a", "", "Name for the command")
	addCmd.Flags().StringP("tag", "t", "", "Tag for the command")
	addCmd.Flags().StringP("note", "n", "", "Note for the command")
}

func setCommandPropertiesWizard(cur_alias string, cur_tags string, cur_note string) (string, string, string, error) {
	prompt := promptui.Prompt{
		Label:     "Alias (Default is the command text)",
		Validate:  nil,
		AllowEdit: true,
		Default:   cur_alias,
	}
	alias, err := prompt.Run()
	if err != nil {
		log.Fatal("Add Cmd: Alias prompt failed:", err)
		return "", "", "", err
	}

	prompt = promptui.Prompt{
		Label:     "Tags (Separate with commas)",
		Validate:  nil,
		AllowEdit: true,
		Default:   cur_tags,
	}
	tags, err := prompt.Run()
	if err != nil {
		log.Fatal("Add Cmd: Tag prompt failed:", err)
		return "", "", "", err
	}

	prompt = promptui.Prompt{
		Label:     "Note",
		Validate:  nil,
		AllowEdit: true,
		Default:   cur_note,
	}
	note, err := prompt.Run()
	if err != nil {
		log.Fatal("Add Cmd: Note prompt failed:", err)
		return "", "", "", err
	}
	return alias, tags, note, nil
}

func addCommand(cmd *cobra.Command, args []string) {
	cmdText := args[0]
	if cmdText == "" {
		log.Fatal("Add Cmd: Command text is required")
		return
	}

	alias, _ := cmd.Flags().GetString("alias")
	tags, _ := cmd.Flags().GetString("tags")
	note, _ := cmd.Flags().GetString("note")

	// If no alias, tags, or note is provided, we will present the user with a form
	var err error
	if alias == "" && tags == "" && note == "" {
		alias, tags, note, err = setCommandPropertiesWizard(alias, tags, note)
		if err != nil {
			log.Fatal("Add Cmd: Wizard failed:", err)
			return
		}
	}

	// We must always have an alias. If one is not provided
	// we will use the command text as the alias
	if alias == "" {
		alias = cmdText
	}

	data_access_layer, err := dal.NewDataAccessLayer()
	if err != nil {
		log.Fatal("Add Cmd: Failed to launch DAL:", err)
		return
	}
	defer data_access_layer.CloseDataAccessLayer()

	err = data_access_layer.AddCommand(alias, cmdText, tags, note)
	if err != nil {
		log.Fatal("Add Cmd: Failed to add command:", err)
		return
	}
}
