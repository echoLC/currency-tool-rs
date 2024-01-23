use currency_tool_rs::{get_default_currency, get_default_rate};

fn main() {
    println!("default currency:{}", get_default_currency());
    println!("default currency rate:{}", get_default_rate());
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("hello");

    {
        let string2 = String::from("world");
        let result = longest(&string1, &string2);

        println!("result is {}", result);
    }
}
