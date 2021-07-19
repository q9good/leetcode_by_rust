use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut record = HashMap::with_capacity(nums.len());
        for (index, value) in nums.iter().enumerate() {
            record.insert(value, index as i32);
        }
        for (index, value) in nums.iter().enumerate() {
            match record.get(&(target - *value)) {
                Some(second) => {
                    if index as i32 != *second {
                        return vec![index as i32, *second as i32];
                    }
                }
                None => (),
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
