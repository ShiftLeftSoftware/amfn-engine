//! List of locales.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use regex::Regex;
use rust_decimal::prelude::*;
use std::cell::Cell;
use std::collections::HashMap;

use crate::core::CoreUtility; 
use super::{ElemLocale, ElemLocaleFormat};

pub struct ListLocale {
    list_locale: Vec<ElemLocale>,

    /// Currently selected user locale element.
    list_index_user: Cell<usize>,

    /// Currently selected cashflow locale element.
    list_index_cashflow: Cell<usize>,

    /// Currently selected event locale element.
    list_index_event: Cell<usize>,
}

/// List of locales default implementation.

impl Default for ListLocale {
    /// Create and return a new list of locale elements.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListLocale::new()
    }
}

/// List of locales implementation.

impl ListLocale {
    /// Create and return a new list of locale elements.
    ///
    /// # Return
    ///
    /// * See description.
    pub fn new() -> ListLocale {
        ListLocale {
            list_locale: Vec::new(),
            list_index_user: Cell::new(usize::MAX),
            list_index_cashflow: Cell::new(usize::MAX),
            list_index_event: Cell::new(usize::MAX),
        }
    }

    /// Add a new locale to the locale list.
    ///
    /// # Arguments
    ///
    /// * `locale_str_param` - ISO language code (ISO 639)_ISO country code (ISO 3166).
    /// * `currency_code_param` - ISO currency code (ISO 4217).
    /// * `decimal_digits_param` - Currency decimal digits.
    /// * `date_in_format` - Date in format.
    /// * `date_out_format` - Date out format.
    /// * `resources_param` - Resources hash map.

    pub fn add_locale(
        &mut self,
        locale_str_param: &str,
        currency_code_param: &str,
        decimal_digits_param: usize,
        date_in_format_param: ElemLocaleFormat,
        date_out_format_param: ElemLocaleFormat,
        resources_param: HashMap<String, String>,
    ) {
        let resources = resources_param;

        self.list_locale.push(ElemLocale::new(
            locale_str_param,
            currency_code_param,
            decimal_digits_param,
            date_in_format_param,
            date_out_format_param,
            resources,
        ));
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

    pub fn copy(&self) -> ListLocale {
        let mut locales = ListLocale::new();

        for locale in self.list_locale.iter() {
            locales.add_locale(
                locale.locale_str(),
                locale.currency_code(),
                locale.decimal_digits(),
                locale.format_in().copy(),
                locale.format_out().copy(),
                locale.resources().clone(),
            );
        }

        locales
    }

    /// Get the list of locales.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list(&self) -> &Vec<ElemLocale> {
        &self.list_locale
    }

    /// Get the mutable list of locales.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_mut(&mut self) -> &mut Vec<ElemLocale> {
        &mut self.list_locale
    }

    /// Clear all locales selects.

    pub fn clear(&self) {
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

    pub fn locale_str(&self, event: bool) -> &str {
        match self.list_locale.get(self.get_locale_index(event)) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o.locale_str(),
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

    pub fn currency_code(&self, event: bool) -> &str {
        match self.list_locale.get(self.get_locale_index(event)) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o.currency_code(),
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

    pub fn decimal_digits(&self, event: bool) -> usize {
        match self.list_locale.get(self.get_locale_index(event)) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o.decimal_digits(),
        }
    }

    /// Get the format in.
    ///
    /// # Arguments
    ///
    /// * `event` - Check event level.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_in(&self, event: bool) -> &ElemLocaleFormat {
        match self.list_locale.get(self.get_locale_index(event)) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o.format_in(),
        }
    }

    /// Get the format out.
    ///
    /// # Arguments
    ///
    /// * `event` - Check event level.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_out(&self, event: bool) -> &ElemLocaleFormat {
        match self.list_locale.get(self.get_locale_index(event)) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o.format_out(),
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

    pub fn resources(&self, event: bool) -> &HashMap<String, String> {
        match self.list_locale.get(self.get_locale_index(event)) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o.resources(),
        }
    }

    /// Get the user locale.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn user_locale(&self) -> &ElemLocale {
        match self.list_locale.get(self.list_index_user.get()) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o,
        }
    }

    /// Get the cashflow locale.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_locale(&self) -> &ElemLocale {
        match self.list_locale.get(self.list_index_cashflow.get()) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o,
        }
    }

    /// Get the event locale.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_locale(&self) -> &ElemLocale {
        match self.list_locale.get(self.list_index_event.get()) {
            None => {
                panic!("Locale list index not set");
            }
            Some(o) => o,
        }
    }

    /// Return the cashflow currency code.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_currency_code(&self) -> &str {
        self.get_locale(false).currency_code()
    }

    /// Return the event currency code.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_currency_code(&self) -> &str {
        self.get_locale(true).currency_code()
    }
    
    /// Format a date and return the internal format.
    ///
    /// # Arguments
    ///
    /// * `display_val` - The display value to parse.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_date_in(&self, display_val: &str) -> String {
        let text: String;

        match Regex::new(self.get_locale(true).format_in().date_regex()) {
            Err(_e) => { return String::from(display_val); }
            Ok(o) => { 
                text = o.replace(display_val,
                    self.get_locale(true).format_in().date_replace()).to_string();
            }
        }

        let dd: Vec<_> = text.split('-').collect();
        if dd.len() != 3 { 
            let now = CoreUtility::date_now();
            return format!("{}-{}-{}", now / 10000, now / 100 % 100, now % 100);
        }

        let mut year = String::from(dd[0]);
        if year.len() < 3 {
            let century = if CoreUtility::parse_integer(year.as_str()) < crate::SERIAL_BASE_CENTURY { "20" } else { "19" };
            year = format!("{}{}", century, self.zerofill(year.as_str(), 2));
        }

        if year.len() != 4 { 
            let now = CoreUtility::date_now();
            return format!("{}-{}-{}", now / 10000, now / 100 % 100, now % 100);
        }

        format!("{}-{}-{}", year , self.zerofill(dd[1], 2), self.zerofill(dd[2], 2))
    }

    /// Format an integer and return the internal format.
    ///
    /// # Arguments
    ///
    /// * `display_val` - The display value to parse.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_integer_in(&self, display_val: &str) -> String {
        match Regex::new(self.get_locale(true).format_in().integer_regex()) {
            Err(_e) => String::from(display_val),
            Ok(o) => o
                .replace(display_val,
                self.get_locale(true).format_in().integer_replace()).to_string()
        }
    }

    /// Format a decimal and return the internal format.
    ///
    /// # Arguments
    ///
    /// * `display_val` - The display value to parse.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_decimal_in(&self, display_val: &str) -> String {
        match Regex::new(self.get_locale(true).format_in().decimal_regex()) {
            Err(_e) => String::from(display_val),
            Ok(o) => o
                .replace(display_val,
                self.get_locale(true).format_in().decimal_replace()).to_string()
        }
    }

    /// Format a currency and return the internal format.
    ///
    /// # Arguments
    ///
    /// * `display_val` - The display value to parse.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format_currency_in(&self, display_val: &str) -> String {
        match Regex::new(self.get_locale(true).format_in().currency_regex()) {
            Err(_e) => String::from(display_val),
            Ok(o) => o
                .replace(display_val,
                self.get_locale(true).format_in().currency_replace()).to_string()
        }
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

    pub fn format_date_out(&self, val: usize) -> String {
        let text = format!("{:04}-{:02}-{:02}", val / 10000, val / 100 % 100, val % 100);

        match Regex::new(self.get_locale(true).format_out().date_regex()) {
            Err(_e) => String::from(text.as_str()),
            Ok(o) => o
                .replace(
                    text.as_str(),
                    self.get_locale(true).format_out().date_replace(),
                )
                .to_string(),
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

    pub fn format_integeri_out(&self, val: i32) -> String {
        let text = val.to_string();

        match Regex::new(self.get_locale(true).format_out().integer_regex()) {
            Err(_e) => String::from(text.as_str()),
            Ok(o) => o
                .replace(
                    text.as_str(),
                    self.get_locale(true).format_out().integer_replace(),
                )
                .to_string(),
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

    pub fn format_integer_out(&self, val: usize) -> String {
        self.format_integeri_out(val as i32)
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

    pub fn format_decimal_out(&self, val: Decimal) -> String {
        let text = CoreUtility::util_round(val, crate::MAXIMUM_DISPLAY_DECIMAL_DIGITS).to_string();

        match Regex::new(self.get_locale(true).format_out().decimal_regex()) {
            Err(_e) => String::from(text.as_str()),
            Ok(o) => o
                .replace(
                    text.as_str(),
                    self.get_locale(true).format_out().decimal_replace(),
                )
                .to_string(),
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

    pub fn format_currency_out(&self, val: Decimal, decimal_digits: usize) -> String {
        let mut text = CoreUtility::util_round(val, decimal_digits).to_string();

        let tokens: Vec<_> = text.split('.').collect();
        let mut fract = String::from("");
        if tokens.len() > 1 {
            fract = String::from(tokens[1]);
            fract.truncate(decimal_digits);
            text = format!("{}.{}", tokens[0], fract);
        }

        let mut zeros = decimal_digits - fract.len(); 
        while zeros > 0 {
            text.push('0');
            zeros -= 1;
        }

        match Regex::new(self.get_locale(true).format_out().currency_regex()) {
            Err(_e) => String::from(text.as_str()),
            Ok(o) => o
                .replace(
                    text.as_str(),
                    self.get_locale(true).format_out().currency_replace(),
                )
                .to_string(),
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

    pub fn get_locale_index(&self, event: bool) -> usize {
        if event && self.list_index_event.get() != usize::MAX {
            return self.list_index_event.get();
        }
        if self.list_index_cashflow.get() != usize::MAX {
            return self.list_index_cashflow.get();
        }

        self.list_index_user.get()
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

    pub fn get_locale(&self, event: bool) -> &ElemLocale {
        if event && self.list_index_event.get() != usize::MAX {
            return self.event_locale();
        }
        if self.list_index_cashflow.get() != usize::MAX {
            return self.cashflow_locale();
        }

        self.user_locale()
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

    pub fn get_locale_str(&self, event: bool) -> &str {
        if event && self.list_index_event.get() != usize::MAX {
            return self.event_locale().locale_str();
        }
        if self.list_index_cashflow.get() != usize::MAX {
            return self.cashflow_locale().locale_str();
        }

        self.user_locale().locale_str()
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
    #[allow(clippy::manual_unwrap_or)]

    pub fn get_resource(&self, key: &str) -> &str {
        match self.get_locale(true).resources().get(key) {
            None => "",
            Some(o) => o,
        }
    }

    /// Select a user locale parameter.
    ///
    /// # Arguments
    ///
    /// * `locale_str_param` - Locale string to select.

    pub fn select_user_locale(&self, locale_str_param: &str) {
        for (index, loc) in self.list_locale.iter().enumerate() {
            if loc.locale_str() == locale_str_param {
                self.list_index_user.set(index);
                return;
            }
        }

        self.list_index_user.set(usize::MAX);
    }

    /// Select a cashflow locale parameter.
    ///
    /// # Arguments
    ///
    /// * `locale_str_param` - Locale string to select.

    pub fn select_cashflow_locale(&self, locale_str_param: &str) {
        for (index, loc) in self.list_locale.iter().enumerate() {
            if loc.locale_str() == locale_str_param {
                self.list_index_cashflow.set(index);
                return;
            }
        }

        self.list_index_cashflow.set(usize::MAX);
    }

    /// Select an event locale parameter.
    ///
    /// # Arguments
    ///
    /// * `locale_str_param` - Locale string to select.

    pub fn select_event_locale(&self, locale_str_param: &str) {
        for (index, loc) in self.list_locale.iter().enumerate() {
            if loc.locale_str() == locale_str_param {
                self.list_index_event.set(index);
                return;
            }
        }

        self.list_index_event.set(usize::MAX);
    }

    /// Fill leading zeros for a number.
    ///
    /// # Arguments
    ///
    /// * `val` - The number to zero fill.
    /// * `size` - The number of digits.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn zerofill(&self, val: &str, size: usize) -> String {

        let mut text = String::from("");
        while val.len() < size {
            text.push('0');
        }

        format!("{}{}", text, val)
    }
}
