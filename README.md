# Splicer

Picture random pick-crop--save. Based on [Windows 10's Photos App Live Tile Changer](https://github.com/deadManAlive/tileupdater) porting to Rust.

Example of `config.toml` (place into working directory):
```toml
[config]
debug = true # optional, default to false
ouput = "output" # optional, default to "output"
locations = [
    "C:/Users/root/Pictures/",
    "D:/images",
    "E:/Family"
]
```
