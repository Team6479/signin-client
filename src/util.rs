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

pub fn log(loc: &str, msg: &str) {
    cache::append("log", &format!("[{} @ {}] {}", loc, super::time::get_time(), msg));
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
        contents.trim().to_string()
    }
}

pub mod traits {
    use std::collections::HashMap;

    pub struct ApiPostRequest {
        pub endpt: String, // an API endpoint such as /get/user
        pub body: String, // this should not contain an API key
    }
    
    pub trait Pushable {
        fn get_post_req(&self, key: &str) -> ApiPostRequest;
    }
    
    pub trait Pullable {
        fn get_post_req(&self, key: &str) -> ApiPostRequest;
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
        fn get_post_req(&self, key: &str) -> ApiPostRequest {
            ApiPostRequest {
                endpt: String::from("/put/entry"),
                body: format!("id={}&start={}&end={}&key={}", self.id, self.start, self.end, key),
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
        cache::read(&format!("sess/active/{}", id)).parse::<u64>().unwrap()
    }
    
    pub fn rm_and_get_sess(id: &str) -> u64 {
        let start = get_sess_start(&id);
        cache::del(&format!("sess/active/{}", id));
        start
    }

    pub fn push_queue(queue: &mut Vec<Box<dyn Pushable>>) {
        for ln in cache::read("sess/queue").split("\n") {
            queue.push(Box::new(Session::deserialize(&ln)));
        }
        cache::clear("sess/queue"); // clear queue file
    }
}

pub mod user {
    use chrono::{offset, Datelike};
    use regex::Regex;
    use super::{cache, traits::*};

    pub struct User {
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
        fn get_post_req(&self, key: &str) -> ApiPostRequest {
            ApiPostRequest {
                endpt: String::from("/put/user"),
                body: format!("id={}&name={}&lvl={}&key={}", self.id, self.name, self.lvl, key), // lvl currenly unused by server
            }
        }
    }
    // TODO: impl Pullable for User

    pub enum Creatability {
        Unobstructed, // user can be created according to the normal processes without suspicion
        Privileged, // user can be created, but requires administrative approval due to suspicious parameters
        Impossible, // user cannot be created
    }
    // checks the creatability of a user (i.e. if the ) based on a requested User
    pub fn is_creatable(req: &User) -> Creatability {
        if validate_id(&req.id) {
            Creatability::Unobstructed
        } else if req.id.contains(",") || req.name.contains(",") {
            Creatability::Impossible
        } else {
            Creatability::Privileged
        }
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

    // this function may assume that the file exists
    fn get_user_from_file(id: &str, fname: &str) -> Option<User> {
        // TODO: this code is horribly inefficient and should be rewritten to use proper sorting
        for ln in cache::read(fname).split("\n") {
            let user = User::deserialize(&ln);
            if user.id == id {
                return Some(user);
            }
        }
        None
    }

    // for efficiency, there is no user_exists function
    pub fn get_user(id: &str) -> Option<User> {
        // there's probably a more efficient way to do this, so look into that sometime
        if cache::exists("user/server") { // single source of truth subject to change
            if let Some(user) = get_user_from_file(id, "user/server") {
                return Some(user);
            }
        }
        if cache::exists("user/local") {
            if let Some(user) = get_user_from_file(id, "user/local") {
                return Some(user);
            }
        }
        None
    }

    pub fn push_and_move_local(queue: &mut Vec<Box<dyn Pushable>>) {
        for ln in cache::read("user/local").split("\n") {
            let user = User::deserialize(&ln);
            cache::append("user/server", &user.serialize()); // move from local to server
            queue.push(Box::new(user));
        }
        cache::clear("user/local"); // all users have been moved to server
    }
}

pub mod remote {
    use super::{cache, traits::*};
    use reqwest;
    use argon2;
    use rand::{self, RngCore};

    fn call(req: ApiPostRequest) -> Option<reqwest::blocking::Response> {
        let api_root = "https://team6479-signin.herokuapp.com/api";
        let client = reqwest::blocking::Client::new();
        let res = client.post(&format!("{}{}", api_root, req.endpt)) // TODO: the actual server
            .body(req.body)
            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(reqwest::header::USER_AGENT, "6479-signin")
            .send();
        if let Ok(data) = res {
            Some(data)
        } else {
            None
        }
    }
    fn call_text(req: ApiPostRequest) -> Option<String> {
        if let Some(resp) = call(req) {
            Some(resp.text().unwrap())
        } else {
            None
        }
    }

    pub enum InternetStatus {
        Online,
        Offline,
        Portal,
    }

    pub fn get_status() -> InternetStatus {
        if let Some(data) = call_text(ApiPostRequest {
            endpt: String::from("/ping"),
            body: String::from("ping"),
        }) {
            if data == "ping" {
                InternetStatus::Online
            } else {
                InternetStatus::Portal
            }
        } else {
            InternetStatus::Offline
        }
    }

    pub fn auth(_usr: &str, passwd: &str, status: &InternetStatus) -> Result<bool, String> {
        match status {
            InternetStatus::Online => {
                if let Some(text) = call_text(ApiPostRequest {
                    endpt: String::from("/key/check"),
                    body: String::from(format!("key={}", passwd)),
                }) {
                    if text == "true" { // TODO: verify password with server
                        let mut salt = [0u8; 16]; // hopefully this doesn't break too much
                        rand::thread_rng().fill_bytes(&mut salt);
                        let hash = argon2::hash_encoded(passwd.as_bytes(), &salt, &argon2::Config::default()).unwrap();
                        cache::create("passwd", &hash);
                        return Ok(true);
                    }
                    Ok(false)
                } else {
                    Err(String::from("reqwest error"))
                }
            },
            _ => {
                if cache::exists("passwd") {
                    let hash = cache::read("passwd"); // I understand the security implications and will fix this later
                    Ok(argon2::verify_encoded(&hash, passwd.as_bytes()).unwrap())
                } else {
                    Err(String::from("No cached password, internet required"))
                }
            },
        }
    }

    pub fn push_many(queue: &Vec<Box<dyn Pushable>>, key: &str) {
        for item in queue {
            call(item.get_post_req(key));
        }
    }
}