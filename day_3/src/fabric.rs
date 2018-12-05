use std::fmt;
use crate::input_utils::Claim;

pub(crate) struct Fabric {
    matrix: [[u8; 1000]; 1000],
    pub claims: Vec<Claim>
}

impl Fabric {
    pub(crate) fn new() -> Self {
        Fabric{
            matrix: [[0; 1000]; 1000],
            claims: Vec::new()
        }
    }

    // Insert the claim into the fabric and return the amount of overlapping data
    pub(crate) fn insert_claim(&mut self, claim: Claim, mut overlapping_count: u32) -> u32 {
        let mut i = claim.top_offset;
        let mut j = claim.left_offset;

        for _ in 0..claim.height {
            for _ in 0..claim.width {
                overlapping_count += self.insert_one(i as usize, j as usize);
                j += 1;
            }
            j = claim.left_offset;
            i += 1;
        }

        self.claims.push(claim);
        overlapping_count
    }

    pub(crate) fn get_non_overlapping_id(&self) -> u16 {
        self.claims.iter()
            .filter(|c| !self.is_overlapping(c))
            .nth(0)
            .unwrap()
            .id
    }

    // Increase by 1 the bucket at the specified positions.
    // If more claims overlap the same area (that is, if the bucket value is
    // greater than 1), we don't count it 
    fn insert_one(&mut self, row_number: usize, row_index: usize) -> u32{
        let mut count = 0;
        if self.matrix[row_number][row_index] == 1 {
            count += 1;
        }
        self.matrix[row_number][row_index] += 1;

        count
    }

    fn is_overlapping(&self, claim: &Claim) -> bool {
        let mut i: usize = claim.top_offset as usize;
        let mut j: usize = claim.left_offset as usize;

        for _ in 0..claim.height {
            for _ in 0..claim.width {
                if self.matrix[i][j] > 1 {
                    return true;
                }
                j += 1;
            }
            j = claim.left_offset as usize;
            i += 1;
        }
        return false;
    }
}
    
impl fmt::Display for Fabric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 1000 * 1000 = rows for columns; + 2000 because every row 
        // will have a leading [ and a ] at the end + 1000 for the \n.
        let mut s = String::with_capacity((1000*1000) + 2000 + 1000);
        for i in 0..1000 {
            s.push('[');
            for j in 0..1000 {
                s.push_str(&self.matrix[i][j].to_string());
            }
            s.push(']');
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}


