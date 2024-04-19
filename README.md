### ⚠️ This project is currently under active development.

# CmdStack: Your Lightning-Fast Command Line Notebook

## Overview

CmdStack is a powerful yet user-friendly tool designed to simplify the way developers manage their notes and commands. Say goodbye to scattered text files and hello to organized, efficient note-taking. With a command line app written in Go and a user-friendly frontend in TypeScript with Tauri, CmdStack offers a seamless experience across platforms.

## Features

-   **Blazing Speed:** CmdStack is engineered for speed and efficiency. Save commands as quickly as you type them in the terminal, reducing friction in your workflow and ensuring nothing is missed.

-   **Cross-Platform Compatibility:** CmdStack's Go-based command line app ensures it runs smoothly on various operating systems, making it accessible to developers on Windows, macOS, and Linux.

-   **User-Friendly GUI:** The TypeScript and Electron-powered frontend provides a beautiful and intuitive graphical user interface, making it easy for both command line enthusiasts and those who prefer a visual approach.

-   **Effortless Organization:** Keep your notes and commands neatly organized within CmdStack. Say goodbye to scattered, hard-to-find files and enjoy a streamlined note-taking experience.

-   **Tagging and Subtagging:** Organize your commands with ease by using tags and subtags. For example, `docker/network`, tags the command with `docker` as the root tag and `network` as the subtag to find it quickly when needed.
-   **Utilizes RedB:** a lightning-fast key-value storage system, ensuring optimal performance and data integrity.

## Usage

CmdStack offers the following features for efficient command and note management:

-   **Add a Command:** Quickly save commands along with tags and notes for reference.

```shell
cmdstack add "{command}" -t "{tag}" -n "{note}"
```

-   **Delete a Command by ID:** Remove specific commands by their unique ID.

```shell
cmdstack delete -i {ID}
```

-   **Search Commands, Notes, or IDs:** Easily locate relevant commands, notes, or IDs.

```shell
cmdstack search "{text to search}"
```

-   **List Commands by Tag:** Organize your commands by tags and subtags for a structured view.

```shell
cmdstack list -t "{tag path including subtags}"
```

## Why CmdStack?

CmdStack is the ideal solution for developers who want to:

-   Boost Productivity: Spend less time searching for notes and more time coding, with lightning-fast command saving.
-   Stay Organized: Keep all your development-related notes and commands in one place, with easy-to-use tagging and subtagging.
-   Enjoy Cross-Platform Flexibility: CmdStack works smoothly on Windows, macOS, and Linux.
-   Embrace an Intuitive Interface: Whether you love the command line or prefer a graphical interface, CmdStack has you covered.

## Contribute

### Future Planned Features

CmdStack has exciting future features in the pipeline, including:

-   **Import/Export Notes to JSON:** Transfer your notes in and out of CmdStack for backup and sharing.
-   **Local Encryption:** Keep your data secure with local encryption options.
-   **Cloud API Integration:** Sync your notes to the cloud to ensure you never lose them.
-   **Workspaces:** Organize your notes into separate sections, projects, or personal and workspaces for enhanced productivity.

CmdStack is open-source and community-driven. Join us in making development notes and commands more accessible and efficient. Get started today by cloning the repository, exploring our documentation, and contributing to the project!

[![GitHub Repo](https://img.shields.io/badge/GitHub-Repository-blue.svg)](https://github.com/danyal002/cmd-stack)

CmdStack - Taking the complexity out of command line note-taking and organization, with the speed you need, the power of RedB, and a bright future ahead.
