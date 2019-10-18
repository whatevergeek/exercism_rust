use std::fmt;
use std::cmp::PartialEq;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        (Clock { hours, minutes })
            .handle_neg_minutes()
            .handle_neg_hours()
            .normalize()
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        (Clock {           
            minutes: self.minutes + minutes,
            ..self
        })
        .handle_neg_minutes()
        .handle_neg_hours()
        .normalize()
    }

    fn handle_neg_minutes(self) -> Self {
        let mut new_minutes = self.minutes;
        let mut sub_hours = 0;

        while new_minutes < 0 {
            new_minutes += 60;
            sub_hours+=1;
        }

        Clock {
            hours: self.hours - sub_hours,
            minutes: new_minutes,
        }
    }

    fn handle_neg_hours(self) -> Self {
        let mut new_hours = self.hours;
        while new_hours < 0 {
            new_hours += 24;
        }

        Clock {
            hours: new_hours,
            minutes: self.minutes,
        }
    }

    fn normalize(self) -> Self {
        Clock {
            hours: (self.hours + (self.minutes / 60)) % 24,
            minutes: self.minutes % 60,
        }
    }
}
