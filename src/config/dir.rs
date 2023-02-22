use dirs::data_local_dir;
use std::{fs, path::PathBuf};

pub fn get_dir() -> Option<PathBuf> {
    let app_parent = data_local_dir()?.join("Packages");
    let photos_app = fs::read_dir(app_parent)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| {
            p.to_str()
                .map(|s| s.contains(".Windows.Photos_"))
                .unwrap_or(false)
        })?;

    let app_dir = photos_app.join("LocalState").join("PhotosAppTile");

    if app_dir.is_dir() {
        Some(app_dir)
    } else {
        None
    }
}
