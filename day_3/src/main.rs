mod fabric; 
mod input_utils;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crate::fabric::Fabric;
use crate::input_utils::Claim;

fn main() -> std::io::Result<()> {
    let mut overlapping_count = 0;
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let mut fabric = Fabric::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let c = Claim::new_from_str(&l);
        overlapping_count = fabric.insert_claim(c, overlapping_count);
    }

    let nonoverlap_id = fabric.get_non_overlapping_id();
    println!("{}", nonoverlap_id);
    Ok(())
}
