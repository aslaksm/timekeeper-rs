use serde::{Deserialize, Serialize};
use std::{
    env::{self, consts::OS},
    fs,
    path::Path,
};

const DEFAULT_UNIX_CONF_PATH: &'static str = "/.config/timekeeper/conf.json";
const DEFAULT_WINDOWS_CONF_PATH: &'static str = "/timekeeper/conf.json";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub starred_timecodes: Vec<String>,
    pub lang: Language,
    pub has_seen_info: bool,
}
impl Config {
    pub fn new() -> Self {
        let filepath_str = Config::get_filepath();
        let filepath = Path::new(&filepath_str);
        let prefix = filepath.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let config_js = fs::read_to_string(&filepath);
        match config_js {
            Ok(c) => serde_json::from_str(&c).expect("ERR: config file corrupted!"),
            _ => {
                // TODO: When this expands, impl default for config
                let new_conf = Config {
                    lang: Language::English,
                    starred_timecodes: vec![],
                    has_seen_info: false,
                };
                fs::write(
                    &filepath,
                    serde_json::to_string_pretty(&new_conf)
                        .expect("ERR: Unable to write config file!"),
                )
                .expect("ERR: Unable to write to file!");
                new_conf
            }
        }
    }
    pub fn get_filepath() -> String {
        match OS {
            "linux" | "macos" | "freebsd" => format!(
                "{}{}",
                env::var("HOME").expect("ERR: HOME variable not set!"),
                DEFAULT_UNIX_CONF_PATH
            ),
            "windows" => format!(
                "{}{}",
                // XXX: Test on Windows
                env::var("LOCALAPPDATA").expect("ERR: Couldn't retrieve LocalAppData variable!"),
                DEFAULT_WINDOWS_CONF_PATH
            ),
            _ => panic!("Whatever you're trying to run this on, it's not supported (Please PR)."),
        }
    }

    pub fn add_timecode(&mut self, timecode: String) {
        if !self.starred_timecodes.contains(&timecode) {
            self.starred_timecodes.push(timecode);
            self.write();
        }
    }
    pub fn remove_timecode(&mut self, timecode: &String) {
        self.starred_timecodes.retain(|tc| tc != timecode);
        self.write();
    }
    pub fn write(&self) {
        fs::write(
            Config::get_filepath(),
            serde_json::to_string_pretty(&self).expect("ERR: Unable to convert data to JSON!"),
        )
        .expect("ERR: Unable to write to file!")
    }
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    Norsk,
    English,
}
