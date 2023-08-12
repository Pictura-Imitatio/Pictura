# Pictura
minimal screenshot and text extraction tool
## Modes
#### --help
```
USAGE: pictura {mode} {flags}
| modes and their flags here
EXAMPLES:
pictura --image -cp
pictura --text -v -o "~/Pictures"
pictura --text -cp -t 5
```
#### --image
| Command                   | Description            |
| ------------------------- | ---------------------- |
| -o  {filepath}, --output | save to dir at path    |
| -cp, --clipboard          | copy to clipboard      |
| -t {seconds}, --time      | delay in seconds       |
#### --text
| Command                   | Description            |
| ------------------------- | ---------------------- |
| -o  {filepath}, --output  | save to dir at path    |
| -v, --verbose             | output to the terminal |
| -cp, --clipboard          | copy to clipboard      |
| -t {seconds}, --time      | delay in seconds       | 

## Dependencies
`scrap=0.5.0`