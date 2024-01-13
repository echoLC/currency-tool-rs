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

fn pretty_print_with_symbol (value: &str, symbol: char) -> String {
  let mut s = String::new();

  let a = value.chars().rev().enumerate();

  for(index, val) in a {
    if index != 0 && index % 3 == 0 {
      s.insert(0, symbol);
    }
    s.insert(0, val);
  }

  s
}

fn get_currency_symbol (currency: Option<&str>) -> String {
  match currency {
    Some("GBP") => String::from("£"),
    Some("USD") => String::from("$"),
    Some("CNY")  => String::from("￥"),
    Some(_)  => String::from("$"),
    None => String::from("$")
  }
}

pub fn format(value: f64, options: CommonFormatOption) -> String {
  let to = options.to;
  let currency_rates = options.currency_rates;
  let from_rate = currency_rates.usd;
  let to_rate = currency_rates.gbp;
  let convert_value = value * to_rate / from_rate;
  let convert_value_str = format!("{:.2}", convert_value);
  let convert_split_arr: Vec<&str> = convert_value_str.split(".").collect();
  let convert_int_part = convert_split_arr[0];
  let currency_symbol = get_currency_symbol(Some(&to)); 

  let format_str = pretty_print_with_symbol(convert_int_part, ',');


  currency_symbol + &format_str + "." + convert_split_arr[1] + " " + &to
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

  #[test]
  fn pretty_format() {
    assert_eq!(format(219930.00, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("GBP"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
      }
    }), "£177,854.31 GBP");
  }

  #[test]
  fn pretty_format_currency_symbol() {
    assert_eq!(format(219930.00, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("CNY"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
      }
    }), "￥177,854.31 CNY");
  }
}