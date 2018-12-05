use crate::INP;

pub fn get_similar_boxes_indexes() -> Option<(usize, usize, usize)> {
    let mut i = 0;
    while i < INP.len() {
        let current_str = INP[i];
        let mut j = i + 1; 

        while j < INP.len() {
           match check_strings_are_similar(current_str, INP[j]) {
               (true, index) => {
                   println!("Found the similar strings: {}, {}", current_str, INP[j]);
                   return Some((i, j, index));
               },
               _ => {}
           }
           j += 1;
        }
        i += 1;
    }
    return None;
}

fn check_strings_are_similar(s1: &str, s2: &str) -> (bool, usize) {
    let mut it1 = s1.char_indices();
    let mut it2 = s2.char_indices();
    let mut num_different_chars = 0;
    let mut different_index = 0;
    loop {
        let cur1 = it1.next();
        let cur2 = it2.next();
        if cur1 == None || cur2 == None {
            break;
        }
        let (i, c1) = cur1.unwrap();
        let (_, c2) = cur2.unwrap();
        if c1 != c2 {
            num_different_chars += 1;
            different_index = i;
        } 

        if num_different_chars > 1 {
            return (false, 0);
        }
    }

    (num_different_chars == 1, different_index)
}
