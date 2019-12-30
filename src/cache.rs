use directories::ProjectDirs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fs;

fn cache_dir() -> PathBuf { // all methods here are relative to the cache directory
    ProjectDirs::from("org", "team6479",  "signin").unwrap().cache_dir().to_path_buf()
}

pub fn init() {
    let mut sess_path = cache_dir();
    sess_path.push(Path::new("sess"));
    fs::create_dir_all(sess_path);
}

fn append(fname: &str, contents: &str) {
    let mut fpath = cache_dir();
    fpath.push(Path::new(fname));
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&fpath.as_path())
        .unwrap();
    writeln!(file, "{}", contents);
}

fn create(fname: &str, contents: &str) {
    let mut fpath = cache_dir();
    fpath.push(Path::new(fname));
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&fpath.as_path())
        .unwrap();
    writeln!(file, "{}", contents);
}

fn del(fname: &str) {
    let mut fpath = cache_dir();
    fpath.push(Path::new(fname));
    fs::remove_file(&fpath.as_path());
}

fn clear(fname: &str) {
    let mut fpath = cache_dir();
    fpath.push(Path::new(fname));
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&fpath.as_path())
        .unwrap();
    writeln!(file, "");
}

fn exists(fname: &str) -> bool {
    let mut fpath = cache_dir();
    fpath.push(Path::new(fname));
    Path::new(&fpath.as_path()).exists()
}

fn read(fname: &str) -> String {
    let mut fpath = cache_dir();
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

pub mod sess {
    pub fn is_signed_in(id: &str) -> bool {
        super::exists(&format!("sess/{}", id))
    }
    
    pub fn mk_sess(id: &str, start: u64) {
        super::create(&format!("sess/{}", id), &format!("{}", start));
    }
    
    pub fn get_sess_start(id: &str) -> u64 {
        super::read(&format!("sess/{}", id)).parse::<u64>().unwrap()
    }
    
    pub fn rm_and_get_sess(id: &str) -> u64 {
        let start = get_sess_start(&id);
        super::del(&format!("sess/{}", id));
        start
    }
}

pub mod user {
    use chrono::{offset, Datelike};
    use regex::Regex;

    // checks if actions (e.g. signin) can be performed upon a theoretical user with the given ID
    pub fn is_actionable(id: &str) -> bool {
        validate_id(id)
    }

    // checks if a user could be created unsuspiciously based on a requested ID number
    // this method is somewhat convoluted; it is commented as best I could, but I recommend using regexr.com and a whiteboard
    pub fn validate_id(id: &str) -> bool {
        let current_yy= offset::Local::today().year() % 100; // 19, if the current year is 2019
        let min_yy = current_yy - 1; // allows superseniors in second semester
        let max_yy = current_yy + 4; // allows freshman in first semester
        let grad_yr_regex = if min_yy / 10 == max_yy / 10 { // same decade
            format!("{}[{}-{}]", (min_yy / 10), (min_yy % 10), (max_yy % 10))
        } else { // different decades
            format!("(?:{}[{}-9])|(?:{}[0-{}])", (min_yy / 10), (min_yy % 10), (max_yy / 10), (max_yy % 10))
        };
        let mid_regex = "[0-9]{3}"; // TODO: figure out what's valid here (I've been told that it's usually 400)
        let end_regex = "[0-9]{3}"; // these numbers appear to be random
        let re = Regex::new(&format!("{}{}{}", &grad_yr_regex, &mid_regex, &end_regex)).unwrap();
        re.is_match(id)
    }
}