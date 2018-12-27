use std::collections::HashMap;
use crate::{Step, Dependencies};


pub struct StepIterator {
    steps_map: HashMap<Step, Dependencies>
}

impl StepIterator {
    pub fn new(steps_map: HashMap<Step, Dependencies>) -> StepIterator {
        StepIterator {
            steps_map
        }
    }

    pub fn len(&self) -> usize {
        self.steps_map.len()
    }

    fn get_next_step(&mut self) -> Step {
        let mut next_steps: Vec<Step> = self.steps_map.iter()
            .filter(|(_, v)| v.len() == 0)
            .map(|(k, _)| *k)
            .collect();
        next_steps.sort();
        let nstep = next_steps[0];

        // remove the step from the steps_map
        self.steps_map.clone()
            .iter()
            .filter(|(k, v)| **k == nstep || v.contains(&nstep))
            .for_each(|(k, _)| self.remove_steps_or_deps(nstep, k));
        
        
        nstep
    }

    fn remove_steps_or_deps(&mut self, to_remove: Step, s: &Step) {
        if to_remove == *s {
            self.steps_map.remove(&to_remove);
        } else {
            self.steps_map.entry(*s).and_modify(|e| { e.remove(&to_remove); });
        }
    }
}

impl Iterator for StepIterator {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_map.len() == 0 {
            return None;
        }
        Some(self.get_next_step())
    }
}
