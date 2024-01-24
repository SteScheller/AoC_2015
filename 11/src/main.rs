use std::collections::HashSet;
use std::str::FromStr;

fn increment_pw(pw: &mut String) {
    let mut completed_increment = false;
    let old_pw = pw.clone();
    for ic in old_pw.char_indices().rev() {
        let (i, c) = ic;
        match c {
            'z' => pw.replace_range(i..i + 1, "a"),
            _ => {
                pw.replace_range(
                    i..i + 1,
                    &std::char::from_u32(c as u32 + 1).unwrap().to_string(),
                );
                completed_increment = true;
                break;
            }
        }
    }
    if !completed_increment {
        pw.insert(0, 'a');
    }
}

fn contains_forbidden_letters(pw: &str) -> bool {
    let mut result = false;
    for c in pw.chars() {
        match c {
            'i' | 'o' | 'l' => {
                result = true;
                break;
            }
            _ => (),
        };
    }
    result
}

fn contains_increasing_straight(pw: &str) -> bool {
    let mut found_straight = false;
    let mut last_codepoint = pw.chars().next().unwrap() as u32;
    let mut len_straight = 1;

    for c in pw.chars().skip(1) {
        let codepoint = c as u32;
        if codepoint == (last_codepoint + 1) {
            len_straight += 1;
        } else {
            len_straight = 1;
        }

        if len_straight >= 3 {
            found_straight = true;
            break;
        }
        last_codepoint = codepoint;
    }
    found_straight
}

fn contains_two_pairs(pw: &str) -> bool {
    let mut pair_letters = HashSet::new();
    let mut last_letter = None;

    for cl in pw.chars() {
        if let Some(ll) = last_letter {
            if cl == ll {
                pair_letters.insert(cl);
                last_letter = None;
            } else {
                last_letter = Some(cl);
            }
        } else {
            last_letter = Some(cl);
        }
    }
    pair_letters.len() >= 2
}

fn part_one(input: &str) -> String {
    let mut pw = String::from_str(input).unwrap();
    while contains_forbidden_letters(&pw)
        || !contains_increasing_straight(&pw)
        || !contains_two_pairs(&pw)
    {
        increment_pw(&mut pw);
    }
    pw
}

fn part_two(input: &str) -> String {
    let mut pw = part_one(input);
    increment_pw(&mut pw);
    part_one(&pw)
}

fn main() {
    let input = common::read_input("input.txt");
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_increment() {
        let inc = |pw: &str| -> String {
            let mut a = String::from_str(pw).unwrap();
            increment_pw(&mut a);
            a
        };
        assert_eq!(inc("a"), "b");
        assert_eq!(inc("m"), "n");
        assert_eq!(inc("y"), "z");
        assert_eq!(inc("z"), "aa");
        assert_eq!(inc("aa"), "ab");
        assert_eq!(inc("zz"), "aaa");
        assert_eq!(inc("hxbxwxba"), "hxbxwxbb");
    }

    #[test]
    fn test_contains_forbidden_letters() {
        assert!(!contains_forbidden_letters("abc"));
        assert!(contains_forbidden_letters("aibc"));
        assert!(contains_forbidden_letters("abco"));
        assert!(contains_forbidden_letters("labc"));
    }

    #[test]
    fn test_contains_increasing_straight() {
        assert!(contains_increasing_straight("abc"));
        assert!(contains_increasing_straight("mno"));
        assert!(contains_increasing_straight("xyz"));
        assert!(!contains_increasing_straight("abd"));
        assert!(!contains_increasing_straight("xyyz"));
    }

    #[test]
    fn test_two_pairs() {
        assert!(contains_two_pairs("aabb"));
        assert!(contains_two_pairs("ccwqerr"));
        assert!(contains_two_pairs("czzwqeaaf"));
        assert!(!contains_two_pairs("zzz"));
        assert!(!contains_two_pairs("zzzz"));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("abcdefgh"), "abcdffaa");
        assert_eq!(part_one("ghijklmn"), "ghjaabcc");
    }
}
