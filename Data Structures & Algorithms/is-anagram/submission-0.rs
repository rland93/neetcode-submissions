impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        let mut charcounts: [i32; 26] = [0; 26];
        for i in 0..s.len() {
            // extract the char 
            let sc: u8 = s.as_bytes()[i];
            let tc: u8 = t.as_bytes()[i];

            // use the char to index into array
            charcounts[(sc - ('a' as u8)) as usize] += 1;
            charcounts[(tc - ('a' as u8)) as usize] -= 1;
        }
        for c in charcounts {
            if c != 0 {
                return false;
            }
        }
        return true;
    }
}
