
#[derive(Debug)]
pub(crate) struct Claim {
    pub id: u16,
    pub left_offset: u16,
    pub top_offset: u16,
    pub width: u16,
    pub height: u16
}

impl Claim {
    pub(crate) fn new_from_str(s: &str) -> Self {
        let mut split = s.split_whitespace();

        let id = extract_id(split.next().unwrap());
        split.next();  // skip @ char
        let (left_offset, top_offset) = extract_offsets(split.next().unwrap());
        let (width, height) = extract_sizes(split.next().unwrap());

        Claim{
            id,
            left_offset,
            top_offset,
            width,
            height
        }
    }
}

fn extract_id(s: &str) -> u16 {
    let id = &s[1..];
    id.parse::<u16>().unwrap()
}

fn extract_offsets(s: &str) -> (u16, u16) {
    let split: Vec<&str> = s.split(',').collect();
    let top_off = split[1];
    // Delete the last character
    let top_off = &top_off[..top_off.len() - 1];
    (split[0].parse::<u16>().unwrap(), top_off.parse::<u16>().unwrap())
}

fn extract_sizes(s: &str) -> (u16, u16) {
    let split: Vec<&str> = s.split('x').collect();
    (split[0].parse::<u16>().unwrap(), split[1].parse::<u16>().unwrap())
}
