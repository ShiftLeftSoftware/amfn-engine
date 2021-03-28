//! The definition of a locale.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;

/// The locale format definition.

pub struct ElemLocaleFormat {
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
}

/// The locale format implementation.

impl ElemLocaleFormat {
    /// Create a new locale format element.
    ///
    /// # Arguments
    ///
    /// * `date_regex_param` - Date regular expression.
    /// * `date_replace_param` - Date replace expression.
    /// * `integer_regex_param` - Integer regular expression.
    /// * `integer_replace_param` - Integer replace expression.
    /// * `decimal_regex_param` - Decimal regular expression.
    /// * `decimal_replace_param` - Decimal replace expression.
    /// * `currency_regex_param` - Currency regular expression.
    /// * `currency_replace_param` - Currency replace expression.
    ///
    /// # Return
    ///
    /// * See description.
    #[allow(clippy::too_many_arguments)]

    pub fn new(
        date_regex_param: &str,
        date_replace_param: &str,
        integer_regex_param: &str,
        integer_replace_param: &str,
        decimal_regex_param: &str,
        decimal_replace_param: &str,
        currency_regex_param: &str,
        currency_replace_param: &str,
    ) -> ElemLocaleFormat {
        ElemLocaleFormat {
            date_regex: String::from(date_regex_param),
            date_replace: String::from(date_replace_param),
            integer_regex: String::from(integer_regex_param),
            integer_replace: String::from(integer_replace_param),
            decimal_regex: String::from(decimal_regex_param),
            decimal_replace: String::from(decimal_replace_param),
            currency_regex: String::from(currency_regex_param),
            currency_replace: String::from(currency_replace_param),
        }
    }

    /// Copy and return a new locale format element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ElemLocaleFormat {
        ElemLocaleFormat::new(
            self.date_regex(),
            self.date_replace(),
            self.integer_regex(),
            self.integer_replace(),
            self.decimal_regex(),
            self.decimal_replace(),
            self.currency_regex(),
            self.currency_replace(),
        )
    }

    /// Get the date regex.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn date_regex(&self) -> &str {
        self.date_regex.as_str()
    }

    /// Get the date replace.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn date_replace(&self) -> &str {
        self.date_replace.as_str()
    }

    /// Get the integer regex.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn integer_regex(&self) -> &str {
        self.integer_regex.as_str()
    }

    /// Get the integer replace.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn integer_replace(&self) -> &str {
        self.integer_replace.as_str()
    }

    /// Get the decimal regex.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_regex(&self) -> &str {
        self.decimal_regex.as_str()
    }

    /// Get the decimal replace.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_replace(&self) -> &str {
        self.decimal_replace.as_str()
    }

    /// Get the currency regex.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn currency_regex(&self) -> &str {
        self.currency_regex.as_str()
    }

    /// Get the currency replace.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn currency_replace(&self) -> &str {
        self.currency_replace.as_str()
    }
}

/// The locale definition.

pub struct ElemLocale {
    /// ISO language code (ISO 639)_ISO country code (ISO 3166).
    locale_str: String,

    /// ISO currency code (ISO 4217).
    currency_code: String,

    /// Currency decimal digits.
    decimal_digits: usize,

    /// Format in.
    format_in: ElemLocaleFormat,

    /// Format out.
    format_out: ElemLocaleFormat,

    /// Resources.
    resources: HashMap<String, String>,
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
    /// * `format_in_param` - Format in.
    /// * `format_out_param` - Format out.
    /// * `resources_param` - Resources hash map.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        locale_str_param: &str,
        currency_code_param: &str,
        decimal_digits_param: usize,
        format_in_param: ElemLocaleFormat,
        format_out_param: ElemLocaleFormat,
        resources_param: HashMap<String, String>,
    ) -> ElemLocale {
        ElemLocale {
            locale_str: String::from(locale_str_param),
            currency_code: String::from(currency_code_param),
            decimal_digits: decimal_digits_param,
            format_in: format_in_param,
            format_out: format_out_param,
            resources: resources_param,
        }
    }

    /// Get the locale string.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn locale_str(&self) -> &str {
        self.locale_str.as_str()
    }

    /// Get the currency code.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn currency_code(&self) -> &str {
        self.currency_code.as_str()
    }

    /// Get the currency decimal digits.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_digits(&self) -> usize {
        self.decimal_digits
    }

    /// Get the format in.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_in(&self) -> &ElemLocaleFormat {
        &self.format_in
    }

    /// Get the format out.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_out(&self) -> &ElemLocaleFormat {
        &self.format_out
    }

    /// Get the resources.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn resources(&self) -> &HashMap<String, String> {
        &self.resources
    }
}
