use std::fs;
use std::path::PathBuf;

use log::info;

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

/// Return a PathBuf pointing at the save directory and make sure it exists.
/// This will panic if it can not create the directory.
/// This will panic if the home dir of the current user can not be detected.
pub fn enforced_save_path() -> PathBuf {
    let result = enforced_data_root().join("saves");

    if !result.exists() {
        // by the time we call this we should have logging setup
        info!("Creating save dir {:?}", &result);
        fs::create_dir_all(&result)
            .unwrap_or_else(|e| panic!("Could not create save directory {:?}: {:?}", result, e));
    }

    if !result.is_dir() {
        panic!("{:?} already exists but is not a directory", result);
    }

    result
}

/// First letter must be alphabetic and the rest must be either alphanumeric or a space or an underscore or a dash
pub fn valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().map_or_else(|| false, |c| c.is_alphabetic())
        && chars.all(|c| c.is_alphanumeric() || c == ' ' || c == '_' || c == '-')
}

#[cfg(test)]
mod tests {
    use crate::utils::valid_name;

    #[test]
    fn test_valid_name() {
        assert!(valid_name("fish"));
        assert!(valid_name("fish car 2456"));
        assert!(valid_name("fish_car"));
        assert!(valid_name("fish-car"));
        assert!(!valid_name("fish&car"));
        assert!(valid_name("_____________car"));
    }
}
