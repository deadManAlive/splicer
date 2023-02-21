use std::{io::Error, time::Instant};
use walkdir::{DirEntry, WalkDir};

mod config;
mod img;

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
    
    let start = Instant::now();

    let sample = if flist.len() < 5 { flist } else {
        let mut rng = rand::thread_rng();
        rand::seq::index::sample(&mut rng, flist.len(), 5)
            .iter()
            .map(|i| flist[i].clone())
            .collect()
    };


    for (i, fimg) in sample.iter().enumerate() {
        let cimg = img::crop(fimg.path());
        if let Err (v) = cimg.unwrap().save(format!("./output/{}.png", i)) {
            println!("{}", v);
        }
    }

    println!("sampling and processing {} images in {} ms", sample.len(), start.elapsed().as_millis());

    Ok(())
}
