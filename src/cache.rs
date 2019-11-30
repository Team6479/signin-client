use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
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

fn read(fname: &str) -> String {
    let mut fpath = cfg_dir();
    fpath.push(Path::new(fname));
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&fpath.as_path())
        .unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    contents
}

pub fn is_signed_in(id: &str) -> bool {
    exists(&format!("sess/{}", id))
}

pub fn mk_sess(id: &str, start: u64) {
    create(&format!("sess/{}", id), &format!("{}", start));
}

pub fn get_sess_start(id: &str) -> u64 {
    read(&format!("sess/{}", id)).parse::<u64>().unwrap()
}

pub fn rm_and_get_sess(id: &str) -> u64 {
    let start = get_sess_start(&id);
    del(&format!("sess/{}", id));
    start
}