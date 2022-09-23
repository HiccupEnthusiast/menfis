use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Hour {
    pub hour: u8,
    pub minute: u8,
}

impl Hour {
    pub fn new() -> Self {
        Hour::default()
    }
}

impl Default for Hour {
    fn default() -> Self {
        Self {
            hour: 00,
            minute: 00,
        }
    }
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}h{:0>2}m", self.hour, self.minute)
    }
}
