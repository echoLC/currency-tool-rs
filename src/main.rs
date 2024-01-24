use currency_tool_rs::{get_default_currency, get_default_rate};
use std::fmt::Display;

fn main() {
    println!("default currency:{}", get_default_currency());
    println!("default currency rate:{}", get_default_rate());
    
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
        println!("Announcement! {}", ann);

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("hello");

    {
        let string2 = String::from("world");
        let result = longest_with_an_announcement(&string1, &string2, String::from("hello"));

        println!("result is {}", result);
    }
}
