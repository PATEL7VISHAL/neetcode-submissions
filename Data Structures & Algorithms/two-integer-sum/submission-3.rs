impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, n) in nums.iter().enumerate() {
            let s = target - *n;
            if let Some(i1) = map.get(&s) {
                return vec![*i1 as i32, index as i32];
            } else {
                map.insert(*n, index);
            }
        }
        return vec![];
    }
}
