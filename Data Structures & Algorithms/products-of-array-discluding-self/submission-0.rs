impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
            let mut lr: Vec<i32> = Vec::with_capacity(nums.len());
    let mut rl: Vec<i32> = Vec::with_capacity(nums.len());
    let mut p = 1;
    for num in nums.iter() {
        p = *num * p;
        lr.push(p)
    }

    p = 1;
    for num in nums.iter().rev() {
        p = *num * p;
        rl.push(p)
    }
    rl.reverse();

    let mut ans: Vec<i32> = Vec::with_capacity(nums.len());
    ans.push(rl[1]);
    let mut iter = nums.iter().enumerate();
    iter.next();
    let len = nums.len();
    for (index, num) in iter {
        let nexi = index + 1;
        let prei = index - 1;
        let mut p = 1;
        if nexi < len {
            p = p * rl[nexi];
        }
        p = p * lr[prei];
        ans.push(p)
    }

    ans

    }
}
