impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hsh = HashMap::<i32, usize>::new();
        nums.iter().for_each(|e| {
            let entry = hsh.entry(*e).or_default();
            *entry += 1;
        });
        let mut v = hsh.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut ans: Vec<i32> = Vec::with_capacity(k as usize);
        for (num, _) in &v[0..(k as usize)] {
            ans.push(*num);
        }
        return ans;
    }
}
