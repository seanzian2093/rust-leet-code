fn one_roman_to_int(s: &u8) -> i32 {
    match s {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => 0,
    }
}
pub fn roman_to_int(s: String) -> i32 {
    let mut s = String::from(s);
    s.push('O');
    let mut skip = false;
    let mut sum = 0;
    for pair in s.into_bytes().windows(2) {
        if skip {
            skip = false;
            continue;
        }
        match pair {
            [b'I', b'V'] => {
                skip = true;
                sum += 4
            }
            [b'I', b'X'] => {
                skip = true;
                sum += 9
            }
            [b'X', b'L'] => {
                skip = true;
                sum += 40
            }
            [b'X', b'C'] => {
                skip = true;
                sum += 90
            }
            [b'C', b'D'] => {
                skip = true;
                sum += 400
            }
            [b'C', b'M'] => {
                skip = true;
                sum += 900
            }
            [x, _] => {
                skip = false;
                sum += one_roman_to_int(x)
            }
            _ => sum += 0,
        };
    }
    sum
}

mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int_3() {
        let s = "III".to_string();
        let res = roman_to_int(s);

        assert_eq!(res, 3);
    }

    #[test]
    fn test_roman_to_int_58() {
        let s = "LVIII".to_string();
        let res = roman_to_int(s);

        assert_eq!(res, 58);
    }

    #[test]
    fn test_roman_to_int_1994() {
        let s = "MCMXCIV".to_string();
        let res = roman_to_int(s);

        assert_eq!(res, 1994);
    }
}
