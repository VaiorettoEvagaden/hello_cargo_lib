use hello_cargo_lib::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_struct() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 0]);
    }
}
