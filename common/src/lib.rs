use std::fs;

pub fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

#[macro_export]
macro_rules! parametrized_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (func, input, expected) = $value;
            assert_eq!(func(input), expected);
        }
    )*
    }
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
