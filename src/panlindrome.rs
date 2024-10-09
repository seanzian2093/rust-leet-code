use core::str;

pub fn is_panlindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x == 0 {
        return true;
    }

    if x % 10 == 0 {
        return false;
    }

    let x = x.to_string();
    let x_iter = x.chars();
    let x_rev = x_iter.clone().rev();
    let res = x_iter.eq(x_rev);

    res
}

mod tests {

    #[test]
    fn test_is_panlindrome() {
        let x = 121;
        let res = super::is_panlindrome(x);

        assert_eq!(res, true);
    }

    #[test]
    fn test_is_panlindrome_not() {
        let x = 122;
        let res = super::is_panlindrome(x);

        assert_eq!(res, false);
    }

    #[test]
    fn test_is_panlindrome_0() {
        let x = 0;
        let res = super::is_panlindrome(x);

        assert_eq!(res, true);
    }
}
