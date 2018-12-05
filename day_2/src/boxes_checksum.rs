use crate::INP;

pub fn get_boxes_checksum() -> u32 {
    let mut cvec: Vec<char>;
    let mut two_rep_counter = 0;
    let mut three_rep_counter = 0;

    for s in INP.iter() {
        let mut previous = '_';
        let mut counter = 0;
        let mut already_2 = false;
        let mut already_3 = false;
        let it = s.chars();
        cvec = it.collect();
        cvec.sort();
        let mut chiter = cvec.into_iter().peekable();
        println!("Analyzing str: {}", s);
        loop {
            if already_2 && already_3 {
                break;
            }
            match chiter.next() {
                Some(ch) => {
                    println!("Analizing char: {}, previous: {}", ch, previous);
                    if ch == previous {
                        counter += 1;
                        match counter {
                            1 => {
                                if chiter.peek() == Some(&previous) {
                                    println!("Next is same, we got at least 3");
                                    continue;
                                }

                                if !already_2 {
                                    println!("2 char rep found, curr: {}, prev: {}", ch, previous);
                                    two_rep_counter += 1;
                                    already_2 = true;
                                }
                                counter = 0;
                            }
                            2 => {
                                if chiter.peek() == Some(&ch) {
                                    println!("Next is same, repetition is long 4");
                                    // advance by 1 and discard, otherwise we may end up
                                    // counting it as a 2 rep
                                    chiter.next();
                                    counter = 0;
                                    continue;
                                }
                                if !already_3 {
                                    println!("3 char rep found, curr: {}, prev: {}", ch, previous);
                                    three_rep_counter += 1;
                                    already_3 = true;
                                }
                                counter = 0;
                            }
                            _ => continue,
                        }
                    }
                    previous = ch;
                }
                None => break,
            }
        }
    }
    two_rep_counter * three_rep_counter
}
