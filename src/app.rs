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
    // Vec of timecodes shown for current week
    pub timecodes: Vec<String>,
    // Timecodes that should be shown for every week, regardless of content
    pub starred_timecodes: Vec<String>,
    // Max 5 timecodes shown at a time; this indicates the range
    pub timecode_range: [usize; 2],
    // Currently highlighted
    pub active_timecode: usize,
    pub active_day: u8,
    pub active_week: u8,
    pub active_year: usize,
    pub state: Vec<State>,
    pub filepath: String,
    // String buffer used when adding new timecode
    pub timecode_buffer: String,
}
impl App {
    pub fn new(filepath: String) -> Result<App, Box<dyn Error>> {
        let current_date = chrono::Utc::now();
        let conf = Config::new();

        let timer_js = fs::read_to_string(&filepath);
        let mut data: TimekeeperData = match timer_js {
            Ok(t) => serde_json::from_str(&t)?,
            _ => TimekeeperData(HashMap::<usize, Year>::new()),
        };

        let active_week = current_date.iso_week().week() as u8;
        let active_year = current_date.year() as usize;
        let active_day = current_date.weekday().num_days_from_monday() as u8;
        let starred_timecodes = conf.starred_timecodes.clone();

        data.load_week(
            active_week,
            active_year,
            starred_timecodes
                .clone()
                .into_iter()
                .map(|tc| Timecode::from_string(tc))
                .collect(),
        );

        let timecodes = data.get_timecodes(active_year, active_week);
        let range_end = timecodes.len().min(5);

        Ok(App {
            data,
            conf,
            timecodes,
            starred_timecodes,
            timecode_range: [0, range_end],
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
            if self.active_timecode >= (self.timecode_range[1]) {
                self.timecode_range[0] += 1;
                self.timecode_range[1] += 1;
            }
        }
    }
    pub fn prev_timecode(&mut self) {
        if self.active_timecode > 0 {
            self.active_timecode -= 1;
            if self.active_timecode < (self.timecode_range[0]) {
                self.timecode_range[0] -= 1;
                self.timecode_range[1] -= 1;
            }
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
        self.data.load_week(
            self.active_week,
            self.active_year,
            self.starred_timecodes
                .clone()
                .into_iter()
                .map(|tc| Timecode::from_string(tc))
                .collect(),
        );
        self.assign_timecodes();
    }
    pub fn prev_week(&mut self) {
        if self.active_week > 1 {
            self.active_week -= 1;
        } else {
            self.active_week = 52;
            self.prev_year();
        }
        self.data.load_week(
            self.active_week,
            self.active_year,
            self.starred_timecodes
                .clone()
                .into_iter()
                .map(|tc| Timecode::from_string(tc))
                .collect(),
        );
        self.assign_timecodes();
    }
    pub fn assign_timecodes(&mut self) {
        self.timecodes = self.data.get_timecodes(self.active_year, self.active_week);
        let len = self.timecodes.len();
        if len < (self.active_timecode + 1) {
            self.active_timecode = (len as i32 - 1).max(0) as usize
        }
        self.timecode_range[1] = self.timecode_range[1].min(self.timecodes.len());
        self.timecode_range[0] = (self.timecode_range[1] as i32 - 5).max(0) as usize;
    }
    pub fn next_year(&mut self) {
        self.active_year += 1;
    }
    // TODO: Support for BC years kappa
    pub fn prev_year(&mut self) {
        self.active_year -= 1;
    }

    pub fn toggle_writing_comment(&mut self) {
        if self.timecodes.len() == 0 {
            return;
        }
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
            None => (),
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
            None => (),
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
            self.timecode_range[0] = self.timecodes.len() - 4;
            self.timecode_range[1] = self.timecodes.len();
        } else if self.get_state() == &State::AddingTimecode {
            self.add_timecode(self.timecode_buffer.clone());
            self.flush_timecode_buffer();
            self.state.pop();
            self.timecode_range[0] = self.timecodes.len() - 5;
            self.timecode_range[1] = self.timecodes.len();
            self.active_timecode = self.timecodes.len() - 1;
        }
    }
    pub fn cancel_adding_timecode(&mut self) {
        self.flush_timecode_buffer();
        self.state.pop();
        self.timecode_range[0] = self.timecodes.len() - 5;
        self.timecode_range[1] = self.timecodes.len();
        self.active_timecode = self.timecodes.len() - 1;
    }
    pub fn add_timecode(&mut self, timecode: String) {
        if !self.timecodes.contains(&timecode) {
            self.timecodes.push(timecode.clone());
            let tc = Timecode::from_string(timecode);
            self.data
                .add_timecode(self.active_week, self.active_year, tc);
        } else {
            // TODO: Show error message in info box
            // Cannot add timecode with existing name!
            ()
        }
    }

    pub fn star_timecode(&mut self) {
        if let Some(tc) = self.get_cur_timecode() {
            if !self.starred_timecodes.contains(&tc) {
                self.starred_timecodes.push(tc.clone());
                self.conf.add_timecode(tc);
            }
        }
    }

    pub fn unstar_timecode(&mut self) {
        if let Some(tc) = self.get_cur_timecode() {
            self.starred_timecodes.retain(|t| t != &tc);
            self.conf.remove_timecode(&tc);
        }
    }

    pub fn get_cur_timecode(&self) -> Option<String> {
        if self.timecodes.len() > self.active_timecode {
            Some(self.timecodes[self.active_timecode].clone())
        } else {
            None
        }
    }
}
