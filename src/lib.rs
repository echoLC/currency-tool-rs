mod default_config;

pub use crate::default_config::*;

#[derive(Debug, Clone)]
pub struct CurrencyRates {
  pub usd: f64,
  pub gbp: f64,
  pub cny: f64
}

#[derive(Debug, Clone)]
pub struct CommonFormatOption {
  pub from: String,
  pub to: String,
  pub currency_rates: CurrencyRates,
  pub with_currency: bool,
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
   use currency_tool_rs::{pretty_print_with_symbol};

   let value1 = pretty_print_with_symbol("12345", ',');
   assert_eq!(value1, "12,345");

   let value2 = pretty_print_with_symbol("12", ',');
   assert_eq!(value2, "12");
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
    use currency_tool_rs::{convert_value_with_rates};

    let value = convert_value_with_rates(0.140449, 1.0, 100.0);
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
  use currency_tool_rs::{currency_format, CommonFormatOption, CurrencyRates};

  let convert_value = currency_format(219930.00, CommonFormatOption {
    from: String::from("USD"),
    to: String::from("CNY"),
    currency_rates: CurrencyRates{
      usd: 1.0,
      gbp: 0.808686,
      cny: 0.140449
    },
    with_currency: true
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

/** `f64_to_int` 将浮点数包括小数部分转换成整数
 
 # Examples
  ```
  use currency_tool_rs::{f64_to_int};

  assert_eq!(f64_to_int(12.12, 2), 1212);
  ```
 */
pub fn f64_to_int (value: f64, digits: u32) -> i64 {
  let base: i32 = 10;
  (value * base.pow(digits) as f64).round() as i64
}

pub struct FloatInfo {
  pub integer: i64,
  pub fraction: Option<i64>
}

/** `get_f64_integer_and_fraction` 获取浮点数的小数和整数部分并返回
 
 # Examples
  ```
  use currency_tool_rs::{get_f64_integer_and_fraction, FloatInfo};

  let result = get_f64_integer_and_fraction(12.12, 2);

  assert_eq!(result.integer, 12);
  assert_eq!(result.fraction, Some(12));
  ```
 */
pub fn get_f64_integer_and_fraction (value: f64, precision: usize) -> FloatInfo {
  if precision == 0 {
    return FloatInfo{
      integer: value.round() as i64,
      fraction: None
    }
  }

  let eps = 1e-4;
  let res: f64 = format!("{:.*}", precision, value).parse().unwrap();
  let mut f = res.abs().fract();

  while (f.round() - f).abs() <= eps {
    f = 10.0 * f;
  }

  while (f.round() - f).abs() > eps {
    f = 10.0 * f;    
  }

  FloatInfo{
    integer: value.round() as i64,
    fraction: Some(f.round() as i64)
  }
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

  #[test]
  fn basic_f64_to_int() {
    assert_eq!(f64_to_int(12.12, 2), 1212);
  }

  #[test]
  fn multi_decimals_f64_to_int() {
    assert_eq!(f64_to_int(12.123, 3), 12123);
  }

  #[test]
  fn basic_get_f64_integer_and_fraction() {
    let result = get_f64_integer_and_fraction(12.12, 2);

    assert_eq!(result.integer, 12);
    assert_eq!(result.fraction, Some(12));
  }

  #[test]
  fn more_decimals_get_f64_integer_and_fraction() {
    let result = get_f64_integer_and_fraction(12.123, 2);
    let result2 = get_f64_integer_and_fraction(12.123, 3);
    let result3 = get_f64_integer_and_fraction(12.12345678, 5);

    assert_eq!(result.integer, 12);
    assert_eq!(result.fraction, Some(12));
    assert_eq!(result2.integer, 12);
    assert_eq!(result3.fraction, Some(12346));
  }

  #[test]
  fn no_precision_get_f64_integer_and_fraction() {
    let result = get_f64_integer_and_fraction(12.123, 0);

    assert_eq!(result.integer, 12);
    assert_eq!(result.fraction, None);
  }
}