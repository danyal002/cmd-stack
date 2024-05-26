/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"github.com/spf13/cobra"
	"log"
)

// addCmd represents the add command
var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Save a command to your command stack",
	Long:  `missing_docs`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		cmdText := args[0]
		alias, _ := cmd.Flags().GetString("alias")
		tags, _ := cmd.Flags().GetString("tag")
		note, _ := cmd.Flags().GetString("note")

		// We must always have an alias. If one is not provided
		// we will use the command text as the alias
		if alias == "" {
			alias = cmdText
		}

		dataAccessLayer, err := dal.NewDataAccessLayer()
		if err != nil {
			log.Fatal(err)
			return
		}
		defer dataAccessLayer.CloseDataAccessLayer()

		err = dataAccessLayer.AddCommand(alias, cmdText, tags, note)
		if err != nil {
			log.Fatal(err)
			return
		}
	},
}

func init() {
	rootCmd.AddCommand(addCmd)
	addCmd.Flags().StringP("alias", "a", "", "Name for the command")
	addCmd.Flags().StringP("tag", "t", "", "Tag for the command")
	addCmd.Flags().StringP("note", "n", "", "Note for the command")
}
