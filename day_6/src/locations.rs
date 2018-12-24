use std::collections::{HashMap, HashSet};
use std::fmt;

pub struct Locations {
    matrix: Vec<Vec<i8>>,
    labels: Vec<(usize, usize, i8)>,
}

impl Locations {
    pub fn new(rows: usize, columns: usize, n_labels: usize) -> Self {
        let mut v = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut v2 = Vec::with_capacity(columns);
            for j in 0..columns {
                v2.insert(j, -1);
            }
            v.insert(i, v2);
        }

        Locations {
            matrix: v,
            labels: Vec::with_capacity(n_labels),
        }
    }

    pub fn insert_new_coordinates(&mut self, x: usize, y: usize, label: i8) {
        let row = &mut self.matrix[y];
        row[x] = label;
        self.labels.push((x, y, label));
    }

    pub fn compute_manhattan_distance(&mut self) {
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                if !self.is_a_label(j, i) {
                    let near_label = self.get_closest_by_manhattan(j, i);
                    self.matrix[i][j] = near_label;
                }
            }
        }
    }

    pub fn find_biggest_finite_area(&mut self) -> u32 {
        let right_bound = self.matrix[0].len();
        let bottom_bound = self.matrix.len();
        let mut infinite_labels = HashSet::new();
        let mut labels_size = HashMap::new();

        for i in 0..self.matrix.len() {
            for j in 0..right_bound {
                let label = self.matrix[i][j];
                if (j == 0 || j == right_bound - 1) || (i == 0 || i ==  bottom_bound) {
                    infinite_labels.insert(label);
                    labels_size.remove(&label);
                } else if !infinite_labels.contains(&label) {
                    let count = labels_size.entry(label).or_insert(0);
                    *count += 1;
                }
            }
        }

        let (_, value) = labels_size.iter().max_by_key(|(_, v)| *v).unwrap();
        *value
    }

    // This is actually a bit of a "dirty" solution:
    // instead of first computing the distance < n we simply count 
    // the number of locations that have distance < n because they're 
    // probably going to be all close together (and indeed it works :D)
    pub fn less_than_n_distance_region(&self, n: i32) -> u32 { 
        let mut counter = 0;
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                counter += self.mark_if_within_n_from_all(n, i, j);
            }
        }

        counter
    }

    fn get_closest_by_manhattan(&self, x: usize, y: usize) -> i8 {
        let mut current_min = -1;
        let mut current_label = -1;
        for (xl, yl, label) in self.labels.iter() {
            let distance = self.get_manhattan_distance(x as i32, *xl as i32, y as i32, *yl as i32) as i8;
            if current_min == -1 {
                current_min = distance;
                current_label = *label;
            } else if distance < current_min {
                current_min = distance;
                current_label = *label;
            } else if distance == current_min {
                current_label = -1
            }
        }

        current_label
    }

    fn get_manhattan_distance(&self, x: i32, x2: i32, y: i32, y2: i32) -> i32 {
        (x - x2).abs() + (y - y2).abs() 
    }

    fn mark_if_within_n_from_all(&self, n: i32, x: usize, y: usize) -> u32 {
        let mut current_score = 0;
        for (xl, yl, _) in self.labels.iter() {
            current_score += self.get_manhattan_distance(x as i32, *xl as i32, y as i32, *yl as i32);
            if current_score >= n {
                return 0;
            }
        }
        return 1;
    }

    fn is_a_label(&self, x: usize, y: usize) -> bool {
        self.labels.iter().any(|(i, j, _)| x == *i && y == *j)
    }
}

impl fmt::Debug for Locations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.matrix.len() {
            s.push('[');
            for j in 0..self.matrix[i].len() {
                s.push_str(self.matrix[i][j].to_string().as_str());
                s.push_str(", ");
            }
            s.push(']');
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
