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

pub struct ConvertOption {
  from: &'static str,
  to: &'static str,
  currency_rates: CurrencyRate
}

pub struct  CommonFormatOption {
  digits: u8,
  code: &'static str
}

pub fn convert_calc(value: f32, from: &'static str, to: &'static str, rates: CurrencyRate) -> f32 {
  if from == to {
    return value
  }

  return (value * (rates.usd)) / (rates.gbp)
}

fn parse_number_by_format_str () {}

pub struct ParseResult {
  integer_part: u32,
  fraction_part: &'static str
}

fn precision_f32(x: f32, decimals: u32) -> f32 {
  if x == 0. || decimals == 0 {
    0.
  } else {
    let shift = decimals as i32 - x.abs().log10().ceil() as i32;
    let shift_factor = 10_f32.powi(shift);

    (x * shift_factor).round() / shift_factor
  }
}

pub fn get_frac(f: f32) -> u32 {
  let eps = 1e-4;
    let mut f = f.abs().fract();
    if f == 0.0 { return 0; }
    
    while (f.round() - f).abs() <= eps {
        f = 10.0 * f;
    }
    
    while (f.round() - f).abs() > eps {
        f = 10.0 * f;
    }
    
    return f.round() as u32;
}

pub fn parse_integer_and_fraction_part(value: f32, precision: u32, decimal_symbol: &'static str) -> ParseResult {
  let precision_value = precision_f32(value, precision);
  let mut integer_part: u32;
  let mut fraction_part: &'static str = "";

  if precision == 0 || decimal_symbol.is_empty() {
    fraction_part = "";
    integer_part = value as u32;
  } else {
    integer_part = precision_value.floor() as u32;
    // let mut fraction_part = precision_f32(value - integer_part, precision).to_string();
  }

  return ParseResult {
    integer_part: integer_part,
    fraction_part: fraction_part
  }
} 

// fn common_format(value: f32, with_currency: bool, options: CommonFormatOption) -> &'static str {
//   let code = options.code;
//   let precision = options.digits;
//   let standard_format_map = get_default_standard_format_map();
//   let group_symbol = standard_format_map.amount.group_symbol;
//   let decimal_symbol = standard_format_map.amount.decimal_symbol;

//   let is_negative = value < 0.0;

// }

// fn basic_convert(value: f32, with_currency: bool, option: ConvertOption) -> &'static str {
//   let to = option.to;
//   let from = option.from;
//   let rates = option.currency_rates; 

//   let mut format_fn: fn(value: f32, options: CommonFormatOption) -> &'static str;

//   if with_currency {
//     format_fn = format
//   } else {
//     format_fn = format_without_currency
//   }

//   let calc_value = convert_calc(value, to, from, rates);


//   return format_fn(value, CommonFormatOption{
//     code: to,
//     digits: 2
//   })
// }

// pub fn format(value: f32, option: CommonFormatOption) -> &'static str {
//   common_format(value, true, option)
// }

// pub fn format_without_currency(value: f32, option: CommonFormatOption) -> &'static str {
//   common_format(value, false, option)
// }

// pub fn convert_format (value: f32, option: ConvertOption) -> &'static str {

// }

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
    // assert_eq!(format(100.0, CommonFormatOption {
    //   from: "USD",
    //   to: "GBP",
    //   currency_rates: CurrencyRate{
    //     usd: 1.0,
    //     gbp: 0.808686,
    //     cny: None,
    //     jpy: None,
    //     my: None
    //   }
    // }), "£1,865.07 GBP");
  }
}