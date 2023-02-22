# Splicer

Picture random pick-crop--save. Based on [Windows 10's Photos App Live Tile Changer](https://github.com/deadManAlive/tileupdater) porting to Rust.

Example of `config.toml` (place into working directory):
```toml
[config]
# Sets wether to enable log
# Defaults to true
log = true

# Sets where output folder is.
# Delete/comment this key to load to the app folder.
# Invalid value defaults to folder "output" of working directory.
output = "output"

# List of locations to look up for images.
locations = [
    "C:/Users/root/Pictures/",
    "D:/images",
    "E:/Family"
]
```
