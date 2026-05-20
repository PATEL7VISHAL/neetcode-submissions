impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        const INIT: u8 = b'a';
        // let mut hash_s: [usize; 26] = [0; 26];
        // let mut hash_t: [usize; 26] = [0; 26];
        let mut hash_s: [usize; 256] = [0; 256];
        let mut hash_t: [usize; 256] = [0; 256];

        let iter = s.as_bytes().iter().zip(t.as_bytes());
        for (a, b) in iter {
            // hash_s[(*a - INIT) as usize] += 1;
            // hash_t[(*b - INIT) as usize] += 1;

            hash_s[*a as usize] += 1;
            hash_t[*b as usize] += 1;
        }

        for (a, b) in hash_s.iter().zip(hash_t.iter()) {
            if a != b {
                return false;
            }
        }
        return true;
    }
}
