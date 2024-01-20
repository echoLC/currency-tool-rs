mod default_config;


pub use crate::default_config::*;

#[derive(Debug, Clone)]
pub struct CurrencyRates {
  usd: f64,
  gbp: f64,
  cny: f64
}

#[derive(Debug, Clone)]
pub struct CommonFormatOption {
  from: String,
  to: String,
  currency_rates: CurrencyRates
}

pub struct ConvertRates {
  to: f64,
  from: f64
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

fn get_convert_rates(from_currency: &str, to_currency: &str, rates: CurrencyRates) -> ConvertRates  {

  let mut to_rate: f64 = 1.0;
  let mut from_rate: f64 = 1.0;

  if from_currency == "USD" {
    from_rate = rates.usd;
  } else if from_currency == "GBP" {
    from_rate = rates.gbp;
  } else if from_currency == "CNY" {
    from_rate = rates.cny;
  }

  if to_currency == "USD" {
    to_rate = rates.usd;
  } else if to_currency == "GBP" {
    to_rate = rates.gbp;
  } else if to_currency == "CNY" {
    to_rate = rates.cny;
  }

  ConvertRates{
    to: to_rate,
    from: from_rate
  }
}

pub fn format(value: f64, options: CommonFormatOption) -> String {
  let to = options.to;
  let currency_rates = get_convert_rates(&options.from, &to, options.currency_rates);

  let convert_value = value * currency_rates.to / currency_rates.from;
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
        cny: 0.140449
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
        cny: 0.140449
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
        cny: 0.140449
      }
    }), "￥177,854.31 CNY");
  }

  #[test]
  fn format_different_rates() {
    assert_eq!(format(219930.00, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("CNY"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
        cny: 0.140449
      }
    }), "￥30,888.95 CNY");
  }
}