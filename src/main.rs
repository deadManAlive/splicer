use std::io::Error;

use walkdir::WalkDir;

mod config;

const IMG_EXTENSIONS: [&str; 3] = [".jpg", ".jpeg", ".png"];

fn main() -> Result<(), Error> {
    let cfg = config::Config::read()?;

    for dir in cfg.locations {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let f_name = entry.path().to_string_lossy();

            for extension in IMG_EXTENSIONS {
                if f_name.ends_with(extension) {
                    println!("{}", f_name);
                }
            }
        }
    }

    Ok(())
}
