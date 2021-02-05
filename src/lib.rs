use std::collections::HashMap;

fn get_numerals(level: &i32) -> HashMap<i32, String> {
    let mut unit_numerals: HashMap<i32, String> = HashMap::new();
    unit_numerals.insert(1, "I".to_string());
    unit_numerals.insert(4, "IV".to_string());
    unit_numerals.insert(5, "V".to_string());
    unit_numerals.insert(9, "IX".to_string());

    let mut ten_numerals: HashMap<i32, String> = HashMap::new();
    ten_numerals.insert(1, "X".to_string());
    ten_numerals.insert(4, "XL".to_string());
    ten_numerals.insert(5, "L".to_string());
    ten_numerals.insert(9, "XC".to_string());

    let mut hundred_numerals: HashMap<i32, String> = HashMap::new();
    hundred_numerals.insert(1, "C".to_string());
    hundred_numerals.insert(4, "CD".to_string());
    hundred_numerals.insert(5, "D".to_string());
    hundred_numerals.insert(9, "CM".to_string());

    let mut thousand_numerals: HashMap<i32, String> = HashMap::new();
    thousand_numerals.insert(1, "M".to_string());
    thousand_numerals.insert(4, "MV̅".to_string());
    thousand_numerals.insert(5, "V̅".to_string());
    thousand_numerals.insert(10, "X̅".to_string());

    match level {
        1 => unit_numerals,
        10 => ten_numerals,
        100 => hundred_numerals,
        1000 => thousand_numerals,
        _ => panic!("Invalid level"),
    }
}

fn roman_numerals(int: i32, level: i32) -> String {
    if int == 0 {
        return "".to_string();
    }

    let numerals = get_numerals(&level);
    let numeral_one = numerals.get(&1).expect("Couldn't get 1");

    let mut units = int % 10;

    let tens = int - units;
    let tens = tens / 10;

    let mut result = "".to_string();

    let modulo = units % 10;

    match modulo {
        4 | 5 | 9  => {
            result.push_str(numerals.get(&modulo).expect("Couldn't get numeral"));
            units -= modulo;
            result.push_str(&roman_numerals(units, level).to_string());
        },
        1 | 2 | 3 | 6 | 7 | 8 => {
            result.push_str(numeral_one);
            units -= 1;
            result = roman_numerals(units, level).to_string() + &result;
        },
        _ => {},
    }

    roman_numerals(tens, level * 10) + &result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_renders_blank() {
        assert_eq!(roman_numerals(0, 1), "");
    }

    #[test]
    fn it_renders_one() {
        assert_eq!(roman_numerals(1, 1), "I");
    }

    #[test]
    fn it_renders_two() {
        assert_eq!(roman_numerals(2, 1), "II");
    }

    #[test]
    fn it_renders_three() {
        assert_eq!(roman_numerals(3, 1), "III");
    }

    #[test]
    fn it_renders_four() {
        assert_eq!(roman_numerals(4, 1), "IV");
    }

    #[test]
    fn it_renders_five() {
        assert_eq!(roman_numerals(5, 1), "V");
    }

    #[test]
    fn it_renders_six() {
        assert_eq!(roman_numerals(6, 1), "VI");
    }

    #[test]
    fn it_renders_seven() {
        assert_eq!(roman_numerals(7, 1), "VII");
    }

    #[test]
    fn it_renders_nine() {
        assert_eq!(roman_numerals(9, 1), "IX");
    }

    #[test]
    fn it_renders_eleven() {
        assert_eq!(roman_numerals(11, 1), "XI");
    }

    #[test]
    fn it_renders_fourteen() {
        assert_eq!(roman_numerals(14, 1), "XIV");
    }

    #[test]
    fn it_renders_fifteen() {
        assert_eq!(roman_numerals(15, 1), "XV");
    }

    #[test]
    fn it_renders_sixteen() {
        assert_eq!(roman_numerals(16, 1), "XVI");
    }

    #[test]
    fn it_renders_nineteen() {
        assert_eq!(roman_numerals(19, 1), "XIX");
    }

    #[test]
    fn it_renders_twenty() {
        assert_eq!(roman_numerals(20, 1), "XX");
    }

    #[test]
    fn it_renders_twenty_one() {
        assert_eq!(roman_numerals(21, 1), "XXI");
    }

    #[test]
    fn it_renders_thirty() {
        assert_eq!(roman_numerals(30, 1), "XXX");
    }

    #[test]
    fn it_renders_forty() {
        assert_eq!(roman_numerals(40, 1), "XL");
    }

    #[test]
    fn it_renders_forty_nine() {
        assert_eq!(roman_numerals(49, 1), "XLIX");
    }

    #[test]
    fn it_renders_fifty() {
        assert_eq!(roman_numerals(50, 1), "L");
    }

    #[test]
    fn it_renders_fifty_seven() {
        assert_eq!(roman_numerals(57, 1), "LVII");
    }

    #[test]
    fn it_renders_sixty() {
        assert_eq!(roman_numerals(60, 1), "LX");
    }

    #[test]
    fn it_renders_ninety_nine() {
        assert_eq!(roman_numerals(99, 1), "XCIX");
    }

    #[test]
    fn it_renders_one_hundred() {
        assert_eq!(roman_numerals(100, 1), "C");
    }

    #[test]
    fn it_renders_four_hundred_and_forty_seven() {
        assert_eq!(roman_numerals(447, 1), "CDXLVII");
    }

    #[test]
    fn it_renders_nine_hundred_and_ninety_nine() {
        assert_eq!(roman_numerals(999, 1), "CMXCIX");
    }

    #[test]
    fn it_renders_one_thousand() {
        assert_eq!(roman_numerals(1000, 1), "M");
    }
}
