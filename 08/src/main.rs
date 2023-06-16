use regex::Regex;

use common;

fn parse_string_code(input: &str) -> String {
    let re_content = Regex::new(r#"^\"(.*)\"$"#).unwrap();
    let content = match re_content.captures(input) {
        Some(c) => c.get(1).unwrap().as_str(),
        _ => "",
    };
    let mut chars = content.chars();
    let mut parsed = String::new();

    while let Some(c) = chars.next() {
        if c == '\\' {
            let escaped_c = chars.next().unwrap();
            let translated_c = match escaped_c {
                'n' => '\n',
                '\\' => '\\',
                'x' => {
                    let first = chars.next().unwrap();
                    let second = chars.next().unwrap();
                    let radix = [first, second].iter().collect::<String>();
                    let codepoint = u8::from_str_radix(radix.as_str(), 16).unwrap();
                    char::from_u32(codepoint as u32).unwrap()
                }
                _ => escaped_c,
            };
            parsed.push(translated_c);
        } else {
            parsed.push(c);
        }
    }
    parsed
}

fn part_one(input: &str) -> usize {
    let mut size = 0;
    let mut num_characters = 0;
    for line in input.lines() {
        size += line.as_bytes().len();
        num_characters += parse_string_code(line).chars().count();
    }
    size - num_characters
}

fn part_two(_input: &str) -> usize {
    0
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(r#""""#), 2);
        assert_eq!(part_one(r#""abc""#), 2);
        assert_eq!(part_one(r#""aaa\"aaa""#), 3);
        assert_eq!(part_one(r#""\x27""#), 5);
    }
}
