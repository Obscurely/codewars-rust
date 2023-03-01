#![allow(dead_code)]
fn order_weight(s: &str) -> String {
    let mut s = s
        .trim()
        .split(" ")
        .filter(|s| s != &"")
        .collect::<Vec<&str>>();
    s.sort();
    s.sort_by(|a, b| {
        a.chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>()
            .cmp(&b.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
    });
    s.join(" ")
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
