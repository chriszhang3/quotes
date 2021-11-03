# Parse and display quotes

This is my first rust project. It is a CLI for parsing and searching text files of quotes.
Files should have two linebreaks in between quotes. Otherwise use the --slb flag.
A single quote may span multiple lines and have multiple authors.

## Quickstart
// Build the binary. Requires [cargo](https://doc.rust-lang.org/book/ch01-01-installation.html). You should be in the same directory as the Cargo.toml file.
```
cargo install --path .
```
// List all quotes.
```
quotes list sample.txt 
```
// Search for quotes by keyword.
```
quotes list -s life sample.txt
```
// Count quotes by author.
```
quotes count sample.txt
```
// Convert text file to have correct formatting.
```
quotes write -slb incorrect_formatting.txt new_file.txt
```
// Instructions on how to use the CLI.
```
quotes --help 
```