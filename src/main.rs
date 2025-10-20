use rust_data_utils::statistics::*;

fn main() {
    let data = vec![3, 2, 5, 1, 4, 2, 2, 2, 5];

    let mean = mean(&data);

    println!("mean: {}", mean);

    let median = median(&data);

    println!("median: {}", median);

    let median = moda(&data);

    println!("median: {}", median);
}
