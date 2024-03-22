fn is_prime (num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let mut i = 2;

    while i <= (num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut index = 0;
    let mut num = 2;

    while index <= n {
        if index == n {
            return num;
        } else {
            num += 1;
        }

        if is_prime(num) {
            index += 1;
        }
    }

    num
}

fn main() {
   nth(1);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_prime() {
        assert_eq!(nth(0), 2);
    }
    #[test]
    fn second_prime() {
        assert_eq!(nth(1), 3);
    }
    #[test]
    fn sixth_prime() {
        assert_eq!(nth(5), 13);
    }
    #[test]
    fn big_prime() {
        assert_eq!(nth(10_000), 104_743);
    }

}

