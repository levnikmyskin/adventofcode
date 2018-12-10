use crate::shift_logs::{Action, ShiftLogs};
use chrono::{NaiveDateTime, Timelike};
use std::collections::HashMap;

pub struct LazyGuards {
    pub per_minute_table: HashMap<u16, [u32; 60]>,
    pub total_asleep_table: HashMap<u16, u32>
}

impl LazyGuards {
    pub fn new(shifts: &Vec<ShiftLogs>) -> LazyGuards {
        let mut per_minute: HashMap<u16, [u32; 60]> = HashMap::new();
        let mut tot_asleep: HashMap<u16, u32> = HashMap::new();
        let mut sleep_start = NaiveDateTime::from_timestamp(0, 42_000_000); 
        for s in shifts.iter() {
            match s.action {
                Action::BeginShift => {},
                Action::WakeUp => Self::register_guard_slumber(&sleep_start, &s.date, s.guard, &mut tot_asleep, &mut per_minute),
                Action::Sleep => sleep_start = s.date
            };
        }

        LazyGuards{
            per_minute_table: per_minute,
            total_asleep_table: tot_asleep
        }
    }

    // Returns the guard that has slept the most.
    // For every guard (entry) in the map, it sum the values.
    // It then takes the maximum and returns it
    pub fn get_laziest_guard(&self) -> (&u16, &u32) {
        self.total_asleep_table.iter().max_by_key(|(_,v)| v.clone()).unwrap()
    }

    pub fn get_most_slept_minute(&self, guard: u16) -> usize {
        let row = self.per_minute_table[&guard];
        println!("Guard {} row:", guard);
        for (i, r) in row.iter().enumerate() {
            println!("minutes {}, value: {}", i, r);
        }
        let max = row.iter().max().unwrap();
        row.iter().position(|x| x == max).unwrap()
    }
    
    fn register_guard_slumber(sleep_start: &NaiveDateTime, sleep_end: &NaiveDateTime, guard: u16, tot_asleep: &mut HashMap<u16, u32>, per_minute: &mut HashMap<u16, [u32;60]>) {
        let s_start = *sleep_start;
        let s_end = *sleep_end;
        let diff = s_end - s_start;
        let minutes_asleep = tot_asleep.entry(guard).or_insert(0);
        *minutes_asleep += diff.num_minutes() as u32;

        let row = per_minute.entry(guard).or_insert([0; 60]);
        for i in sleep_start.minute()..sleep_end.minute() + 1 {
            row[i as usize] += 1;
        }
    }
}
