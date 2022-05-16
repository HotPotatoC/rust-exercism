use std::fmt;

const MINUTES_PER_DAY: i32 = 1440;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(hours * MINUTES_PER_HOUR + minutes).normalize()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock(self.0 + minutes).normalize()
    }

    fn normalize(&self) -> Self {
        Clock(self.0.rem_euclid(MINUTES_PER_DAY))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:02}:{:02}", self.0 / MINUTES_PER_HOUR, self.0 % 60)
    }
}
