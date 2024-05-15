/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
)

// updateCmd represents the update command
var updateCmd = &cobra.Command{
	Use:   "update",
	Short: "Search for and update a command in your command stack",
	Long:  `missing_docs`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("update called")
	},
}

func init() {
	rootCmd.AddCommand(updateCmd)
}
