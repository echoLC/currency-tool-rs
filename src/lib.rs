mod default_config;

use std::option;

pub use crate::default_config::*;

fn format_number_by_group_symbol(num: f32, symbol: String) -> String {
  String::from("5.2")
}

pub struct CurrencyRate {
  usd: f32,
  cny: Option<f32>,
  my: Option<f32>,
  jpy: Option<f32>,
  gbp: f32
}

pub struct FormatOption {
  from: &'static str,
  to: &'static str,
  currency_rates: CurrencyRate
}

pub fn convert_calc(value: f32, from: &'static str, to: &'static str, rates: CurrencyRate) -> f32 {
  if from == to {
    return value
  }

  return (value * (rates.usd)) / (rates.gbp)
}

fn basic_format(value: f32, with_currency: bool, option: FormatOption) -> &'static str {
  let to = option.to;
  let from = option.from;
  let rates = option.currency_rates; 

  let calc_value = convert_calc(value, to, from, rates);



}

pub fn format(value: f32, option: FormatOption) -> &'static str {
  basic_format(value, true, option)
}

pub fn format_without_currency(value: f32, option: FormatOption) -> &'static str {
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
      from: "USD",
      to: "GBP",
      currency_rates: CurrencyRate{
        usd: 1.0,
        gbp: 0.808686,
        cny: None,
        jpy: None,
        my: None
      }
    }), "£1,865.07 GBP");
  }
}