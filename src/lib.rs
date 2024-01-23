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
  currency_rates: CurrencyRates,
  with_currency: bool,
}

pub struct ConvertRates {
  to: f64,
  from: f64
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

fn get_convert_rates(from_currency: &str, to_currency: &str, rates: CurrencyRates) -> Result<ConvertRates, String>  {

  let mut to_rate: f64 = 1.0;
  let mut from_rate: f64 = 1.0;

  if from_currency == "USD" {
    from_rate = rates.usd;
  } else if from_currency == "GBP" {
    from_rate = rates.gbp;
  } else if from_currency == "CNY" {
    from_rate = rates.cny;
  } else {
    return Err(String::from("unknown currency"))
  }

  if to_currency == "USD" {
    to_rate = rates.usd;
  } else if to_currency == "GBP" {
    to_rate = rates.gbp;
  } else if to_currency == "CNY" {
    to_rate = rates.cny;
  } else {
    return Err(String::from("unknown currency"))
  }

  Ok(ConvertRates{
    to: to_rate,
    from: from_rate
  })
}

/** `pretty_print_with_symbol` 使用指定的分隔符美化字符串金额
  
  # Examples
  ```
   let value1 = pretty_print_with_symbol("12345.12", ',');
   assert_eq!(value1, "12,345.12");

   let value2 = pretty_print_with_symbol("12.12", ',');
   assert_eq!(value2, "12.12");
  ```
 */
pub fn pretty_print_with_symbol (value: &str, symbol: char) -> String {
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

/** `convert_value_with_rates` 将传入金额根据相关的货币进行汇率转换

  # Examples

  ```
    let value = convert_value_with_rates(0.140449, 1.0, 100);
    assert_eq!(value, "14.04");
  ```  
*/
pub fn convert_value_with_rates(to_rate: f64, from_rate: f64, value: f64) -> String {
  if to_rate == from_rate {
    return format!("{:.2}", value)
  }

  let convert_value = value * to_rate / from_rate;

  format!("{:.2}", convert_value)
}

/** `format` 将传入金额根据相关的货币进行汇率转换，并进行美化展示
 
 # Examples

 ```
  let convert_value = currency_format(219930.00, CommonFormatOption {
    from: String::from("USD"),
    to: String::from("CNY"),
    currency_rates: CurrencyRates{
      usd: 1.0,
      gbp: 0.808686,
      cny: 0.140449
    }
  });

  assert_eq!(convert_value, "￥30,888.95 CNY");
 ```  
 */
pub fn currency_format(value: f64, options: CommonFormatOption) -> String {
  let to = options.to;
  let currency_rates_result = get_convert_rates(&options.from, &to, options.currency_rates);

  let currency_rates = match currency_rates_result  {
    Ok(value) => value,
    Err(message) => panic!("error message:{}", message),
  };

  let convert_value_str = convert_value_with_rates(currency_rates.to, currency_rates.from, value);
  let convert_split_arr: Vec<&str> = convert_value_str.split(".").collect();
  let convert_int_part = convert_split_arr[0];
  let currency_symbol = get_currency_symbol(Some(&to)); 

  let format_str = pretty_print_with_symbol(convert_int_part, ',');

  let mut result = currency_symbol + &format_str + "." + convert_split_arr[1];

  if options.with_currency {
    result = result + " " + &to;
  }


  result
}

#[cfg(test)]
mod unit_test {
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
    assert_eq!(currency_format(100.0, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("GBP"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
        cny: 0.140449
      },
      with_currency: true,
    }), "£80.87 GBP");
  }

  #[test]
  fn pretty_format() {
    assert_eq!(currency_format(219930.00, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("GBP"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
        cny: 0.140449
      },
      with_currency: true,
    }), "£177,854.31 GBP");
  }

  #[test]
  fn format_different_rates() {
    assert_eq!(currency_format(219930.00, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("CNY"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
        cny: 0.140449
      },
      with_currency: true,
    }), "￥30,888.95 CNY");
  }

  #[test]
  #[should_panic]
  fn format_unknown_currency() {
    currency_format(219930.00, CommonFormatOption {
      from: String::from("JPY"),
      to: String::from("CNY"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
        cny: 0.140449
      },
      with_currency: true,
    });
  }

  #[test]
  fn format_without_currency() {
    assert_eq!(currency_format(219930.00, CommonFormatOption {
      from: String::from("USD"),
      to: String::from("CNY"),
      currency_rates: CurrencyRates{
        usd: 1.0,
        gbp: 0.808686,
        cny: 0.140449
      },
      with_currency: false,
    }), "￥30,888.95");
  }

  #[test]
  fn basic_convert_value_with_rates() {
    assert_eq!(convert_value_with_rates(1.0, 1.0, 12.0), "12.00");
  }
}