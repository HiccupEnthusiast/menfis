use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Month {
    total_days: u8,
    pub name: MonthNames,
    pub number: u8,
}

#[derive(Copy, Clone, Default, Debug)]
pub enum MonthNames {
    #[default]
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Default for Month {
    fn default() -> Self {
        Self {
            total_days: 31,
            name: MonthNames::default(),
            number: 1,
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}", self.number)
    }
}
