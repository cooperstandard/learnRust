pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn main() {
    let _ = is_valid("()".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn example_2() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn example_3() {
        assert!(!is_valid("(]".to_string()));
    }

    #[test]
    fn example_4() {
        assert!(is_valid("([])".to_string()));
    }

    #[test]
    fn example_5() {
        assert!(!is_valid("[".to_string()));
    }
}
