# Splicer

Windows 10's Photos App Live Tile Changer ported to Rust.

Example of `config.toml` (place into working directory):
```toml
[config]
debug = true # optional, default to false
locations = [
    "C:/Users/root/Pictures/",
    "D:/images",
    "E:/Family"
]
ouput = "output"
```
