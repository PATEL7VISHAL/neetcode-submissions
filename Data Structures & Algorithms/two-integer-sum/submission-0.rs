impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let mut j = 0usize;
        let mut i = 0usize;
        
        while i<len {
            j = i+1;
            while j < len{
                if nums[i] + nums[j] == target{
                    return vec![i as i32,j as i32];
                }
                j+=1;
            }
            i+=1;
        }
        return vec![];
    }
}
