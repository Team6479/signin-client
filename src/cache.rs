use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::path::Path;
use std::fs;

cfg_dir = ProjectDirs::from("org", "team6479",  "signin").unwrap().config_dir(); // all methods here are relative to the cache directory

fn append(fname: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(cfg_dir.push(Path::new(fname)))
        .unwrap();
    writeln!(file, contents);
}

fn create(fname: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(cfg_dir.push(Path::new(fname)))
        .unwrap();
    writeln!(file, contents);
}

fn del(fname: &str) {
    fs::remove_file(cfg_dir.push(Path::new(fname)));
}

fn clear(fname: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(cfg_dir.push(Path::new(fname)))
        .unwrap();
    writeln!(file, "");
}

fn exists(fname: &str) -> bool {
    Path::new(cfg_dir.push(Path::new(fname))).exists()
}