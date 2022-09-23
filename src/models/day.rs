use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Day {
    pub name: DayNames,
    pub number: u8
}

#[derive(Copy, Clone, Default, Debug)]
pub enum DayNames {
    #[default]
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

impl Day {
    pub fn new() -> Self {
        Day::default()
    }
}

impl Default for Day {
    fn default() -> Self {
        Day {
            name: DayNames::default(),
            number: 1,
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}", self.number)
    }
}

