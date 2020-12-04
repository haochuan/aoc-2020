use std::collections::HashMap;
extern crate regex;
use regex::Regex;

type Passport = HashMap<String, String>;
type PassportList = Vec<Passport>;

fn is_valid_passport(passport: &Passport, validation: bool) -> bool {
    if !passport.contains_key("byr")
        || !passport.contains_key("iyr")
        || !passport.contains_key("eyr")
        || !passport.contains_key("hgt")
        || !passport.contains_key("hcl")
        || !passport.contains_key("ecl")
        || !passport.contains_key("pid")
    {
        return false;
    }

    if validation {
        return is_valid_byr(passport.get("byr").unwrap().to_string())
            && is_valid_iyr(passport.get("iyr").unwrap().to_string())
            && is_valid_eyr(passport.get("eyr").unwrap().to_string())
            && is_valid_hgt(passport.get("hgt").unwrap().to_string())
            && is_valid_hcl(passport.get("hcl").unwrap().to_string())
            && is_valid_ecl(passport.get("ecl").unwrap().to_string())
            && is_valid_pid(passport.get("pid").unwrap().to_string());
    }
    true
}

fn is_valid_byr(input: String) -> bool {
    let input = input.parse::<u32>().unwrap();
    input >= 1920 && input <= 2002
}

fn is_valid_iyr(input: String) -> bool {
    let input = input.parse::<u32>().unwrap();
    input >= 2010 && input <= 2020
}

fn is_valid_eyr(input: String) -> bool {
    let input = input.parse::<u32>().unwrap();
    input >= 2020 && input <= 2030
}

fn is_valid_hgt(input: String) -> bool {
    let re = Regex::new(r"(\d+)(in|cm)").unwrap();
    if let Some(caps) = re.captures(&input) {
        let value = caps
            .get(1)
            .map_or("", |m| m.as_str())
            .parse::<u32>()
            .unwrap();
        let unit = caps.get(2).map_or("", |m| m.as_str());
        (unit == "cm" && value >= 150 && value <= 193)
            || (unit == "in" && value >= 59 && value <= 76)
    } else {
        false
    }
}

fn is_valid_hcl(input: String) -> bool {
    let re = Regex::new(r"#[0-9a-f]{6}").unwrap();
    re.is_match(&input)
}

fn is_valid_ecl(input: String) -> bool {
    let option_list = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    option_list.iter().any(|option| option == &input)
}

fn is_valid_pid(input: String) -> bool {
    let re = Regex::new(r"^\d{9}$").unwrap();
    re.is_match(&input)
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> PassportList {
    let mut passport_list: PassportList = vec![];
    for data in input.split("\n\n") {
        let mut passport: Passport = HashMap::new();
        let data = data.trim();
        for item in data.split(|c| c == ' ' || c == '\n') {
            let item = item.trim();
            let value_list: Vec<&str> = item.split(':').collect();
            let key = value_list[0];
            let value = value_list[1];
            passport.insert(key.to_string(), value.to_string());
        }
        passport_list.push(passport);
    }
    passport_list
}

#[aoc(day4, part1)]
pub fn part1(input: &PassportList) -> usize {
    let mut valid_count = 0;
    for passport in input {
        if is_valid_passport(passport, false) {
            valid_count = valid_count + 1;
        }
    }
    valid_count
}

#[aoc(day4, part2)]
pub fn part2(input: &PassportList) -> usize {
    let mut valid_count = 0;
    for passport in input {
        if is_valid_passport(passport, true) {
            valid_count = valid_count + 1;
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::{
        input_generator, is_valid_byr, is_valid_ecl, is_valid_hcl, is_valid_hgt, is_valid_pid,
        part1, part2,
    };

    #[test]
    fn bry() {
        assert!(is_valid_byr("2002".to_string()));
        assert!(!is_valid_byr("2003".to_string()));
    }

    #[test]
    fn hgt() {
        assert!(is_valid_hgt("60in".to_string()));
        assert!(is_valid_hgt("190cm".to_string()));
        assert!(!is_valid_hgt("190in".to_string()));
        assert!(!is_valid_hgt("190".to_string()));
    }

    #[test]
    fn hcl() {
        assert!(is_valid_hcl("#123abc".to_string()));
        assert!(!is_valid_hcl("#123abz".to_string()));
        assert!(!is_valid_hcl("123abc".to_string()));
    }

    #[test]
    fn ecl() {
        assert!(is_valid_ecl("brn".to_string()));
        assert!(!is_valid_ecl("wat".to_string()));
    }

    #[test]
    fn pid() {
        assert!(is_valid_pid("000000001".to_string()));
        assert!(!is_valid_pid("0123465789".to_string()));
    }

    #[test]
    fn part1_sample1() {
        let text = concat!(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n",
            "byr:1937 iyr:2017 cid:147 hgt:183cm\n",
            "\n",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n",
            "hcl:#cfa07d byr:1929\n",
            "hcl:#ae17e1 iyr:2013\n",
            "eyr:2024\n",
            "ecl:brn pid:760753108 byr:1931\n",
            "hgt:179cm\n",
            "\n",
            "hcl:#cfa07d eyr:2025 pid:166559648\n",
            "iyr:2011 ecl:brn hgt:59in\n",
        );
        let input = input_generator(text);
        assert_eq!(part1(&input), 2);
    }
    #[test]
    fn part2_sample1() {
        let text = concat!(
            "eyr:1972 cid:100\n",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n",
            "\n",
            "iyr:2019\n",
            "hcl:#602927 eyr:1967 hgt:170cm\n",
            "ecl:grn pid:012533040 byr:1946\n",
            "\n",
            "hcl:dab227 iyr:2012\n",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n",
            "\n",
            "hgt:59cm ecl:zzz\n",
            "eyr:2038 hcl:74454a iyr:2023\n",
            "pid:3556412378 byr:2007\n",
        );
        let input = input_generator(text);
        assert_eq!(part2(&input), 0);
    }
    #[test]
    fn part2_sample2() {
        let text = concat!(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n",
            "hcl:#623a2f\n",
            "\n",
            "eyr:2029 ecl:blu cid:129 byr:1989\n",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n",
            "\n",
            "hcl:#888785\n",
            "hgt:164cm byr:2001 iyr:2015 cid:88\n",
            "pid:545766238 ecl:hzl\n",
            "eyr:2022\n",
            "\n",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n",
        );
        let input = input_generator(text);
        assert_eq!(part2(&input), 4);
    }
}
