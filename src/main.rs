pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    } 

    if s == 1 {
        1
    } else {
        square(s - 1) * 2
    }
}

pub fn total() -> u64 {
    let mut sum = 0;

    for i in 1..65 {
        sum += square(i)
    }

    sum
}

fn main() {
   
}

#[cfg(test)]
mod tests {

    use super::*;

    pub fn process_square_case(input: u32, expected: u64) {
        assert_eq!(square(input), expected);
    }

        #[test]
    fn one() {
        process_square_case(1, 1);
    }
    #[test]
    #[ignore]
    fn two() {
        process_square_case(2, 2);
    }
    #[test]
    #[ignore]
    fn three() {
        process_square_case(3, 4);
    }
    #[test]
    #[ignore]
    fn four() {
        process_square_case(4, 8);
    }
    #[test]
    #[ignore]
    fn sixteen() {
        process_square_case(16, 32_768);
    }
    #[test]
    #[ignore]
    fn thirty_two() {
        process_square_case(32, 2_147_483_648);
    }
    #[test]
    #[ignore]
    fn sixty_four() {
        process_square_case(64, 9_223_372_036_854_775_808);
    }
    #[test]
    #[ignore]
    #[should_panic(expected = "Square must be between 1 and 64")]
    fn square_0_raises_an_exception() {
        square(0);
    }
    #[test]
    #[ignore]
    #[should_panic(expected = "Square must be between 1 and 64")]
    fn square_greater_than_64_raises_an_exception() {
        square(65);
    }
    #[test]
    #[ignore]
    fn returns_the_total_number_of_grains_on_the_board() {
        assert_eq!(total(), 18_446_744_073_709_551_615);
    }
}

