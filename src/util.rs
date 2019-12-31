use std::fs;
use std::path::Path;

pub fn init() { // all code in this file may rely on the fact that this function has been executed
    let mut sess_path = cache::cache_dir();
    sess_path.push(Path::new("sess/active"));
    fs::create_dir_all(sess_path);

    let mut user_path = cache::cache_dir();
    user_path.push(Path::new("user"));
    fs::create_dir_all(user_path);
}

pub mod time {
    use chrono::{offset, NaiveTime};
    use std::convert::TryInto;

    pub fn get_time() -> u64 {
        offset::Local::now().timestamp().try_into().unwrap()
    }

    pub fn format_time(secs: u64) -> String {
        // note that this code will break if a user is signed in for multiple decades
        // note that this does not accept a timestamp, but rather a duration
        // FIXME: this breaks critically for sessions >= 24hrs
        // a custom algorithm is better suited to this
        NaiveTime::from_num_seconds_from_midnight(secs.try_into().unwrap(), 0).format("%H:%M:%S").to_string() // 0ns
    }
}

mod cache {
    use directories::ProjectDirs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::{Path, PathBuf};
    use std::fs;

    pub fn cache_dir() -> PathBuf { // all methods here are relative to the cache directory
        ProjectDirs::from("org", "team6479",  "signin").unwrap().cache_dir().to_path_buf()
    }
    
    pub fn append(fname: &str, contents: &str) {
        let mut fpath = cache_dir();
        fpath.push(Path::new(fname));
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&fpath.as_path())
            .unwrap();
        writeln!(file, "{}", contents);
    }
    
    pub fn create(fname: &str, contents: &str) {
        let mut fpath = cache_dir();
        fpath.push(Path::new(fname));
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&fpath.as_path())
            .unwrap();
        writeln!(file, "{}", contents);
    }
    
    pub fn del(fname: &str) {
        let mut fpath = cache_dir();
        fpath.push(Path::new(fname));
        fs::remove_file(&fpath.as_path());
    }
    
    pub fn clear(fname: &str) {
        let mut fpath = cache_dir();
        fpath.push(Path::new(fname));
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&fpath.as_path())
            .unwrap();
        writeln!(file, "");
    }
    
    pub fn exists(fname: &str) -> bool {
        let mut fpath = cache_dir();
        fpath.push(Path::new(fname));
        Path::new(&fpath.as_path()).exists()
    }
    
    pub fn read(fname: &str) -> String {
        let mut fpath = cache_dir();
        fpath.push(Path::new(fname));
        let file = OpenOptions::new()
            .read(true)
            .open(&fpath.as_path())
            .unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents);
        contents
    }
}

pub mod traits {
    use std::collections::HashMap;

    pub struct ApiPostRequest {
        pub endpt: String, // an API endpoint such as /get/user
        pub body: String, // this should not contain an API key
    }
    
    pub trait Pushable {
        fn get_post_req(&self) -> ApiPostRequest;
    }
    
    pub trait Pullable {
        fn get_post_req(&self) -> ApiPostRequest;
        fn callback(resp: HashMap<String, String>);
    }
    
    pub trait Cacheable {
        fn serialize(&self) -> String;
        fn deserialize(serialized: &str) -> Self;
        fn cache(&self);
    }
}

pub mod sess {
    use super::{cache, traits::*};

    pub struct Session {
        pub id: String,
        pub start: u64,
        pub end: u64,
    }
    impl Cacheable for Session {
        fn serialize(&self) -> String {
            format!("{},{},{}", self.id, self.start, self.end)
        }
        fn deserialize(serialized: &str) -> Session { // this MUST receive well-structured data
            let data: Vec<&str> = serialized.split(",").collect();
            Session {
                id: data[0].to_owned(),
                start: data[1].parse().unwrap(),
                end: data[2].parse().unwrap(),
            }
        }
        fn cache(&self) {
            cache::append("sess/queue", &self.serialize());
        }
    }
    impl Pushable for Session {
        fn get_post_req(&self) -> ApiPostRequest {
            ApiPostRequest {
                endpt: String::from("/put/entry"),
                body: format!("id={}&start={}&end={}", self.id, self.start, self.end),
            }
        }
    }

    pub fn is_signed_in(id: &str) -> bool {
        cache::exists(&format!("sess/active/{}", id))
    }
    
    pub fn mk_sess(id: &str, start: u64) {
        cache::create(&format!("sess/active/{}", id), &format!("{}", start));
    }
    
    pub fn get_sess_start(id: &str) -> u64 {
        cache::read(&format!("sess/active/{}", id)).trim().parse::<u64>().unwrap()
    }
    
    pub fn rm_and_get_sess(id: &str) -> u64 {
        let start = get_sess_start(&id);
        cache::del(&format!("sess/active/{}", id));
        start
    }
}

pub mod user {
    use chrono::{offset, Datelike};
    use regex::Regex;
    use super::{cache, traits::*};

    struct User {
        pub id: String,
        pub name: String,
        pub lvl: u8, // privilege level, currently unused
    }
    impl Cacheable for User {
        fn serialize(&self) -> String {
            format!("{},{},{}", self.id, self.name, self.lvl)
        }
        fn deserialize(serialized: &str) -> User { // this MUST receive well-structured data
            let data: Vec<&str> = serialized.split(",").collect();
            User {
                id: data[0].to_owned(),
                name: data[1].to_owned(),
                lvl: data[2].parse().unwrap(),
            }
        }
        fn cache(&self) {
            cache::append("user/local", &self.serialize());
        }
    }
    impl Pushable for User {
        fn get_post_req(&self) -> ApiPostRequest {
            ApiPostRequest {
                endpt: String::from("/put/user"),
                body: format!("id={}&name={}&lvl={}", self.id, self.name, self.lvl),
            }
        }
    }
    // TODO: impl Pullable for User

    // checks if actions (e.g. signin) can be performed upon a theoretical user with the given ID
    pub fn is_actionable(id: &str) -> bool {
        validate_id(id)
    }

    pub enum Creatability {
        Unobstructed, // user can be created according to the normal processes without suspicion
        Privileged, // user can be created, but requires administrative approval due to suspicious parameters
        Impossible, // user cannot be created
    }
    // checks the creatability of a user (i.e. if the ) based on a requested ID number
    pub fn is_creatable(id: &str) -> Creatability {
        if validate_id(id) {
            Creatability::Unobstructed
        } else {
            Creatability::Privileged
        }
        // TODO: check for bad chars like ','
    }

    // this method is somewhat convoluted; it is commented as best I could, but I recommend using regexr.com and a whiteboard
    fn validate_id(id: &str) -> bool {
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