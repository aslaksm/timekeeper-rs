use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};

const DAY_INC: f32 = 0.5;

#[derive(PartialEq)]
pub enum State {
    Browsing,
    Selected,
}

pub struct App {
    pub data: TimekeeperData,
    pub timecodes: Vec<String>,
    pub active_timecode: usize,
    pub active_day: u8,
    pub active_week: u8,
    pub active_year: usize,
    pub state: State,
    pub filepath: &'static str,
}
impl App {
    pub fn new(filepath: &'static str) -> Result<App, Box<dyn Error>> {
        let timer_js = fs::read_to_string(filepath)?;
        let data: TimekeeperData = serde_json::from_str(&timer_js)?;
        let active_week = 1;
        let active_year = 2021;
        let timecodes = data.get_timecodes(active_year, active_week);
        Ok(App {
            filepath,
            data,
            timecodes,
            active_timecode: 0,
            active_day: 0,
            active_week,
            active_year,
            state: State::Browsing,
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
    // pub fn get_active_day_mut(&mut self) -> Option<&mut Day> {
    //     let a = ((((self.data.get_mut(self.active_year)?).get_mut(self.active_week))?)
    //         .get_mut(self.active_timecode)?)
    //     .get_mut(self.active_day)?;
    //     Some(a)
    // }
    pub fn get_active_day(&self) -> Option<&Day> {
        let a = ((((self.data.get(self.active_year)?).get(self.active_week))?)
            .get(self.active_timecode)?)
        .get(self.active_day)?;
        Some(a)
    }
    pub fn next_timecode(&mut self) {
        if self.active_timecode < self.timecodes.len() - 1 {
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
        }
    }
    pub fn prev_day(&mut self) {
        if self.active_day > 0 {
            self.active_day -= 1;
        }
    }
    pub fn toggle_select(&mut self) {
        if self.state == State::Browsing {
            self.state = State::Selected
        } else if self.state == State::Selected {
            self.state = State::Browsing
        }
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

    pub fn write(&self) {
        fs::write(
            self.filepath,
            serde_json::to_string_pretty(&self.data).expect("ERR: Unable to convert data to JSON!"),
        )
        .expect("ERR: Unable to write to file!")
    }
}

#[derive(Serialize, Deserialize)]
pub struct TimekeeperData(HashMap<usize, Year>);

impl TimekeeperData {
    pub fn get(&self, year: usize) -> Option<&Year> {
        Some(self.0.get(&year)?)
    }
    pub fn get_mut(&mut self, year: usize) -> Option<&mut Year> {
        Some(self.0.get_mut(&year)?)
    }

    fn get_timecodes(&self, year: usize, week: u8) -> Vec<String> {
        let tcs = self.get(year).and_then(|y| y.get(week));
        match tcs {
            Some(w) => w.0.iter().map(|t| t.timecode.clone()).collect(),
            None => vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Year(HashMap<u8, Week>);
impl Year {
    pub fn get(&self, week: u8) -> Option<&Week> {
        Some(self.0.get(&week)?)
    }
    pub fn get_mut(&mut self, week: u8) -> Option<&mut Week> {
        Some(self.0.get_mut(&week)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Week(pub Vec<Timecode>);
impl Week {
    pub fn get(&self, timecode: usize) -> Option<&Timecode> {
        Some(&self.0[timecode])
    }
    pub fn get_mut(&mut self, timecode: usize) -> Option<&mut Timecode> {
        Some(&mut self.0[timecode])
    }
}

#[derive(Serialize, Deserialize)]
pub struct Timecode {
    pub timecode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<Day>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<Day>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<Day>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<Day>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<Day>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<Day>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<Day>,
}

impl Timecode {
    fn get_mut(&mut self, day: u8) -> Option<&mut Day> {
        match day {
            0 => self.monday.as_mut(),
            1 => self.tuesday.as_mut(),
            2 => self.wednesday.as_mut(),
            3 => self.thursday.as_mut(),
            4 => self.friday.as_mut(),
            5 => self.saturday.as_mut(),
            6 => self.sunday.as_mut(),
            _ => None,
        }
    }
    fn get(&self, day: u8) -> Option<&Day> {
        match day {
            0 => self.monday.as_ref(),
            1 => self.tuesday.as_ref(),
            2 => self.wednesday.as_ref(),
            3 => self.thursday.as_ref(),
            4 => self.friday.as_ref(),
            5 => self.saturday.as_ref(),
            6 => self.sunday.as_ref(),
            _ => None,
        }
    }
    fn set_day(&mut self, day_idx: u8, day: Day) {
        let day = Some(day);
        match day_idx {
            0 => self.monday = day,
            1 => self.tuesday = day,
            2 => self.wednesday = day,
            3 => self.thursday = day,
            4 => self.friday = day,
            5 => self.saturday = day,
            6 => self.sunday = day,
            _ => panic!("ERR: Invalid date passed to set_day!"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub hours: f32,
    pub comment: String,
}
