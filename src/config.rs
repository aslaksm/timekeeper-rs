// use std::env::{self, consts::OS};

// const DEFAULT_UNIX_CONF_PATH: &'static str = "/.config/timekeeper/conf.json";
// // const DEFAULT_WINDOWS_CONF_PATH: &'static str = "/.config/timekeeper/conf.json";

// struct Config {
//     timecodes: Vec<String>,
//     // lang: Languages
// }
// impl Config {
//     pub fn new() -> Self {
//         let path = match OS {
//             "linux" | "macos" | "freebsd" => format!(
//                 "{}{}",
//                 env::var("HOME").expect("ERR: HOME variable not set!"),
//                 DEFAULT_UNIX_CONF_PATH
//             ),
//             "windows" => format!(
//                 "{}{}",
//                 // XXX: Test on Windows
//                 env::var("LOCALAPPDATA").expect("ERR: Couldn't retrieve LocalAppData variable!"),
//                 DEFAULT_UNIX_CONF_PATH
//             ),
//             _ => panic!("Whatever you're trying to run this on, it's not supported (Please PR)."),
//         };
//         Config {}
//     }
// }

// enum Languages {
//     Norwegian,
//     English,
// }
