use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for (index, num) in nums.into_iter().enumerate() {
        if let Some(complement) = target.checked_sub(num) {
            if let Some(&partner_index) = seen.get(&complement) {
                return vec![
                    partner_index,
                    i32::try_from(index).expect("input index exceeds i32::MAX"),
                ];
            }
        }

        seen.insert(
            num,
            i32::try_from(index).expect("input index exceeds i32::MAX"),
        );
    }

    vec![]
}

fn main() {
    let result = two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut result = two_sum(vec![2, 7, 11, 15], 9);
        result.sort_unstable();
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_2() {
        let mut result = two_sum(vec![3, 2, 4], 6);
        result.sort_unstable();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let mut result = two_sum(vec![3, 3], 6);
        result.sort_unstable();
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(two_sum(vec![-3, 4, 3, 90], 0), vec![0, 2]);
    }

    #[test]
    fn no_solution() {
        for nums in [vec![], vec![3], vec![1, 2, 8]] {
            assert!(two_sum(nums, 6).is_empty());
        }
    }

    #[test]
    fn overflowing_complement_does_not_hide_valid_pair() {
        assert_eq!(two_sum(vec![i32::MIN, 0, 1], 1), vec![1, 2]);
    }

    #[test]
    fn overflowing_sum_is_not_a_match() {
        assert!(two_sum(vec![i32::MAX, 1], i32::MIN).is_empty());
    }
}
