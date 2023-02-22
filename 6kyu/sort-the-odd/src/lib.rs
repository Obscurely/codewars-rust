#![allow(dead_code)]

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let enumerated = arr.to_owned().into_iter().enumerate();
    let odd: (Vec<usize>, Vec<i32>) = enumerated.to_owned().filter(|x| x.1 % 2 != 0).unzip();
    let (odd_indexes, mut odd_sorted_values) = odd;
    odd_sorted_values.sort();
    let odd_index_sorted: Vec<(usize, i32)> = odd_indexes.into_iter().zip(odd_sorted_values.into_iter()).collect();
    
    let mut arr_final = arr.to_owned();
    for x in odd_index_sorted {
        arr_final[x.0] = x.1;
    }

    arr_final
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}
