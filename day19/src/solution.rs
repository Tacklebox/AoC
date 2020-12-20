use regex::RegexSet;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Rule {
    Lit(char),
    Sequence(Vec<i32>),
    Either(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn from_str(source: &str, rule_type: usize) -> Self {
        match rule_type {
            0 => {
                let mut parts = source.split(" | ").map(|seq| {
                    seq.split(" ")
                        .map(|r| r.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                }).collect::<Vec<Vec<i32>>>();
                Rule::Either(
                    Box::new(Rule::Sequence(parts.swap_remove(0))),
                    Box::new(Rule::Sequence(parts.swap_remove(0))),
                )
            }
            1 => {
                Rule::Sequence(
                    source
                    .split(" ")
                    .map(|r| r.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
                )
            }
            2 => {
                Rule::Lit(source.chars().skip(1).next().unwrap())
            }
            _ => unreachable!(),
        }
    }
}
struct Rules {
    rules: HashMap<i32, Rule>,
}

impl Rules {
    fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut rules: HashMap<i32, Rule> = HashMap::new();
        let patterns = &[
            r"^\d+: (\d+ )+\| (\d+ )*\d+$",
            r"^\d+: (\d+ )*\d+$",
            r#"^\d+: "."$"#,
        ];
        let set = RegexSet::new(patterns).unwrap();
        for line in lines {
            if line == "" {
                break;
            }
            let matches: Vec<_> = set.matches(&line).into_iter().collect();
            if matches.len() != 1 {
                panic!("Somehow matched multiple rules");
            }
            let parts: Vec<&str> = line.split(": ").collect();
            rules.insert(parts[0].parse().unwrap(), Rule::from_str(parts[1], matches[0]));
        }
        Rules { rules }
    }

    fn matches(&self, r: Option<&Rule>, s: &str) -> Option<usize> {
        let r = if let Some(r) = r {
            r
        } else {
            self.rules.get(&0).unwrap()
        };
        match r {
            Rule::Lit(c) => {
                if let Some(first) = s.chars().nth(0) {
                    if first == *c {
                        return Some(1);
                    }
                }
            }
            Rule::Sequence(seq) => {
                let mut distance = 0;
                for &r in seq.iter() {
                    if let Some(further) =
                        self.matches(Some(self.rules.get(&r).unwrap()), &s[distance..])
                    {
                        distance += further;
                    } else {
                        return None;
                    }
                }
                return Some(distance);
            }
            Rule::Either(r1, r2) => {
                if let Some(distance) = self.matches(Some(&r1), s) {
                    return Some(distance);
                }
                if let Some(distance) = self.matches(Some(&r2), s) {
                    return Some(distance);
                }
                return None;
            }
        }
        None
    }
}

pub fn part1(mut input: impl Iterator<Item = String>) -> i64 {
    let rules = Rules::from_lines(&mut input);
    let mut matches = 0;
    for line in input {
        if let Some(len) = rules.matches(None, &line) {
            if len == line.len() {
                matches += 1;
            }
        }
    }
    matches
}

pub fn part2(mut input: impl Iterator<Item = String>) -> i64 {
    let rules = Rules::from_lines(&mut input);
    let mut matches = 0;
    for line in input {
        if let Some(len) = rules.matches(None, &line) {
            if len == line.len() {
                matches += 1;
            }
        }
    }
    matches
}

mod test {
    #[test]
    fn parse_rule() {
        use super::Rule::Sequence;
        use super::Rule::Either;
        use super::Rule::Lit;
        use super::Rule;
        assert_eq!(Rule::from_str("4 1 5", 1), Sequence(vec![4,1,5]));
        assert_eq!(Rule::from_str("2 3 | 3 2", 0), Either(Box::new(Sequence(vec![2,3])), Box::new(Sequence(vec![3,2]))));
        assert_eq!(Rule::from_str("4 4 | 5 5", 0), Either(Box::new(Sequence(vec![4,4])), Box::new(Sequence(vec![5,5]))));
        assert_eq!(Rule::from_str("4 5 | 5 4", 0), Either(Box::new(Sequence(vec![4,5])), Box::new(Sequence(vec![5,4]))));
        assert_eq!(Rule::from_str("\"a\"", 2), Lit('a'));
        assert_eq!(Rule::from_str("\"b\"", 2), Lit('b'));
    }

    #[test]
    fn part1() {
        let input = vec![
            String::from("0: 4 1 5"),
            String::from("1: 2 3 | 3 2"),
            String::from("2: 4 4 | 5 5"),
            String::from("3: 4 5 | 5 4"),
            String::from("4: \"a\""),
            String::from("5: \"b\""),
            String::from(""),
            String::from("ababbb"),
            String::from("bababa"),
            String::from("abbbab"),
            String::from("aaabbb"),
            String::from("aaaabbb"),
        ];
        assert_eq!(super::part1(&mut input.into_iter()), 2);
    }

    #[test]
    fn part2() {
        let input = vec![
            String::from("42: 9 14 | 10 1"),
            String::from("9: 14 27 | 1 26"),
            String::from("10: 23 14 | 28 1"),
            String::from("1: \"a\""),
            String::from("11: 42 31 | 42 11 31"),
            String::from("5: 1 14 | 15 1"),
            String::from("19: 14 1 | 14 14"),
            String::from("12: 24 14 | 19 1"),
            String::from("16: 15 1 | 14 14"),
            String::from("31: 14 17 | 1 13"),
            String::from("6: 14 14 | 1 14"),
            String::from("2: 1 24 | 14 4"),
            String::from("0: 8 11"),
            String::from("13: 14 3 | 1 12"),
            String::from("15: 1 | 14"),
            String::from("17: 14 2 | 1 7"),
            String::from("23: 25 1 | 22 14"),
            String::from("28: 16 1"),
            String::from("4: 1 1"),
            String::from("20: 14 14 | 1 15"),
            String::from("3: 5 14 | 16 1"),
            String::from("27: 1 6 | 14 18"),
            String::from("14: \"b\""),
            String::from("21: 14 1 | 1 14"),
            String::from("25: 1 1 | 1 14"),
            String::from("22: 14 14"),
            String::from("8: 42 | 42 8"),
            String::from("26: 14 22 | 1 20"),
            String::from("18: 15 15"),
            String::from("7: 14 5 | 1 21"),
            String::from("24: 14 1"),
            String::from(""),
            String::from("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa"),
            String::from("bbabbbbaabaabba"),
            String::from("babbbbaabbbbbabbbbbbaabaaabaaa"),
            String::from("aaabbbbbbaaaabaababaabababbabaaabbababababaaa"),
            String::from("bbbbbbbaaaabbbbaaabbabaaa"),
            String::from("bbbababbbbaaaaaaaabbababaaababaabab"),
            String::from("ababaaaaaabaaab"),
            String::from("ababaaaaabbbaba"),
            String::from("baabbaaaabbaaaababbaababb"),
            String::from("abbbbabbbbaaaababbbbbbaaaababb"),
            String::from("aaaaabbaabaaaaababaa"),
            String::from("aaaabbaaaabbaaa"),
            String::from("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa"),
            String::from("babaaabbbaaabaababbaabababaaab"),
            String::from("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"),
            ];
        assert_eq!(super::part2(&mut input.into_iter()), 12);
    }
}
