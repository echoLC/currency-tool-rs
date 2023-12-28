mod default_config;

pub use crate::default_config::*;

pub struct CurrencyRates {
  USD: f32,
  GBP: f32
}

pub struct CommonFormatOption {
  from: String,
  to: String,
  currency_rates: CurrencyRates
}

pub fn format(value: f64, options: CommonFormatOption) -> String {
  return String::from("£1,865.07 GBP")
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
    assert_eq!(format(100.0, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("GBP"),
      currency_rates: CurrencyRates{
        USD: 1.0,
        GBP: 0.808686,
      }
    }), "£1,865.07 GBP");
  }
}