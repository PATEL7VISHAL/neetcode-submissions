use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hsh = HashMap::<i32, usize>::new();
        nums.iter().for_each(|e| {
            let entry = hsh.entry(*e).or_default();
            *entry += 1;
        });

        let mut bucket: Vec<Vec<i32>> = vec![Vec::new(); nums.len()+1];
        hsh.iter().for_each(|(num, count)| {
            bucket[*count].push(*num);
        });
        let len = k as usize;
        let mut ans: Vec<i32> = Vec::with_capacity(len);
        for e in bucket.into_iter().rev() {
            if e.len() == 0 {
                continue;
            }
            for n in e.into_iter() {
                ans.push(n);
                if ans.len() == len {
                    return ans;
                }
            }
        }
        return ans;
    }
}
