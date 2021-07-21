use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // nums.iter().enumerate().for_each(|(index, value)| {
        for (index , value) in nums.iter().enumerate() {
            let diff = target - value;
            if let Some(second) = nums[index + 1..].iter().position(|&x| x == diff) {
                return vec![index as i32, second as i32 + index as i32 + 1]
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = [2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums.to_vec(), target);
        assert_eq!(result, vec![0, 1])
    }

    #[test]
    fn test2() {
        let nums = [3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums.to_vec(), target);
        assert_eq!(result, vec![1, 2])
    }

    #[test]
    fn test3() {
        let nums = [3, 3];
        let target = 6;
        let result = Solution::two_sum(nums.to_vec(), target);
        assert_eq!(result, vec![0, 1])
    }
}
