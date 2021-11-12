use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TimekeeperData(pub HashMap<usize, Year>);

impl TimekeeperData {
    pub fn get(&self, year: usize) -> Option<&Year> {
        Some(self.0.get(&year)?)
    }

    pub fn get_mut(&mut self, year: usize) -> Option<&mut Year> {
        Some(self.0.get_mut(&year)?)
    }

    pub fn get_timecodes(&self, conf_timecodes: Vec<String>, year: usize, week: u8) -> Vec<String> {
        let tcs = self.get(year).and_then(|y| y.get(week));
        let mut tcs = match tcs {
            Some(w) => w.0.iter().map(|t| t.timecode.clone()).collect(),
            None => vec![],
        };
        // XXX: This may cause different ordering depending on circumstances
        for code in conf_timecodes {
            if !tcs.contains(&code) {
                tcs.push(code);
            }
        }
        tcs
    }

    pub fn add_timecode(&mut self, week: u8, year: usize, timecode: Timecode) {
        let a = self
            .get_mut(year)
            .unwrap()
            .get_mut(week)
            .unwrap()
            .0
            .push(timecode);
    }

    pub fn create_week_if_not_exists(&mut self, week: u8, year: usize, timecodes: Vec<Timecode>) {
        let year_data = self
            .0
            .entry(year)
            .or_insert(Year(HashMap::<u8, Week>::new()));
        year_data.0.entry(week).or_insert(Week(timecodes));
    }
}

#[derive(Serialize, Deserialize)]
pub struct Year(pub HashMap<u8, Week>);
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
        if timecode < self.0.len() {
            Some(&self.0[timecode])
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, timecode: usize) -> Option<&mut Timecode> {
        if timecode < self.0.len() {
            Some(&mut self.0[timecode])
        } else {
            None
        }
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
    pub fn get_mut(&mut self, day: u8) -> Option<&mut Day> {
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
    pub fn get(&self, day: u8) -> Option<&Day> {
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
    pub fn set_day(&mut self, day_idx: u8, day: Day) {
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
    // impl From seems too implicit for this
    pub fn from_string(tc_string: String) -> Timecode {
        Timecode {
            timecode: tc_string,
            monday: None,
            tuesday: None,
            wednesday: None,
            thursday: None,
            friday: None,
            saturday: None,
            sunday: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Day {
    pub hours: f32,
    pub comment: String,
}
