use chrono::NaiveDateTime;
use std::fmt;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd)]
pub enum Action {
    WakeUp,
    Sleep,
    BeginShift
}

#[derive(Eq, PartialOrd)]
pub struct ShiftLogs {
    pub date: NaiveDateTime,
    pub action: Action,
    pub guard: u16
}

impl ShiftLogs {
    pub fn new(log: &str) -> ShiftLogs {
        let (date, action, guard_str) = ShiftLogs::extract_data_from_str(log);
        let guard = match guard_str.is_empty() {
            true => 0,
            false => guard_str.parse().unwrap()
        };

        ShiftLogs {
            date: NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M").unwrap(),
            action: action,
            guard: guard
        }
    }

    fn extract_data_from_str(log: &str) -> (String, Action, String) {
        let mut guard = String::new();

        // Skip the first square bracket and collect all bytes 'til the closing one
        let mut bytes = log.as_bytes().into_iter().skip(1);

        let date_iter = bytes.by_ref().take_while(|b| **b != b']');
        let date = date_iter.map(|&b| b as char).collect();

        // Skip the space and check what info we got 
        let mut bytes = bytes.skip(1);
        let log_info = match bytes.next() {
            Some(b'f') => Action::Sleep,
            Some(b'w') => Action::WakeUp,
            _ => {
                guard = bytes.skip_while(|&b| *b != b'#')
                    .skip(1)  // skip the '#'
                    .take_while(|&b| *b != b' ')
                    .map(|&b| b as char)
                    .collect();
                Action::BeginShift
            }
        };

        (date, log_info, guard)
    }
}

impl Ord for ShiftLogs {
    fn cmp(&self, other: &ShiftLogs) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialEq for ShiftLogs {
    fn eq(&self, other: &ShiftLogs) -> bool {
        self.date == other.date && self.action == other.action && self.guard == other.guard
    }
}

impl fmt::Display for ShiftLogs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match self.action {
            Action::Sleep => "Sleeps",
            Action::WakeUp => "Wakes up",
            Action::BeginShift => "Begins shift"
        };
        write!(f, "ShiftLogs:\n\taction: {}\n\tdate: {}\n\tguard: {}", 
               action, self.date, self.guard)
    }
}
