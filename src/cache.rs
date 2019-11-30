use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;

fn cfg_dir() -> PathBuf { // all methods here are relative to the cache directory
    ProjectDirs::from("org", "team6479",  "signin").unwrap().config_dir().to_path_buf()
}

fn append(fname: &str, contents: &str) {
    let mut fpath = cfg_dir();
    fpath.push(Path::new(fname));
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&fpath.as_path())
        .unwrap();
    writeln!(file, "{}", contents);
}

fn create(fname: &str, contents: &str) {
    let mut fpath = cfg_dir();
    fpath.push(Path::new(fname));
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&fpath.as_path())
        .unwrap();
    writeln!(file, "{}", contents);
}

fn del(fname: &str) {
    let mut fpath = cfg_dir();
    fpath.push(Path::new(fname));
    fs::remove_file(&fpath.as_path());
}

fn clear(fname: &str) {
    let mut fpath = cfg_dir();
    fpath.push(Path::new(fname));
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&fpath.as_path())
        .unwrap();
    writeln!(file, "");
}

fn exists(fname: &str) -> bool {
    let mut fpath = cfg_dir();
    fpath.push(Path::new(fname));
    Path::new(&fpath.as_path()).exists()
}