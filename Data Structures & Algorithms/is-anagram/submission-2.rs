impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s = s;
        let mut t = t;
        s.make_ascii_lowercase();
        t.make_ascii_lowercase();
        
        let mut charcounts: [i32; 26] = [0; 26];
        for (sc, tc) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
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
