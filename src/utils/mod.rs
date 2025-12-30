use std::fs;
use std::path::PathBuf;

use crate::APPLICATION_NAME;

pub fn data_dir() -> PathBuf {
    ::dirs::config_local_dir()
        .expect("Can not find local config path! Giving up")
        .join(APPLICATION_NAME)
}

/// Return a PathBuf pointing at the system root dir ($HOME/.config/$APPLICATION_NAME) and make sure that it exists.
/// This will panic if it can not create the directory.
/// This will panic if the home dir of the current user can not be detected.
pub fn enforced_data_root() -> PathBuf {
    let result = data_dir();
    if !result.exists() {
        println!(
            "Data directory {} not found. Creating it",
            &result.display()
        );
        fs::create_dir_all(&result)
            .unwrap_or_else(|e| panic!("Could not create data directory {:?}: {:?}", result, e));
    }

    if !result.is_dir() {
        panic!("{:?} already exists but is not a directory", result);
    }

    result
}
