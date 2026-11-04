use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    nums.into_iter().any(|num| !seen.insert(num))
}

fn main() {
    let result = contains_duplicate(vec![1, 2, 3, 1]);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_2() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_3() {
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }

    #[test]
    fn single_element() {
        assert!(!contains_duplicate(vec![42]));
    }

    #[test]
    fn negative_numbers() {
        assert!(contains_duplicate(vec![-1, 0, -1]));
    }

    #[test]
    fn duplicate_at_end() {
        assert!(contains_duplicate(vec![1, 2, 3, 4, 4]));
    }
}
