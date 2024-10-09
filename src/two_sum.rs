pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    for num_1 in 0..nums.len() {
        for num_2 in num_1 + 1..nums.len() {
            if nums[num_1] + nums[num_2] == target {
                res.push(num_1 as i32);
                res.push(num_2 as i32);
                break;
            }
        }
    }

    res
}

mod tests {
    use super::two_sum;

    #[test]
    fn test_two_sum_1() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let res = two_sum(nums, target);

        let expected = vec![1, 2];
        assert_eq!(res, expected);
    }
}
