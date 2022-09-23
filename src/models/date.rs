use crate::models::Month;
use crate::models::Day;
use crate::models::Hour;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Date {
    pub year: u16,
    pub month: Month,
    pub day: Day,
    pub hour: Hour,
    pub timezone: i8,
}

impl Date {
    pub fn new() -> Self {
        Date::default()
    }
    pub fn prettify(self) -> String {
       format!("{:?}, {:?} {}, {} {}({})", self.day.name, self.month.name, 
           self.month.number, self.year, self.hour, self.timezone) 
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: 1970,
            month: Month::default(),
            day: Day::default(),
            hour: Hour::default(),
            timezone: 0,
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>4}-{:0>2}-{:0>2} {:0>2} ({:0>2})", self.year, self.month, self.day, 
            self.hour, self.timezone) 
    } 
}
