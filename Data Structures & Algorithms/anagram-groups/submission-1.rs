use std::collections::HashMap;


impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut counter: [u8; 128] = [0; 128];
        let mut map: HashMap<String, Vec<String>> = HashMap::with_capacity(strs.len());

        for str in strs {
            let t = str.as_bytes().iter().for_each(|byte| {
                counter[*byte as usize] += 1;
            });

            let hash = String::from_utf8_lossy(&counter[97..=122]).to_string();
            let existing = map.entry(hash).or_insert(Vec::new());
            existing.push(str);

            (&mut counter[97..=122]).fill_with(|| 0); // reinit for next string
        }
        
        return map.into_values().collect();
    }
}
