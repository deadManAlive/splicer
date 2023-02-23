use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

use toml::Value;

pub mod dir;

#[derive(Debug)]
pub struct Config<P: AsRef<Path>> {
    pub log: bool,
    pub locations: Vec<String>,
    pub output: P,
}

fn mk_io_err(error: &str) -> Error {
    Error::new(ErrorKind::Other, error)
}

impl<P: AsRef<Path>> Config<P> {
    pub fn read() -> Result<Config<PathBuf>, Error> {
        let mut config_file = File::open("./config.toml")?;
        let mut config_contents = String::new();

        config_file.read_to_string(&mut config_contents)?;

        let config: Value = match config_contents.parse::<Value>() {
            Ok(v) => v,
            Err(_) => return Err(mk_io_err("error reading config.toml")),
        };

        let log = match config.get("config").and_then(|cfg| cfg.get("log")) {
            Some(v) => v.as_bool().unwrap_or(true),
            None => false,
        };

        let locations = match config.get("config").and_then(|cfg| cfg.get("locations")) {
            Some(v) => match v.as_array() {
                Some(w) => w.to_owned(),
                None => return Err(mk_io_err("'locations' key is not in valid format (array)")),
            },
            None => return Err(mk_io_err("cannot find 'locations' key in config file")),
        };

        let output = match config.get("config").and_then(|cfg| cfg.get("output")) {
            Some(v) => {
                let d = v.as_str().unwrap_or("output");
                PathBuf::from(d)
            }
            None => dir::get_dir().unwrap_or(PathBuf::from("output")),
        };
        // let output = PathBuf::from(output);

        let locations = locations
            .iter()
            .filter_map(|v| v.as_str())
            .map(|v| v.to_owned())
            .filter_map(|p| -> Option<String> {
                if Path::new(&p).is_dir() {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();

        Ok(Config {
            log,
            locations,
            output,
        })
    }
}
