#![allow(dead_code)]

fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut tower = Vec::with_capacity(n_floors);
    let max_stars = n_floors * 2 - 1;
    let mut i = 0;
    while max_stars > i {
        tower.push(" ".repeat(i /2) + &"*".repeat(max_stars - i) + &" ".repeat(i / 2));
        i += 2;
    }
    tower.reverse();
    tower
}

#[cfg(test)]
mod tests {
    use super::tower_builder;

    #[test]
    fn fixed_tests() {
        assert_eq!(tower_builder(1), vec!["*"]);
        assert_eq!(tower_builder(2), vec![" * ", "***"]);
        assert_eq!(tower_builder(3), vec!["  *  ", " *** ", "*****"]);
    }
}
