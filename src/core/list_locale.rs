//! List of locales.
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

use std::cell::Cell;
use std::collections::HashMap;
use rust_decimal::prelude::*;
use regex::Regex;

use super::{CoreUtility, ElemLocale};

pub struct ListLocale {

  list_locale: Vec<ElemLocale>,

  /// Currently selected user locale element. 
  list_index_user: Cell<usize>,

  /// Currently selected cashflow locale element. 
  list_index_cashflow: Cell<usize>,

  /// Currently selected event locale element. 
  list_index_event: Cell<usize>,

}

/// List of locales implementation.

impl ListLocale {

  /// Create and return a new list of locale elements.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new() -> ListLocale {
  
    return ListLocale {
      list_locale: Vec::new(),
      list_index_user: Cell::new(usize::MAX),
      list_index_cashflow: Cell::new(usize::MAX),
      list_index_event: Cell::new(usize::MAX)
    }
  }

  /// Add a new locale to the locale list.
  /// 
  /// # Arguments
  ///
  /// * `locale_str_param` - Locale string.
  /// * `currency_code_param` - Currency code.
  /// * `decimal_digits_param` - Decimal digits.
  /// * `date_regex_param` - Date regular expression.
  /// * `date_replace_param` - Date replace expression.
  /// * `integer_regex_param` - Integer regular expression.
  /// * `integer_replace_param` - Integer replace expression.
  /// * `decimal_regex_param` - Decimal regular expression.
  /// * `decimal_replace_param` - Decimal replace expression.
  /// * `currency_regex_param` - Currency regular expression.
  /// * `currency_replace_param` - Currency replace expression.
  /// * `resources_param` - Resources hash map.

  pub fn add_locale(self: &mut Self, locale_str_param: &str, currency_code_param: &str, decimal_digits_param: usize,
      date_regex_param: &str, date_replace_param: &str, integer_regex_param: &str, integer_replace_param: &str, 
      decimal_regex_param: &str, decimal_replace_param: &str, currency_regex_param: &str, 
      currency_replace_param: &str, resources_param: &HashMap<String, String>) -> () {

    let resources = resources_param.clone();

    self.list_locale.push(ElemLocale::new(locale_str_param, currency_code_param, decimal_digits_param,
      date_regex_param, date_replace_param, integer_regex_param, integer_replace_param,
      decimal_regex_param, decimal_replace_param, currency_regex_param, currency_replace_param, resources));
  }

  /// Copy the locale list and return a new locale list.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ListLocale {
    
    let mut locales = ListLocale::new();

    for locale in self.list_locale.iter() {
      let locale_str = locale.locale_str();
      let currency_code = locale.currency_code();
      let decimal_digits = locale.decimal_digits();

      let date_regex = locale.date_regex();
      let date_replace = locale.date_replace();

      let integer_regex = locale.integer_regex();
      let integer_replace = locale.integer_replace();

      let decimal_regex = locale.decimal_regex();
      let decimal_replace = locale.decimal_replace();

      let currency_regex = locale.currency_regex();
      let currency_replace = locale.currency_replace();

      let resources = locale.resources();

      locales.add_locale(locale_str, currency_code, decimal_digits, date_regex, date_replace, 
        integer_regex, integer_replace, decimal_regex, decimal_replace, currency_regex, 
        currency_replace, resources);
    }

    return locales;
  }

  /// Clear all locales selects.

  pub fn clear(self: &mut Self) -> () {
    
    self.list_index_user.set(usize::MAX);
    self.list_index_cashflow.set(usize::MAX);
    self.list_index_event.set(usize::MAX);
  }

  /// Get the locale string.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn locale_str(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.locale_str(); }
    }
  }

  /// Get the currency code.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn currency_code(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.currency_code(); }
    }
  }

  /// Get the decimal digits.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_digits(self: &Self, event: bool) -> usize {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.decimal_digits(); }
    }
  }

  /// Get the date regex.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn date_regex(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.date_regex(); }
    }
  }

  /// Get the date replace.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn date_replace(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.date_replace(); }
    }
  }

  /// Get the integer regex.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn integer_regex(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.integer_regex(); }
    }
  }

  /// Get the integer replace.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn integer_replace(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.integer_replace(); }
    }
  }

  /// Get the decimal regex.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_regex(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.decimal_regex(); }
    }
  }

  /// Get the decimal replace.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_replace(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.decimal_replace(); }
    }
  }

  /// Get the currency regex.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn currency_regex(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.currency_regex(); }
    }
  }

  /// Get the currency replace.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn currency_replace(self: &Self, event: bool) -> &str {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.currency_replace(); }
    }
  }

  /// Get the resources.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn resources(self: &Self, event: bool) -> &HashMap<String, String> {

    match self.list_locale.iter().nth(self.get_locale_index(event)) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o.resources(); }
    }
  }

  /// Get the user locale.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn user_locale(self: &Self) -> &ElemLocale {

    match self.list_locale.iter().nth(self.list_index_user.get()) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o; }
    }
  }

  /// Get the cashflow locale.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn cashflow_locale(self: &Self) -> &ElemLocale {

    match self.list_locale.iter().nth(self.list_index_cashflow.get()) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o; }
    }
  }

  /// Get the event locale.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn event_locale(self: &Self) -> &ElemLocale {

    match self.list_locale.iter().nth(self.list_index_event.get()) {
      None => { panic!("Locale list index not set"); }
      Some(o) => { return o; }
    }
  }

  /// Return the cashflow currency code.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cashflow_currency_code(self: &Self) -> &str {

    return self.get_locale(false).currency_code();
  }

  /// Return the event currency code.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn event_currency_code(self: &Self) -> &str {

    return self.get_locale(true).currency_code();
  }

  /// Format and return a date string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The usize date value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_date(self: &Self, val: usize) -> String {

    let text = format!("{:04}-{:02}-{:02}", val / 10000, val / 100 % 100, val % 100);

    match Regex::new(self.get_locale(true).date_regex()) {
      Err(_e) => { return String::from(text); }
      Ok(o) => { 
        return o.replace(text.as_str(), self.get_locale(true).date_replace()).to_string();
      }
    }
  }

  /// Format and return an integer string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The i32 value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_integeri(self: &Self, val: i32) -> String {

    let text = val.to_string();

    match Regex::new(self.get_locale(true).integer_regex()) {
      Err(_e) => { return String::from(text); }
      Ok(o) => { 
        return o.replace(text.as_str(), self.get_locale(true).integer_replace()).to_string();
      }
    }
  }

  /// Format and return an integer string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The usize value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_integer(self: &Self, val: usize) -> String {

    return self.format_integeri(val as i32);
  }

  /// Format and return a decimal string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The decimal value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_decimal(self: &Self, val: Decimal) -> String {
    
    let text = CoreUtility::util_round(val, crate::MAXIMUM_DISPLAY_DECIMAL_DIGITS).to_string();

    match Regex::new(self.get_locale(true).decimal_regex()) {
      Err(_e) => { return String::from(text); }
      Ok(o) => { 
        return o.replace(text.as_str(), self.get_locale(true).decimal_replace()).to_string();
      }
    }
  }

  /// Format and return a currency string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The decimal value to format.
  /// * `decimal_digits` - The number of decimal digits to round.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_currency(self: &Self, val: Decimal, decimal_digits: usize) -> String {

    let mut text = CoreUtility::util_round(val, decimal_digits).to_string();

    let tokens: Vec<_> = text.split('.').collect();
    if tokens.len() > 1 {
      text = format!("{}.{:0<2}", tokens[0], tokens[1]);
    } else {
      text = format!("{}.00", text);
    }

    match Regex::new(self.get_locale(true).currency_regex()) {
      Err(_e) => { return String::from(text); }
      Ok(o) => { 
        return o.replace(text.as_str(), self.get_locale(true).currency_replace()).to_string();
      }
    }
  }

  /// Get the most relevant locale index.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn get_locale_index(self: &Self, event: bool) -> usize {
    
    if event && self.list_index_event.get() != usize::MAX {
      return self.list_index_event.get();
    }
    
    if self.list_index_cashflow.get() != usize::MAX {
      return self.list_index_cashflow.get();
    }

    return self.list_index_user.get();
  }

  /// Get the most relevant locale.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn get_locale(self: &Self, event: bool) -> &ElemLocale {
    
    if event && self.list_index_event.get() != usize::MAX {
      return self.event_locale();
    }
    
    if self.list_index_cashflow.get() != usize::MAX {
      return self.cashflow_locale();
    }

    return self.user_locale();
  }

  /// Get the most relevant locale string.
  /// 
  /// # Arguments
  ///
  /// * `event` - Check event level.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn get_locale_str(self: &Self, event: bool) -> &str {
    
    if event && self.list_index_event.get() != usize::MAX {
      return self.event_locale().locale_str();
    }
    
    if self.list_index_cashflow.get() != usize::MAX {
      return self.cashflow_locale().locale_str();
    }

    return self.user_locale().locale_str();
  }

  /// Get the resource string for the locale.
  /// 
  /// # Arguments
  ///
  /// * `key` - The resource key.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn get_resource(self: &Self, key: &str) -> &str {

    match self.get_locale(true).resources().get(key) {
      None => { return ""; }
      Some(o) => { return o; }
    }
  }

  /// Select a user locale parameter.
  /// 
  /// # Arguments
  ///
  /// * `locale_str_param` - Locale string to select.
  
  pub fn select_user_locale(self: &mut Self, locale_str_param: &str) -> () {
    let mut index: usize = 0;

    for loc in self.list_locale.iter() {
      if loc.locale_str() == locale_str_param {
        self.list_index_user.set(index);
        return;
      }

      index += 1;
    }

    self.list_index_user.set(usize::MAX);
  }

  /// Select a cashflow locale parameter.
  /// 
  /// # Arguments
  ///
  /// * `locale_str_param` - Locale string to select.
  
  pub fn select_cashflow_locale(self: &mut Self, locale_str_param: &str) -> () {
    let mut index: usize = 0;

    for loc in self.list_locale.iter() {
      if loc.locale_str() == locale_str_param {
        self.list_index_cashflow.set(index);
        return;
      }

      index += 1;
    }

    self.list_index_cashflow.set(usize::MAX);
  }

  /// Select an event locale parameter.
  /// 
  /// # Arguments
  ///
  /// * `locale_str_param` - Locale string to select.
  
  pub fn select_event_locale(self: &mut Self, locale_str_param: &str) -> () {
    let mut index: usize = 0;

    for loc in self.list_locale.iter() {
      if loc.locale_str() == locale_str_param {
        self.list_index_event.set(index);
        return;
      }

      index += 1;
    }

    self.list_index_event.set(usize::MAX);
  }

}