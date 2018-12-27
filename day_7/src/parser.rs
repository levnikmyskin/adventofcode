use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;

pub type Dependencies = HashSet<char>;
pub type Step = char;

pub fn parse_steps_with_dependency(
    steps_file_path: &str,
) -> Result<HashMap<Step, Dependencies>, Error> {
    let mut map: HashMap<Step, Dependencies> = HashMap::new();
    let file = File::open(steps_file_path)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z])").unwrap();

    for line in reader.lines() {
        let l = line.unwrap();
        let caps = re.captures(&l).unwrap();
        let step = caps[2].chars().next().unwrap();
        let dep = caps[1].chars().next().unwrap();
        
        // We insert the dependency as a step with no dependencies
        // otherwise we'll never catch them
        map.entry(dep).or_insert(HashSet::new());

        // We then insert the step with the step it depends on in the map
        let dependencies = map.entry(step).or_insert(HashSet::new());
        dependencies.insert(dep);
    }

    Ok(map)
}
