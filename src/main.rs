use currency_tool_rs::{get_default_currency, get_default_rate, SymbolOrderEnum};

fn main() {
    println!("default currency:{}", get_default_currency());
    println!("default currency rate:{}", get_default_rate());
    println!("{:.2}", 12.0);

    let prefix = SymbolOrderEnum::Prefix;

    dbg!(prefix);
}
