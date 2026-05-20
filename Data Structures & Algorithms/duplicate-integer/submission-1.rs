impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        if nums.len() < 2{
            return false;
        }
        let mut nums = nums;
        nums.sort();
        let mut iter = nums.iter();
        let mut tmp =  iter.next().unwrap();

        for n in iter{
            if n == tmp{
                return true;
            }
            tmp = n;
        }
        return false;
    }
}
