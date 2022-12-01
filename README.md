# pstat
Do you want to know how many lines of code you've written, see how your project structure looks, what languages you've used to code, see recently modified files, all in one glance? 

pstat (Project Stat), is a TUI built in Rust. Get all important stats of your project with a single glance!
Using a Git repo? Get all important stats on one screen! 

pstat currently supports:
  - Project directory structure
  - Language distribution
  - File stats
  - Recently modified files
  - Git stats

Currently supports basic projects and languages. Support for other languages will be coming soon.

## Setup
To use pstat, you'll need to have `Rust` or `Cargo` installed. 
```
cargo run --release
```

If you want to use the binary run:
```
cargo build --release
```
The pstat binary will be created at `target/release/pstat`
You can add this binary to PATH and use `pstat` to run the TUI.


## Usage
```
USAGE:
    pstat [OPTIONS]

OPTIONS:
    -c, --color <color>    Color scheme to use
    -h, --help             Print help information
    -i, --ignore           Use .gitignore if exists
    -p, --path <path>      Path to project directory (Defaults to current path)
    -V, --version          Print version information
   
Navigation:
    Up, Down            Scroll project tree or git status
    Left, Right         Switch between tabs
    v                   Toggle git log graph
    q                   Quit

Colors:
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray 
    DarkGray, LightRed, LightGreen, LightYellow, LightBlue, 
    LightMagenta, LightCyan, White 
```
Examples:
```
// Using cargo  
cargo run --release -- -p path/to/project

// Using binary
pstat -p /path/to/project
```

## Demo

Note that the color and background of the terminal is subject to your color scheme and settings.

![alt text](https://github.com/SRP457/pstat/blob/main/screenshots/home.png?raw=true)
![alt text](https://github.com/SRP457/pstat/blob/main/screenshots/git.png?raw=true)
