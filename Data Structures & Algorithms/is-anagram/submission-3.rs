impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut charct: [u8; 26] = [0;26];
        for (sc, tc) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
            charct[(*sc as u8 - 'a' as u8) as usize] += 1;
            charct[(*tc as u8 - 'a' as u8) as usize] -= 1;
        }
        for ch in charct {
            if ch != 0 {
                return false;
            }
        }
        return true;
    }
}
