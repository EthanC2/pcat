# PCAT: Programming Competition Automation Toolchain

# Commands
| Subcommand | Action |
| ------- | ------ |
| `new <language> <filename>` | creates a new copy of the main program template `<language>` named `<filename>.<language_extension>` |
| `test <filename>.test` | runs all the unit tests in `<filename>.test` on the given exe |
| `comp <filename>` | compiles `<filename>`; any other arguments are passed to the compiler |

# Customization and Settings
pcat's settings are located at `$HOME/.pcat/settings.ron`

# Todo
- Work on the PEST parser for unit tests
- Add a setting to auto-generate headers for tests (--number 3, -n3?)

# Platform Support
This program should work on any Unix-based OS, but can be ported to any OS by creating the
necessary environment variables ($HOME, $EDITOR, etc).

# Installation and Setup
Installation is as easy as downloading the install.sh file (Linux, MacOS) or install.ps1
(Windows) and running the script. From there, feel free to edit the $HOME/.pcat/settings.ron
file to your liking.

# Creating a Program from a Template with `new`

# Compiling a Program with `compile`
`comp` (short for `compile), compiles a program given the settings

# Testing a Program with `test`