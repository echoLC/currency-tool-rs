mod default_config;

pub use crate::default_config::*;

pub struct CurrencyRates {
  usd: f64,
  gbp: f64
}

pub struct CommonFormatOption {
  from: String,
  to: String,
  currency_rates: CurrencyRates
}



pub fn format(value: f64, options: CommonFormatOption) -> String {
  let to = options.to;
  let currency_rates = options.currency_rates;
  let from_rate = currency_rates.usd;
  let to_rate = currency_rates.gbp;
  let convert_value = value * to_rate / from_rate;
  let convert_value_str = format!("{:.2}", convert_value);


  String::from("£") + &convert_value_str + " " + &to
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
        usd: 1.0,
        gbp: 0.808686,
      }
    }), "£80.87 GBP");
  }
}