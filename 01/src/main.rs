use std::fs;

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

fn part_one(input: &str) -> i32 {
    let mut result = 0;
    for c in input.chars() {
        match c {
            '(' => result += 1,
            ')' => result -= 1,
            _ => (),
        }
    }

    result
}
fn main() {
    let input = read_input("input.txt");
    println!("{}", part_one(&input));
}

#[test]
fn test_part_one() {
    assert!((part_one("(())") == part_one("()()")) && part_one("(())") == 0);
    assert!(
        (part_one("(((") == part_one("(()(()("))
            && (part_one("(((") == part_one("))((((("))
            && part_one("(((") == 3
    );
    assert!((part_one("())") == part_one("))(")) && part_one("())") == -1);
    assert!((part_one(")))") == part_one(")())())")) && part_one(")))") == -3);
}
