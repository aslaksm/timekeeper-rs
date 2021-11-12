use crate::config::Config;
use crate::data::{Day, Timecode, TimekeeperData, Week, Year};
use chrono::Datelike;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(PartialEq)]
pub enum State {
    Browsing,
    WritingComment,
    AddingTimecode,
}

pub struct App {
    pub data: TimekeeperData,
    pub conf: Config,
    pub timecodes: Vec<String>,
    pub active_timecode: usize,
    pub active_day: u8,
    pub active_week: u8,
    pub active_year: usize,
    pub state: Vec<State>,
    pub filepath: &'static str,
    pub timecode_buffer: String,
}
impl App {
    pub fn new(filepath: &'static str) -> Result<App, Box<dyn Error>> {
        let current_date = chrono::Utc::now();
        let conf = Config::new();

        let timer_js = fs::read_to_string(filepath);
        let mut data: TimekeeperData = match timer_js {
            Ok(t) => serde_json::from_str(&t)?,
            _ => TimekeeperData(HashMap::<usize, Year>::new()),
        };

        let active_week = current_date.iso_week().week() as u8;
        let active_year = current_date.year() as usize;
        let active_day = current_date.weekday().num_days_from_monday() as u8;
        let timecodes = data.get_timecodes(conf.timecodes.clone(), active_year, active_week);

        data.create_week_if_not_exists(
            active_week,
            active_year,
            timecodes
                .clone()
                .into_iter()
                .map(|tc| Timecode::from_string(tc))
                .collect(),
        );
        Ok(App {
            data,
            conf,
            timecodes,
            active_timecode: 0,
            active_day,
            active_week,
            active_year,
            state: vec![State::Browsing],
            filepath,
            timecode_buffer: String::from(""),
        })
    }
    pub fn get_active_week(&self) -> Option<&Week> {
        Some((self.data.get(self.active_year)?).get(self.active_week))?
    }
    pub fn get_active_timecode(&mut self) -> Option<&mut Timecode> {
        let tc = (((self.data.get_mut(self.active_year)?).get_mut(self.active_week))?)
            .get_mut(self.active_timecode)?;
        Some(tc)
    }
    pub fn get_active_day_mut(&mut self) -> Option<&mut Day> {
        let a = ((((self.data.get_mut(self.active_year)?).get_mut(self.active_week))?)
            .get_mut(self.active_timecode)?)
        .get_mut(self.active_day)?;
        Some(a)
    }
    pub fn get_active_day(&self) -> Option<&Day> {
        let a = ((((self.data.get(self.active_year)?).get(self.active_week))?)
            .get(self.active_timecode)?)
        .get(self.active_day)?;
        Some(a)
    }

    pub fn next_timecode(&mut self) {
        if self.timecodes.len() != 0 && self.active_timecode < self.timecodes.len() - 1 {
            self.active_timecode += 1;
        }
    }
    pub fn prev_timecode(&mut self) {
        if self.active_timecode > 0 {
            self.active_timecode -= 1;
        }
    }
    pub fn next_day(&mut self) {
        if self.active_day < 6 {
            self.active_day += 1;
        } else {
            self.active_day = 0;
            self.next_week();
        }
    }
    pub fn prev_day(&mut self) {
        if self.active_day > 0 {
            self.active_day -= 1;
        } else {
            self.active_day = 6;
            self.prev_week();
        }
    }
    pub fn next_week(&mut self) {
        if self.active_week < 52 {
            self.active_week += 1;
        } else {
            self.active_week = 1;
            self.next_year();
        }
        self.data.create_week_if_not_exists(
            self.active_week,
            self.active_year,
            self.timecodes
                .clone()
                .into_iter()
                .map(|tc| Timecode::from_string(tc))
                .collect(),
        );
    }
    pub fn prev_week(&mut self) {
        if self.active_week > 1 {
            self.active_week -= 1;
        } else {
            self.active_week = 52;
            self.prev_year();
        }
        self.data.create_week_if_not_exists(
            self.active_week,
            self.active_year,
            self.timecodes
                .clone()
                .into_iter()
                .map(|tc| Timecode::from_string(tc))
                .collect(),
        );
    }
    pub fn next_year(&mut self) {
        self.active_year += 1;
    }
    // TODO: Support for BC years kappa
    pub fn prev_year(&mut self) {
        self.active_year -= 1;
    }

    pub fn toggle_writing_comment(&mut self) {
        if self.get_state() == &State::Browsing {
            self.state.push(State::WritingComment);
            let day_idx = self.active_day;
            if let None = self.get_active_day_mut() {
                let day = Day {
                    comment: String::from(""),
                    hours: 0.0,
                };
                self.get_active_timecode().unwrap().set_day(day_idx, day)
            }
        } else if self.get_state() == &State::WritingComment {
            self.state.pop();
        }
    }

    pub fn get_state(&self) -> &State {
        self.state.last().unwrap()
    }
    pub fn change_hours(&mut self, change: f32) {
        let act = self.active_day;
        match self.get_active_timecode() {
            Some(t) => match t.get_mut(act) {
                Some(day) => {
                    if day.hours >= -change {
                        day.hours += change
                    }
                }
                None => {
                    let new_day = Day {
                        hours: change.max(0.0),
                        comment: String::from(""),
                    };
                    t.set_day(act, new_day)
                }
            },
            None => panic!("ERR: No active timecode!"),
        }
    }

    pub fn set_hours(&mut self, val: f32) {
        let act = self.active_day;
        match self.get_active_timecode() {
            Some(t) => match t.get_mut(act) {
                Some(day) => day.hours = val,
                None => {
                    let new_day = Day {
                        hours: val.max(0.0),
                        comment: String::from(""),
                    };
                    t.set_day(act, new_day)
                }
            },
            None => panic!("ERR: No active timecode!"),
        }
    }

    pub fn write(&self) {
        fs::write(
            &self.filepath,
            serde_json::to_string_pretty(&self.data).expect("ERR: Unable to convert data to JSON!"),
        )
        .expect("ERR: Unable to write to file!")
    }
    pub fn append_char_to_comment(&mut self, c: char) {
        self.get_active_day_mut().unwrap().comment.push(c);
    }
    pub fn delete_char_from_comment(&mut self) {
        self.get_active_day_mut().unwrap().comment.pop();
    }
    // XXX: Might be superfluous
    pub fn should_show_cursor(&self) -> bool {
        matches!(self.get_state(), State::WritingComment)
    }
    pub fn append_char_to_timecode_buffer(&mut self, c: char) {
        self.timecode_buffer.push(c);
    }
    pub fn delete_char_from_timecode_buffer(&mut self) {
        self.timecode_buffer.pop();
    }
    pub fn flush_timecode_buffer(&mut self) {
        self.timecode_buffer.clear();
    }
    pub fn toggle_adding_timecode(&mut self) {
        if self.get_state() == &State::Browsing {
            self.state.push(State::AddingTimecode);
        } else if self.get_state() == &State::AddingTimecode {
            self.add_timecode(self.timecode_buffer.clone());
            self.flush_timecode_buffer();
            self.state.pop();
        }
    }
    pub fn cancel_adding_timecode(&mut self) {
        self.flush_timecode_buffer();
        self.state.pop();
    }
    pub fn add_timecode(&mut self, timecode: String) {
        self.timecodes.push(timecode.clone());
        self.conf.add_timecode(timecode.clone());
        let tc = Timecode::from_string(timecode);
        self.data
            .add_timecode(self.active_week, self.active_year, tc);
    }
    pub fn remove_timecode(&mut self) {
        let timecode = self.timecodes[self.active_timecode].clone();
        self.conf.remove_timecode(&timecode);
        self.timecodes.retain(|tc| tc != &timecode);
        self.data
            .remove_timecode(self.active_week, self.active_year, timecode);
    }
}
