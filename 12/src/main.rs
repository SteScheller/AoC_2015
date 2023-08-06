use serde_json::Value;

use common;

fn compute_sum(object: Value) -> i32 {
    let mut sum = 0;
    match object {
        Value::Object(obj) => {
            for (_key, value) in obj {
                sum += compute_sum(value);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                sum += compute_sum(item);
            }
        }
        Value::Number(num) => {
            if let Some(num) = num.as_i64() {
                sum += num as i32;
            }
        }
        _ => (),
    }
    sum
}

fn part_one(input: &str) -> i32 {
    let json: Value = serde_json::from_str(input).unwrap();

    compute_sum(json)
}

fn part_two(_input: &str) -> i32 {
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
        assert_eq!(part_one("[1,2,3]"), 6);
        assert_eq!(part_one("{\"a\": 2, \"b\":4}"), 6);
        assert_eq!(part_one("[[[3]]]"), 3);
        assert_eq!(part_one("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(part_one("{\"a\":[-1,1]}"), 0);
        assert_eq!(part_one("[-1,{\"a\":1}]"), 0);
        assert_eq!(part_one("[]"), 0);
        assert_eq!(part_one("{}"), 0);
    }

    #[test]
    fn test_part_two() {
        assert!(false);
    }
}
