# Pstat

A Project Statistics TUI built in Rust. Get all important stats of your project with a single glance: 
  - Project directory structure
  - Language distribution
  - File stats
  - Recently modified files
  - Git stats

Currently supports basic projects with language support for Go, Python and Rust. Support for other languages will be coming soon.

## Usage
To open the TUI: ```cargo run -- PATH_TO_PROJECT```

Switch Tabs using the left and right arrows

Use the up and down arrows to scroll the project tree

To exit, press: ```q```

## Screenshots
Note that the color of the terminal is subject to your color scheme and the background is not included :)

![alt text](https://github.com/SRP457/pstat/blob/main/screenshots/home.png?raw=true)
![alt text](https://github.com/SRP457/pstat/blob/main/screenshots/git.png?raw=true)
