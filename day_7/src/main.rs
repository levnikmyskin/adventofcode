mod parser;
mod decision_maker;

use crate::parser::{parse_steps_with_dependency, Step, Dependencies};
use crate::decision_maker::StepIterator;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let step_dep = parse_steps_with_dependency(&args[1]).unwrap();
    let it = StepIterator::new(step_dep);
    let mut s = String::with_capacity(it.len());
    for el in it {
        s.push(el);
    }
    println!("Sequence: {}", s);
}
