impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = &strs[0][..]; // slice
        for s in &strs[1..] {
            while !s.starts_with(&prefix) {
                prefix = &prefix[..prefix.len() - 1];
            }
        }
        return prefix.to_string();
    }
}
