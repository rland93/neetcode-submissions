impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        let mut strs = strs;
        strs.sort_by_key(|s| s.len());

        // length of strs
        let nstrs = strs.len();

        // lenth of common prefix
        let mut prefix_len = 0;

        // iterate by character idx up to length of shortest string
        for i in 0..strs[0].len() {
            let mut acc: u8 = 0;

            // for each string, check equality of ith char across each
            for n in 0..nstrs {
                let bn = strs[n].as_bytes()[i];
                acc |= strs[0].as_bytes()[i] ^ bn;
            }

            // acc will is nonzero if any of them were different
            if (acc == 0) {
                prefix_len += 1;
            }
            else {
                // break out; we've reached maximum common prefix.
                break;
            }
        }

        if (prefix_len == 0) {
            return String::from("");
        } else {
            return strs[0][..prefix_len].to_string();
        }
    }
}
