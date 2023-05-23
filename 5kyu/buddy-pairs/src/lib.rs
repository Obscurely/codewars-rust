#![allow(unused)]
// Getting the divisors of the numbers using prime factorization.
// I've actually learned how prime factorization works and how to get the divisors of a number using it.
// This code is completly composed by my own mind and what I tought would yield the most
// performance by going through a lot of testing and failures, but I finally got it.
// It's actually quite fast.
// I'm pretty proud honestly :).
fn get_divisors(n: i64) -> Vec<i64> {
    let mut divisors = vec![n];

    let mut factor = 2;
    while n % factor == 0 && factor < n {
        divisors.push(n / factor);
        divisors.push(factor);
        factor *= 2;
    }

    for div in divisors.clone() {
        let n_sqrt = f64::sqrt(n as f64) as i64;
        let mut i = 3;
        while i <= n_sqrt {
            let mut new_div = div;
            while new_div % i == 0 {
                new_div /= i;
                if !divisors.contains(&new_div) {
                    divisors.push(new_div);
                }
                if !divisors.contains(&i) {
                    divisors.push(i);
                }
            }
            i += 1;
        }
    }

    // Add 1 to the divisors if not present
    if !divisors.contains(&1) {
        divisors.push(1);
    }

    divisors.sort_unstable();
    divisors
}

fn get_divisors_without_n(n: i64) -> Vec<i64> {
    let mut v = get_divisors(n);
    v.pop();
    v
}

fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
    (start..=limit)
        .map(|n| {
            let divisors = get_divisors_without_n(n);
            let div_sum = divisors.iter().sum::<i64>();
            (n, div_sum - 1) // -1 in order to get the buddy pair.
        })
        .find(|n| get_divisors_without_n(n.1).iter().sum::<i64>() - 1 == n.0 && n.0 < n.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(start: i64, limit: i64, exp: Option<(i64, i64)>) -> () {
        println!("start:{}", start);
        println!("limit:{}", limit);
        let ans = buddy(start, limit);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10, 50, Some((48, 75)));
        dotest(1081180, 1103735, Some((1081184, 1331967)));
    }
}
