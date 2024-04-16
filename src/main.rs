pub fn egg_count(display_value: u32) -> usize {
    let binary_str = format!("{:b}", display_value);
    let mut count = 0;

    for c in binary_str.chars() {
        if c == '1' {
            count += 1;
        }
    }

    count
}


fn main() {
    let res = format!("{:b}", 16);
    println!("{}", res);
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
fn test_0_eggs() {
    let input = 0;
    let output = egg_count(input);
    let expected = 0;
    assert_eq!(output, expected);
}
#[test]
fn test_1_egg() {
    let input = 16;
    let output = egg_count(input);
    let expected = 1;
    assert_eq!(output, expected);
}
#[test]
fn test_4_eggs() {
    let input = 89;
    let output = egg_count(input);
    let expected = 4;
    assert_eq!(output, expected);
}
#[test]
fn test_13_eggs() {
    let input = 2000000000;
    let output = egg_count(input);
    let expected = 13;
    assert_eq!(output, expected);
}

}

