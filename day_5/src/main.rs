use std::fs::File;
use std::io::Read;


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();

    file.read_to_string(&mut input);

    let mut scores: Vec<usize> = Vec::new();
    for c in b'a'..b'z' {
        let result = remove_character_and_react(&input, c as char);
        scores.push(result);
    }

    println!("Minimum is: {}", scores.iter().min().unwrap());
    Ok(())
}

fn remove_character_and_react(polymer: &str, lower_chr: char) -> usize {
    // We first remove the character and then its uppercase counterpart
    // is there a cleaner way to do this?
    let upper_chr = lower_chr.to_ascii_uppercase();
    let mut s = polymer.replace(lower_chr, "").replace(upper_chr, "");
    
    exec_reactions_on_polymer(&mut s).len()
}

fn exec_reactions_on_polymer(polymer: &mut str) -> String {
    let mut polym = String::with_capacity(polymer.len());
    let mut it = polymer.chars().peekable();

    while let Some(el) = it.next() {
        match it.peek() {
            Some(nxt) => {
                if el != *nxt && is_opposite(el, *nxt) {
                    // we need to consume the next element 
                    // because it reacted with our current one
                    // (we need to eliminate both)
                    it.next();
                } else {
                    polym.push(el);
                }
            }
            None => polym.push(el),
        }
    }

    if polymer.len() > polym.len() {
        return exec_reactions_on_polymer(&mut polym);
    }

    polym
}

fn is_opposite(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b || b.to_ascii_lowercase() == a
}
