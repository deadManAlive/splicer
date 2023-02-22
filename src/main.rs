use std::{
    ffi::OsStr,
    fs::{create_dir, metadata, read_dir, remove_file, OpenOptions},
    io::{Write, Error},
    path::PathBuf,
    time::Instant,
};
use walkdir::{DirEntry, WalkDir};
use chrono::Local;

mod config;
mod img;

const IMG_EXTENSIONS: [&str; 3] = [".jpg", ".jpeg", ".png"];

fn main() -> Result<(), Error> {
    let cfg = config::Config::<PathBuf>::read()?;

    let mut flist: Vec<DirEntry> = vec![];

    let start = Instant::now();

    for dir in &cfg.locations {
        let mut files: Vec<DirEntry> = WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|f| -> Option<DirEntry> {
                let f_name = f.file_name().to_string_lossy().to_ascii_lowercase();

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

    let sample = if flist.len() < 5 {
        flist.clone()
    } else {
        let mut rng = rand::thread_rng();
        rand::seq::index::sample(&mut rng, flist.len(), 5)
            .iter()
            .map(|i| flist[i].clone())
            .collect()
    };

    if !metadata(&cfg.output)?.is_dir() {
        create_dir(&cfg.output)?;
    }

    for files in (read_dir(&cfg.output)?).flatten() {
        if let Err(v) = remove_file(files.path()) {
            println!("{}", v);
        }
    }

    for (i, fimg) in sample.iter().enumerate() {
        let cimg = img::crop(fimg.path());
        let mut output_dir = cfg.output.clone();
        output_dir.push(format!("Tile{}", i));
        output_dir.set_extension(fimg.path().extension().unwrap_or(OsStr::new("jpg")).to_ascii_lowercase());
        if let Err(v) = cimg
            .unwrap()
            .save(output_dir.to_string_lossy().to_string())
        {
            println!("{}", v);
        }
    }

    if cfg.log {
        if let Ok(v) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt") {
                let now = Local::now().format("%Y-%m-%d %H:%M:%S (%a)").to_string();
    
                let mut logfile = v;
    
                writeln!(logfile, "{}: Done sampling {} from {} found in {} ms.", now, sample.len(), flist.len(), start.elapsed().as_millis())?;
            }
    }

    Ok(())
}
