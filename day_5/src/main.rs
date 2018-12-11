use std::fs::File;
use std::io::Read;


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();

    file.read_to_string(&mut input);
    let s = exec_reactions_on_polymer(&mut input);
    println!("Initial units: {}, final units: {}", input.len(), s.len());

    Ok(())
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
