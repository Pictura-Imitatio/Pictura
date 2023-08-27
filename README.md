<h1 align="center">
  <img src="pictura_baner_v1.png">
</h1>

Pictura is a minimal screenshot and text extraction tool

Features
--------
- [x] Written in Rust => Blazingy fast + Lightweight
- [ ] fullscreen screenshots
- [ ] area selection screenshots
- [ ] text extraction
- [ ] optional gui
- [ ] cli flags



## Usage
#### --help
```
USAGE: pictura {mode} {flags}
| ( TODO: modes and their flags here )
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

| Command | Description | 
| ---------------- | ---------------------- |
| --display-info    |   outputs global information display |
## Dependencies
`scrap=0.5.0`
`iced= { _FILL_IN_BLANKS_ }`
