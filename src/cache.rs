use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

cfg_dir = ProjectDirs::from("org", "team6479",  "signin").unwrap().config_dir();

// appends to a file in the cache directory
fn append(fname: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(cfg_dir.push(Path::new(fname)))
        .unwrap();
    writeln!(file, contents);
}

// empties a file in the cache directory
fn clear(fname: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(cfg_dir.push(Path::new(fname)))
        .unwrap();
    writeln!(file, "");
}