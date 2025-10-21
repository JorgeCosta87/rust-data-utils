use rust_data_utils::statistics::*;
use rust_data_utils::codex::*;

fn main() {
    let data = vec![3, 2, 5, 1, 4, 2, 2, 2, 5];

    let mean = mean(&data);

    println!("mean: {}", mean);

    let median = median(&data);

    println!("median: {}", median);

    let median = moda(&data);

    println!("median: {}", median);

    let string = String::from("first");
    let converted_str = convert_to_pig_latin(&string);

    println!("original text: {} - pig latin text: {}", string, converted_str);

    let string = String::from("안irst");
    let converted_str = safe_convert_to_pig_latin(&string);
    println!("[SAFE] original text: {} - pig latin text: {}", string, converted_str);
}
