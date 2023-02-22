#![allow(dead_code)]

fn digital_root(n: i64) -> i64 {
    let mut root = n;
    while root > 9 {
        root = root.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() as i64;
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!(digital_root(16), 7);
      assert_eq!(digital_root(1245), 3);
    }    
}
