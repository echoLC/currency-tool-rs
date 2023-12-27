mod default_config;

pub use crate::default_config::*;

fn format_number_by_group_symbol(num: f32, symbol: String) -> String {
  String::from("5.2")
}

pub struct CurrencyRate {
  usd: f32,
  cny: f32,
  my: f32,
  jpy: f32
}

pub struct FormatOption {
  from: String,
  to: String,
  currency_rates: CurrencyRate
}

fn basic_format(value: f32, with_currency: bool, option: FormatOption) -> String {
  String::from("5.2")
}

pub fn format(value: f32, option: FormatOption) -> String {
  basic_format(value, true, option)
}

pub fn format_without_currency(value: f32, option: FormatOption) -> String {
  basic_format(value, false, option)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn default_currency () {
    assert_eq!(get_default_currency(), "CNY");
  }

  #[test]
  fn default_rate () {
    assert_eq!(get_default_rate(), 0.7);
  }

  #[test]
  fn default_digit() {
    assert_eq!(get_default_digit(), 2);
  }

  #[test]
  fn default_format() {
    assert_eq!(get_default_format(), "{{amount}}");
  }

  #[test]
  fn default_format_symbol() {
    assert_eq!(get_default_format_symbol(), "amount");
  }

  #[test]
  fn basic_format() {
    assert_eq!(format(100.0, FormatOption {
      from: String::from("USD"),
      to: String::from("EUR"),
      currency_rates: CurrencyRate{
        usd: 0.7,
        jpy: 0.4,
        my: 0.2,
        cny: 1.0
      }
    }), "€0,92 EUR");
  }
}