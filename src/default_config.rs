use::std::fmt;

const DEFAULT_CURRENCY: &str = "CNY"; 
const DEFAULT_RATE: f32 = 0.7;
const DEFAULT_DIGIT: u32 = 2;
const DEFAULT_FORMAT: &str = "{{amount}}";
const DEFAULT_FORMAT_SYMBOL: &str = "amount";

pub struct BasicFormatMap {
  pub value: &'static str,
  /** 整数与小数部分的分隔符 */
  pub decimal_symbol: &'static str,
  /** 千分位分隔符 */
  pub group_symbol: &'static str,
  pub format: &'static str
}

pub struct StandardFormatMap {
  pub amount: BasicFormatMap,
  pub amount_no_decimals: BasicFormatMap,
  pub amount_with_comma_separator: BasicFormatMap,
  pub amount_no_decimals_with_comma_separator: BasicFormatMap,
  pub amount_with_apostrophe_separator: BasicFormatMap,
  pub amount_no_decimals_with_space_separator: BasicFormatMap,
  pub amount_with_space_separator: BasicFormatMap
}

#[derive(Debug)]
pub enum SymbolOrderEnum {
  Prefix,
  Suffix
}

pub fn get_default_currency () -> &'static str {
  DEFAULT_CURRENCY
}

pub fn get_default_rate () -> f32 {
  DEFAULT_RATE
}

pub fn get_default_digit () -> u32 {
  DEFAULT_DIGIT
}

pub fn get_default_format () -> &'static str {
  DEFAULT_FORMAT
}

pub fn get_default_format_symbol () -> &'static str {
  DEFAULT_FORMAT_SYMBOL
}

pub fn get_default_standard_format_map () -> StandardFormatMap {
   StandardFormatMap {
    amount: BasicFormatMap {
      value: "amount",
      decimal_symbol: ".",
      group_symbol: ",",
      format: "amount",
    },
    amount_no_decimals: BasicFormatMap{
      value: "amount_no_decimals",
      decimal_symbol: "",
      group_symbol: ",",
      format: "amount_no_decimals",
    },
    amount_with_comma_separator: BasicFormatMap{
      value: "amount_with_comma_separator",
      decimal_symbol: ",",
      group_symbol: ".",
      format: "amount_with_comma_separator",
    },
    amount_no_decimals_with_comma_separator: BasicFormatMap{
      value: "amount_no_decimals_with_comma_separator",
      decimal_symbol: "",
      group_symbol: ".",
      format: "amount_no_decimals_with_comma_separator",
    },
    amount_with_apostrophe_separator: BasicFormatMap{
      value: "amount_with_apostrophe_separator",
      decimal_symbol: ".",
      group_symbol: " ",
      format: "amount_with_apostrophe_separator",
    },
    amount_no_decimals_with_space_separator: BasicFormatMap{
      value: "amount_no_decimals_with_space_separator",
      decimal_symbol: "",
      group_symbol: " ",
      format: "amount_no_decimals_with_space_separator",
    },
    amount_with_space_separator: BasicFormatMap {
      value: "amount_with_space_separator",
      decimal_symbol: ",",
      group_symbol: " ",
      format: "amount_with_space_separator",
    },
  }
}