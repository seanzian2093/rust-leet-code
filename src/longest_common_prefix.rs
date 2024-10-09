pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let shortest = strs.iter().min_by_key(|s| s.len()).unwrap();
    let res = (0..shortest.len())
        .take_while(|i| {
            let pat = &shortest[..i + 1];
            strs.iter().all(|s| s.starts_with(pat))
        })
        .max();

    match res {
        Some(r) => String::from(&shortest[0..r + 1]),
        None => String::from(""),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn test_longest_common_prefix_null() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        assert_eq!(longest_common_prefix(strs), "".to_string());
    }
}
