use std::{io::Error, time::Instant, path::PathBuf, ffi::OsStr, fs::{read_dir, remove_file, metadata, create_dir}};
use walkdir::{DirEntry, WalkDir};

mod config;
mod img;

const IMG_EXTENSIONS: [&str; 3] = [".jpg", ".jpeg", ".png"];

fn main() -> Result<(), Error> {
    let cfg = config::Config::<PathBuf>::read()?;

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

    println!("Found {} files in {} ms.", flist.len(), start.elapsed().as_millis());
    
    let start = Instant::now();

    let sample = if flist.len() < 5 { flist } else {
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
        output_dir.push(i.to_string());
        output_dir.set_extension(fimg.path().extension().unwrap_or(OsStr::new("jpg")));
        if let Err (v) = cimg
                            .unwrap()
                            .save(output_dir) {
            println!("{}", v);
        }
    }

    println!("sampling and processing {} images in {} ms", sample.len(), start.elapsed().as_millis());

    Ok(())
}
