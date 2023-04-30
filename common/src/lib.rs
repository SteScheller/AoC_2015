use std::fs;

pub fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let reference = include_str!("../ressources/some_input.txt");
        assert_eq!(read_input("ressources/some_input.txt"), reference);
    }
}
