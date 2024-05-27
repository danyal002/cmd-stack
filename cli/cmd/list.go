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
	"slices"
)

// listCmd represents the list command
var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List your most recently used commands in command stack",
	Long:  `missing_docs`,
	Run: func(cmd *cobra.Command, args []string) {
		use_recent, _ := cmd.Flags().GetBool("recent")
		resultLimit, _ := cmd.Flags().GetInt("result-limit")
		printOption, _ := cmd.Flags().GetString("print")
		printLimit, _ := cmd.Flags().GetInt("print-limit")
		if !slices.Contains(dal.CmdPrintingOptions, printOption) {
			fmt.Println("Invalid print argument")
			log.Fatal("Search Cmd: Invalid print argument")
			return
		}

		dataAccessLayer, err := dal.NewDataAccessLayer()
		if err != nil {
			log.Fatal("List Command: Failed to get Data Access Layer: ", err)
			return
		}
		defer dataAccessLayer.CloseDataAccessLayer()

		commands, err := dataAccessLayer.GetCommands(resultLimit, use_recent)
		if err != nil {
			log.Fatal("List Command: Failed to retrieve commands from the database: ", err)
			return
		}

		// Format the commands for printing
		formattedCommands := dal.FormatCommands(commands, printOption)

		// Prompt the user to select a command
		prompt := promptui.Select{
			Label: "Select Command (" + dal.GetPrintedValues(printOption) + ")",
			Items: formattedCommands,
			Size:  printLimit,
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
	},
}

func init() {
	rootCmd.AddCommand(listCmd)
	listCmd.Flags().BoolP("recent", "r", false, "List most recently used commands")
	listCmd.Flags().IntP("result-limit", "l", 25, "Limit the number of commands listed")
	listCmd.Flags().StringP("print", "p", "all", "Select how commands are presented to you (all, command, alias)")
	listCmd.Flags().IntP("print-limit", "", 10, "Select how many commands are presented to you")
}
