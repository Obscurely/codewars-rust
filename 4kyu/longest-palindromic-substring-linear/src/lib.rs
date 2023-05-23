#![allow(unused)]

use std::{todo, dbg};

fn longest_palindrome(s: &str) -> String {
    if s.len() < 2 {
        return s.to_string();
    }

    if s == s.chars().rev().collect::<String>() {
        return s.to_string();
    }

    let mut longest = "";

    for n in 0..s.len() {
        let sub = s.split_at(n).1;
        
        if sub.len() < longest.len() {
            break;
        }

        for x in (0..sub.len() + 1).rev() {
            let palindrome = sub.split_at(x).0;
            if palindrome == palindrome.chars().rev().collect::<String>() {
                if palindrome.len() > longest.len() {
                    longest = palindrome;
                    break;
                }
            }
        }
    }

    longest.to_string()
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;
    
    const ERR_MSG: &str = ", your result (left) did not match the expected output (right)";
    
    fn do_test(s: &str, expected: &str) {
        if s.len() < 300 {
            assert_eq!(longest_palindrome(s), expected, "\nWith s = \"{s}\"{ERR_MSG}")
        } else {
            assert_eq!(longest_palindrome(s), expected, "\nWith input string of length {}{}", s.len(), ERR_MSG)
        }
    }

    #[test]
    fn odd_length_strings() {
        for (s, expected) in [("babad", "bab"), ("madam", "madam"), ("dde", "dd"), ("ababbab", "babbab"), ("abababa", "abababa")] {
            do_test(s, expected);
        }
    }
    #[test]
    fn even_length_strings() {
        for (s, expected) in [("banana", "anana"), ("abba", "abba"), ("cbbd", "bb"), ("zz", "zz"), ("dddd", "dddd")] {
            do_test(s, expected);
        }
    }
    #[test]
    fn edge_cases() {
        for (s, expected) in [("", ""), ("abcdefghijklmnopqrstuvwxyz", "a"), ("ttaaftffftfaafatf", "aaftffftfaa"), ("bbaaacc", "aaa"), ("m", "m")] {
            do_test(s, expected);
        }
    }
    #[test]
    fn performance_test() {
        let s = &"a".repeat(10000);
        do_test(s, s);
    }
}
