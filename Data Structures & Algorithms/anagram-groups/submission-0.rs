use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<[u8; 26], Vec<String>>::new();
        for s in strs {
            let mut charcount: [u8; 26] = [0; 26];
            for c in s.as_bytes().iter() {
                charcount[(*c as u8 - 'a' as u8) as usize] += 1;
            }
            match map.get_mut(&charcount) {
                Some(cc_arr) => {
                    cc_arr.push(s);
                },
                None => {
                    map.insert(charcount, vec![s]);
                }
            }
        }
        map.into_iter().map(|(_, v)| {v}).collect()
    }
}
