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
- Create an example `settings.ron` file
- Creating a working ron settings
- Use CLAP multicall to create a frontend for the toolchain