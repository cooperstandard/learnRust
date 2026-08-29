pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length = 0;
    let mut last_index = [0usize; 256];
    let mut start = 0;

    for (end, &b) in s.as_bytes().iter().enumerate() {
        start = start.max(last_index[b as usize]);
        max_length = max_length.max(end - start + 1);
        last_index[b as usize] = end + 1;
    }

    max_length as i32
}

fn main() {
    length_of_longest_substring(String::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_abcabcbb() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
    }

    #[test]
    fn example_2_bbbbb() {
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
    }

    #[test]
    fn example_3_pwwkew() {
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    }

    #[test]
    fn empty_string() {
        assert_eq!(length_of_longest_substring(String::new()), 0);
    }

    #[test]
    fn dvdf_regression() {
        assert_eq!(length_of_longest_substring("dvdf".into()), 3);
    }
}
