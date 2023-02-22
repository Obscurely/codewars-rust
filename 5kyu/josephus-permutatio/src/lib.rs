#![allow(dead_code)]

fn josephus<T:Clone+Copy>(xs:Vec<T>,k:usize)-> Vec<T> {
    let mut xs = xs.to_owned();
    let mut eliminated = Vec::new();
    let mut current_index = k - 1;
    while xs.len() != 0 {
        if current_index >= xs.len() {
            current_index = current_index - xs.len();
            continue;
        }

        eliminated.push(xs.remove(current_index));

        current_index += k - 1;
    }
    eliminated
}

#[cfg(test)]
mod josephus {
    use super::josephus;

    #[test]
    fn test_works_with_integers() {
      assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
      assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2), vec![2, 4, 6, 8, 10, 3, 7, 1, 9, 5]);
    }
    #[test]
    fn test_works_with_strings() {
        assert_eq!(josephus("CodeWars".chars().collect::<Vec<char>>(), 4), "esWoCdra".chars().collect::<Vec<char>>());
    }
}
