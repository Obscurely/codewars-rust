#![allow(unused)]
fn ips_between(start: &str, end: &str) -> u32 {
    let start_split: Vec<i128> = start.split(".").into_iter().map(|n| n.parse().unwrap()).collect();
    let end_split: Vec<i128> = end.split(".").into_iter().map(|n| n.parse().unwrap()).collect();

    let mut sum = 0;
    sum += start_split[3] - end_split[3];
    sum += (start_split[2] - end_split[2]) * 256;
    sum += (start_split[1] - end_split[1]) * 256 * 256;
    sum += (start_split[0] - end_split[0]) * 256 * 256 * 256;

    sum.abs().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}
