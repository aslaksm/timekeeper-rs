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

    // Gets timecodes of currently active week.
    pub fn get_timecodes(&self, year: usize, week: u8) -> Vec<String> {
        let week = self.get(year).unwrap().get(week).unwrap();
        week.0.iter().map(|t| t.timecode.clone()).collect()
    }

    pub fn add_timecode(&mut self, week: u8, year: usize, timecode: Timecode) {
        self.get_mut(year)
            .unwrap()
            .get_mut(week)
            .unwrap()
            .0
            .push(timecode);
    }

    // pub fn remove_timecode(&mut self, week: u8, year: usize, timecode: String) {
    //     self.get_mut(year)
    //         .unwrap()
    //         .get_mut(week)
    //         .unwrap()
    //         .0
    //         .retain(|tc| tc.timecode != timecode);
    // }

    // Adds starred timecodes to current week, or creates new week if no exists
    // TODO: Timecode with all days set to null should not load/be shown
    pub fn load_week(&mut self, week: u8, year: usize, starred_timecodes: Vec<Timecode>) {
        let year_data = self
            .0
            .entry(year)
            .or_insert(Year(HashMap::<u8, Week>::new()));
        match year_data.0.get_mut(&week) {
            Some(w) => {
                w.remove_empty();
                w.add_timecodes(starred_timecodes);
            }
            None => {
                let mut w = Week(vec![]);
                w.add_timecodes(starred_timecodes);
                year_data.0.insert(week, w);
            }
        };
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
    // TODO: Figure out how to do timecode ordering
    pub fn add_timecodes(&mut self, timecodes: Vec<Timecode>) {
        for new_tc in timecodes.into_iter() {
            if let None = self.0.iter().find(|tc| tc.timecode == new_tc.timecode) {
                // self.0.insert(0, new_tc);
                self.0.push(new_tc);
            }
        }
    }
    pub fn remove_empty(&mut self) {
        self.0.retain(|tc| !tc.is_empty());
    }
}

// TODO: Timecodes with no content (i.e. all days are None) should not serialize
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
    pub fn is_empty(&self) -> bool {
        if self.monday.is_some()
            || self.tuesday.is_some()
            || self.wednesday.is_some()
            || self.thursday.is_some()
            || self.friday.is_some()
            || self.saturday.is_some()
            || self.sunday.is_some()
        {
            false
        } else {
            true
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
