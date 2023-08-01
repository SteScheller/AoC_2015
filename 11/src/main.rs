use common;

fn increment_pw(pw: &mut String) {
    let mut idx = None;
    for ic in pw.char_indices() {
        let (i, c) = ic;
        match c {
            'z' => {}
            _ => {
                idx = Some(i);
                break;
            }
        }
    }
    if let Some(i) = idx {
        let c = pw.chars().nth(i).unwrap();
        pw.replace_range(
            i..i + 1,
            &std::char::from_u32(c as u32 + 1).unwrap().to_string(),
        );
    } else {
        pw.insert(0, 'a');
    }
}

fn part_one(_input: &str) -> String {
    String::new()
}

fn part_two(_input: &str) -> String {
    String::new()
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
    }
}
