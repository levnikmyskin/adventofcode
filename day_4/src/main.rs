mod shift_logs;
mod shift_table;
use crate::shift_logs::ShiftLogs;
use crate::shift_logs::Action;
use crate::shift_table::LazyGuards;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut logs = Vec::new();

    for line in reader.lines() {
        let s = ShiftLogs::new(&line.unwrap());
        logs.push(s);
    }
    logs.sort();
    assign_guard_to_shifts(&mut logs);
    let l = LazyGuards::new(&logs);

    let (guard, min) = l.get_most_inveterate_guard();
    println!("Most inveterate guard is {}, who slept most on minute {}. Answer is {}", guard, min, guard as usize * min);
    Ok(())
}

fn assign_guard_to_shifts(logs: &mut Vec<ShiftLogs>) {
    let mut previous_guard = 0;
    for shift in logs.iter_mut() {
        let guard = match shift.action {
            Action::BeginShift => { previous_guard = shift.guard; shift.guard },
            _ => previous_guard
        };
        shift.guard = guard;
    }
}
