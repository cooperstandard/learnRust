use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for (index, num) in nums.into_iter().enumerate() {
        if let Some(complement) = target.checked_sub(num) {
            match seen.get(&complement) {
                Some(&partner_index) => {
                    return vec![
                        partner_index,
                        i32::try_from(index).expect("input index exceeds i32::MAX"),
                    ];
                }
                None => {}
            }
        }

        seen.insert(
            num,
            i32::try_from(index).expect("input index exceeds i32::MAX"),
        );
    }

    unreachable!("problem guarantees exactly one valid pair")
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
    #[should_panic(expected = "problem guarantees exactly one valid pair")]
    fn no_solution() {
        two_sum(vec![1, 2, 8], 6);
    }

    #[test]
    #[should_panic(expected = "problem guarantees exactly one valid pair")]
    fn empty_input() {
        two_sum(vec![], 6);
    }

    #[test]
    #[should_panic(expected = "problem guarantees exactly one valid pair")]
    fn single_element_cannot_match_itself() {
        two_sum(vec![3], 6);
    }

    #[test]
    fn overflowing_complement_does_not_hide_valid_pair() {
        assert_eq!(two_sum(vec![i32::MIN, 0, 1], 1), vec![1, 2]);
    }

    #[test]
    #[should_panic(expected = "problem guarantees exactly one valid pair")]
    fn overflowing_sum_is_not_a_match() {
        two_sum(vec![i32::MAX, 1], i32::MIN);
    }
}
