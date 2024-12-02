pub fn valid_parentheses(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '}' | ')' | ']' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }

    stack.is_empty()
}

mod tests {
    use crate::valid_parentheses::valid_parentheses;

    #[test]
    fn test_valid_parentheses_1() {
        let s = String::from("()");
        let res = valid_parentheses(s);

        assert!(res)
    }




}