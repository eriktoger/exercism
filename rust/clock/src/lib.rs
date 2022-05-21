#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_hours = hours;
        let mut new_minutes = minutes;
        if minutes < 0 {
            let hours_needed = (minutes.abs() + 59) / 60;
            new_minutes += hours_needed * 60;
            new_hours -= hours_needed;
        }
        while new_hours < 0 {
            new_hours += 24;
        }
        while new_minutes < 0 {
            new_minutes += 60;
        }

        Clock {
            hours: (240 + new_hours + new_minutes / 60) % 24,
            minutes: (60 + new_minutes) % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_minutes = self.minutes + minutes;
        let mut new_hours = self.hours;

        while new_minutes < 0 {
            new_hours -= 1;
            new_minutes += 60;
        }
        while new_hours < 0 {
            new_hours += 24;
        }

        new_hours += new_minutes / 60;
        Clock {
            hours: new_hours % 24,
            minutes: new_minutes % 60,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours % 24, self.minutes % 60)
    }
}
