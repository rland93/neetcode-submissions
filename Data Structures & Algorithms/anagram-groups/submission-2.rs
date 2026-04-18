impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 1. if two different strings are anagrams, they are the same length
        //    and they have the same count of characters.
        // 2. therefore, we need to keep track of collections of strings sharing
        //    these properties. 
        // 3. if we go through the list of strings, calculate their character
        //    count, then we can build up a count->list in a hashmap
        // 4. if the counts are unique then each count will have a list of anagrams.
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        let l = strs.len();
        for s in strs {
            let mut cc: [u8; 26] = [0; 26];
            for c in s.as_bytes() {
                cc[(*c as u8 - 'a' as u8) as usize] += 1;
            }
            // now cc stores the character count of this string. Check the set and
            // either add or append.
            match map.get_mut(&cc) {
                Some(list) => {
                    list.push(s);
                },
                None => {
                    map.insert(cc, vec![s]);
                }
            }
        }

        let mut counter = 0;
        for container in map.values() {
            counter += container.len();
        }
        assert!(counter == l);

        // collect the hashset into a vec.
        return map.into_iter().map(|(_, v)| v).collect();
    }
}
