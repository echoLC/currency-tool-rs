pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let power_digit = num_str.len();
    let sum = num_str.chars().map(|c| c.to_digit(10).unwrap().pow(power_digit as u32) as u64).sum::<u64>();

    sum == num as u64
}

fn main() {
    assert_eq!(is_armstrong_number(3_999_999_999), true);
}
