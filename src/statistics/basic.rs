use std::collections::HashMap;

pub fn mean(data: &[i32]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let sum: i32 = data.iter().sum();

    sum as f64 / data.len() as f64
}

pub fn median(data: &[i32]) -> i32 {
    if data.is_empty() {
        return 0;
    }

    let mut data_copy = data.to_vec();
    data_copy.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = data.len();
    if len % 2 == 0 {
        let m1 = data_copy[len / 2];
        let m2 = data_copy[len / 2 - 1];

        (m1 + m2) / 2
    } else {
        data_copy[len / 2]
    }
}

pub fn moda(data: &[i32]) -> i32 {
    if data.is_empty() {
        return 0;
    }
    
    let mut map: HashMap<i32, i32> = HashMap::new();

    for v in data.iter() {
        let count = map.entry(*v).or_insert(0);
        *count += 1;
    }

    map.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(v, _)| v)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(&vec![1, 2, 3, 4, 5]), 3.0);
        assert_eq!(mean(&vec![]), 0.0);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&vec![1, 2, 3, 4, 5]), 3);
        assert_eq!(median(&vec![1, 2, 3, 4]), 2);
    }

    #[test]
    fn test_moda() {
        assert_eq!(moda(&vec![1, 2, 2, 3]), 2);
        assert_eq!(moda(&vec![]), 0);
    }
}
