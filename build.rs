extern crate ureq;
extern crate walkdir;

use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // locate executable path even if the project is in workspace
    let executable_path = locate_target_dir_from_output_dir(&out_dir)
        .expect("failed to find target dir")
        .join(env::var("PROFILE").unwrap());

    // Download fonts and put into resources/static/fonts
    let fonts_dir = manifest_dir.join("resources").join("static").join("fonts");
    let font_list_path = manifest_dir.join("font_list.txt");
    check_fonts(&font_list_path, &fonts_dir).unwrap();

    // Copy the resources directory into the build dir
    copy(&manifest_dir.join("resources"), &executable_path);

    copy(
        &manifest_dir.join("config"),
        &executable_path.join("config"),
    );
}

fn locate_target_dir_from_output_dir(mut target_dir_search: &Path) -> Option<&Path> {
    loop {
        // if path ends with "target", we assume this is correct dir
        if target_dir_search.ends_with("target") {
            return Some(target_dir_search);
        }

        // otherwise, keep going up in tree until we find "target" dir
        target_dir_search = match target_dir_search.parent() {
            Some(path) => path,
            None => break,
        }
    }

    None
}

fn copy(from: &Path, to: &Path) {
    let from_path: PathBuf = from.into();
    let to_path: PathBuf = to.into();
    if !from_path.exists() {
        println!("Copy path {:?} does not exist. Skipping", from_path);
        println!("cargo::rerun-if-changed={:?}", from);
        return;
    }

    for entry in WalkDir::new(from_path.clone()) {
        let entry = entry.unwrap();

        if let Ok(rel_path) = entry.path().strip_prefix(&from_path) {
            let target_path = to_path.join(rel_path);

            if entry.file_type().is_dir() {
                DirBuilder::new()
                    .recursive(true)
                    .create(target_path)
                    .expect("failed to create target dir");
            } else {
                fs::copy(entry.path(), &target_path).expect("failed to copy");
            }
        }
    }
    println!("cargo::rerun-if-changed={:?}", from);
}

fn check_fonts(font_list_path: &Path, font_path: &Path) -> Result<(), String> {
    // There is no font list so do nothing...
    if !font_list_path.exists() {
        println!("Missing font list. Doing nothing");
        return Ok(());
    }

    // load font list
    for line in fs::read_to_string(font_list_path)
        .map_err(|e| format!("could not read file {:?}: {:?}", &font_list_path, e))?
        .lines()
    {
        // lines should be urls. but allow comments with # on the front
        if line.starts_with("#") {
            continue;
        }
        let font_name_start = line
            .rfind("/")
            .ok_or(format!("Could not find file name in font url: {}", line))?;
        let font_name: String = line.chars().skip(font_name_start + 1).collect();
        let target_path = font_path.join(&font_name);
        println!("font_name: {:?}", &font_name);
        println!("font_path: {:?}", font_path);
        println!("target_path: {:?}", target_path);
        // Download the file, this will skip if the file already exists.
        download(line, &target_path)?;
    }

    println!("cargo::rerun-if-changed={:?}", font_list_path);

    Ok(())
}

fn download(from: &str, to: &Path) -> Result<(), String> {
    println!("to path: {:?}", to);
    // exit if the file already exists.
    if to.exists() {
        return Ok(());
    }

    // make sure the target directory exists.
    if !to.parent().unwrap().exists() {
        fs::create_dir_all(to.parent().unwrap()).map_err(|error| {
            format!(
                "could not find parent dir: {:?}: {:?}",
                to,
                error.to_string()
            )
        })?;
    }

    let response = ureq::get(from)
        .call()
        .map_err(|error| format!("could not get request: {from}: {:?}", error.to_string()))?;

    // Only accept 2xx status codes
    if !(200..300).contains(&response.status()) {
        return Err(format!("Download error: HTTP {}", response.status()));
    }

    let mut body = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut body)
        .map_err(|error| format!("Could not read response {:?}", error.to_string()))?;

    // write the file into the source tree.
    fs::write(to, body).map_err(|error| {
        format!(
            "could not write output to {:?}: {:?}",
            to,
            error.to_string()
        )
    })?;

    Ok(())
}
