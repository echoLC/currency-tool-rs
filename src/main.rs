use currency_tool_rs::{get_default_currency, get_default_rate};

fn main() {
    println!("default currency:{}", get_default_currency());
    println!("default currency rate:{}", get_default_rate());
}
