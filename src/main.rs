pub fn collatz(n: u64) -> Option<u64> {

    if n == 0 {
        return None;
    }

    let mut new_value = n;
    let mut count: u64 = 0;

    while new_value != 1 {
        if new_value % 2 == 0 {
            new_value = new_value / 2;
        } else {
            // overflow in runtime possibly，return directly
            if new_value.checked_mul(3) == None || new_value.checked_mul(3).unwrap().checked_add(1) == None {
                return None;
            } else {
                new_value = new_value * 3 + 1; 
            }
        }

        count = count + 1;
    }

    Some(count)
}


fn main() {
    let x: u8 = 10;

    let v = x.checked_add(u8::MAX).unwrap_or(0);
    println!("{}", v)
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
fn one() {
    assert_eq!(Some(0), collatz(1));
}
#[test]
fn sixteen() {
    assert_eq!(Some(4), collatz(16));
}
#[test]
fn twelve() {
    assert_eq!(Some(9), collatz(12));
}
#[test]
fn one_million() {
    assert_eq!(Some(152), collatz(1_000_000));
}
#[test]
fn zero() {
    assert_eq!(None, collatz(0));
}
#[test]
fn test_110243094271() {
    let val = 110243094271;
    assert_eq!(None, collatz(val));
}
#[test]
fn max_div_3() {
    let max = u64::MAX / 3;
    assert_eq!(None, collatz(max));
}
#[test]
fn max_minus_1() {
    let max = u64::MAX - 1;
    assert_eq!(None, collatz(max));
}  
  
}

