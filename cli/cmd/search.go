/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"cmdstack/dal"
	"fmt"

	"github.com/spf13/cobra"
)

// searchCmd represents the search command
var searchCmd = &cobra.Command{
	Use:   "search",
	Short: "missing_docs",
	Long:  `missing_docs`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		searchText := args[0]
		fmt.Println(dal.Search(searchText))
	},
}

func init() {
	rootCmd.AddCommand(searchCmd)
}
