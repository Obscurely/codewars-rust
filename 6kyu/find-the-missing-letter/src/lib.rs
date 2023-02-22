#![allow(dead_code)]

fn find_missing_letter(chars: &[char]) -> char {
    (chars[0]..=chars[chars.len() -1]).filter(|x| !chars.contains(x)).collect::<Vec<char>>()[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}
