use currency_tool_rs::{get_default_currency, get_default_rate, get_frac};

fn main() {
    println!("default currency:{}", get_default_currency());
    println!("default currency rate:{}", get_default_rate());
    println!("{}", get_frac(1.212));
    println!("{}", get_frac(3.1412));
    println!("{}", get_frac(3.000));
}
