use std::{io::Error, time::Instant};
use walkdir::{DirEntry, WalkDir};

mod config;

const IMG_EXTENSIONS: [&str; 3] = [".jpg", ".jpeg", ".png"];

fn main() -> Result<(), Error> {
    let cfg = config::Config::read()?;

    let mut flist: Vec<DirEntry> = vec![];

    let start = Instant::now();

    for dir in cfg.locations {
        let mut files: Vec<DirEntry> = WalkDir::new(&dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|f| -> Option<DirEntry> {
                let f_name = f.file_name().to_string_lossy();

                for extension in IMG_EXTENSIONS {
                    if f_name.ends_with(extension) {
                        return Some(f);
                    }
                }
                None
            })
            .collect();

        flist.append(&mut files);
    }

    println!("found {} files in {} ms.", flist.len(), start.elapsed().as_millis());

    Ok(())
}
