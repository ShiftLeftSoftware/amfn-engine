//! The definition of a locale.
// Copyright (c) 2021 ShiftLeft Software
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::collections::HashMap;

pub struct ElemLocale {

    /// ISO language code (ISO 639)_ISO country code (ISO 3166). 
    locale_str: String,

    /// ISO currency code (ISO 4217). 
    currency_code: String,

    /// Currency decimal digits. 
    decimal_digits: usize,

    /// Date. 
    date_regex: String,
    date_replace: String,

    /// Integer. 
    integer_regex: String,
    integer_replace: String,

    /// Decimal. 
    decimal_regex: String,
    decimal_replace: String,

    /// Currency (Decimal). 
    currency_regex: String,
    currency_replace: String,

    /// Resources. 
    resources: HashMap<String, String>
}

/// The locale implementation.

impl ElemLocale {

  /// Create a new locale element.
  /// 
  /// # Arguments
  ///
  /// * `locale_str_param` - ISO language code (ISO 639)_ISO country code (ISO 3166).
  /// * `currency_code_param` - ISO currency code (ISO 4217).
  /// * `decimal_digits_param` - Currency decimal digits.
  /// * `date_regex_param` - Date regular expression.
  /// * `date_replace_param` - Date replace expression.
  /// * `integer_regex_param` - Integer regular expression.
  /// * `integer_replace_param` - Integer replace expression.
  /// * `decimal_regex_param` - Decimal regular expression.
  /// * `decimal_replace_param` - Decimal replace expression.
  /// * `currency_regex_param` - Currency regular expression.
  /// * `currency_replace_param` - Currency replace expression.
  /// * `resources_param` - Resources hash map.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(
      locale_str_param: &str, currency_code_param: &str, decimal_digits_param: usize,
      date_regex_param: &str, date_replace_param: &str, 
      integer_regex_param: &str, integer_replace_param: &str,
      decimal_regex_param: &str, decimal_replace_param: &str, 
      currency_regex_param: &str, currency_replace_param: &str,
      resources_param: HashMap<String, String>) -> ElemLocale {

    return ElemLocale {
      locale_str: String::from(locale_str_param),
      currency_code: String::from(currency_code_param),
      decimal_digits: decimal_digits_param,
      date_regex: String::from(date_regex_param),
      date_replace: String::from(date_replace_param),
      integer_regex: String::from(integer_regex_param),
      integer_replace: String::from(integer_replace_param),
      decimal_regex: String::from(decimal_regex_param),
      decimal_replace: String::from(decimal_replace_param),
      currency_regex: String::from(currency_regex_param),
      currency_replace: String::from(currency_replace_param),
      resources: resources_param
    }
  }

  /// Get the locale string.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn locale_str(self: &Self) -> &str {

    return self.locale_str.as_str();
  }

  /// Get the currency code.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn currency_code(self: &Self) -> &str {

    return self.currency_code.as_str();
  }

  /// Get the currency decimal digits.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_digits(self: &Self) -> usize {

    return self.decimal_digits;
  }

  /// Get the date regex.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn date_regex(self: &Self) -> &str {

    return self.date_regex.as_str();
  }

  /// Get the date replace.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn date_replace(self: &Self) -> &str {

    return self.date_replace.as_str();
  }

  /// Get the integer regex.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn integer_regex(self: &Self) -> &str {

    return self.integer_regex.as_str();
  }

  /// Get the integer replace.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn integer_replace(self: &Self) -> &str {

    return self.integer_replace.as_str();
  }

  /// Get the decimal regex.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_regex(self: &Self) -> &str {

    return self.decimal_regex.as_str();
  }

  /// Get the decimal replace.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_replace(self: &Self) -> &str {

    return self.decimal_replace.as_str();
  }

  /// Get the currency regex.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn currency_regex(self: &Self) -> &str {

    return self.currency_regex.as_str();
  }

  /// Get the currency replace.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn currency_replace(self: &Self) -> &str {

    return self.currency_replace.as_str();
  }

  /// Get the resources.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn resources(self: &Self) -> &HashMap<String, String> {

    return &self.resources;
  }
  
}