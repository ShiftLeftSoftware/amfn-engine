//! List of exchange rates.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::{Cell};
use std::cmp::Ordering::Equal;

use super::ElemExchangeRate;
use crate::{ListTrait};

pub struct ListExchangeRate {
    /// The list of exchange rates.
    list_exchange_rate: Vec<ElemExchangeRate>,

    /// The index of the currently selected exchange rate element.
    list_index: Cell<usize>,

    /// If true sort when a exchange rate is added, otherwise do not sort (for bulk adds).
    sort_on_add: bool,

    /// Updated while sort_on_add was false.
    sort_updated: bool,
}

/// List of exchange rates default implementation.

impl Default for ListExchangeRate {
    /// Create a new symbol element.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListExchangeRate::new()
    }
}

/// List of exchange rates list implementation.

impl ListTrait for ListExchangeRate {
    /// Clear all exchange rates from the exchange rate list.

    fn clear(&mut self) {
        self.list_exchange_rate.clear();
        self.list_index.set(usize::MAX);
        self.sort_on_add = true;
        self.sort_updated = false;
    }

    /// Get the count of the exchange rate list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_exchange_rate.len()
    }

    /// Get the index of the selected exchange rate (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
    }

    /// Select a exchange rate based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index value of the exchange rate to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_exchange_rate.len() {
            return false;
        }

        self.set_index(index_param);

        true
    }

    /// Set the list index.
    ///
    /// # Arguments
    ///
    /// * `index_param` - See description.

    fn set_index(&self, index_param: usize) -> bool {
        if index_param >= self.list_exchange_rate.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of exchange rates implementation.

impl ListExchangeRate {
    /// Create and return a new list exchange rate.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new() -> ListExchangeRate {
        ListExchangeRate {
            list_exchange_rate: Vec::new(),
            list_index: Cell::new(usize::MAX),
            sort_on_add: true,
            sort_updated: false,
        }
    }

    /// Add a new exchange rate into the exchange rate list.
    /// The exchange rate is updated if it already exists.
    ///
    /// # Arguments
    ///
    /// * `from_code` - International currency code "from".
    /// * `to_code` - International currency code "to".
    /// * `exchange_rate` - The exchange rate in "from" (unit) / "to" (unit).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn add_exchange_rate(
        &mut self,
        from_code: &str,
        to_code: &str,
        exchange_rate: Decimal,
    ) -> bool {
        if self.get_element_by_name(from_code, to_code) {
            // Check for duplicate name
            match self.list_exchange_rate.get_mut(self.list_index.get()) {
                None => {}
                Some(o) => {
                    o.set_exchange_rate(exchange_rate);
                }
            }

            return true;
        }

        let new_elem_exch: ElemExchangeRate =
            ElemExchangeRate::new(from_code, to_code, exchange_rate);

        self.list_exchange_rate.push(new_elem_exch);
        if self.sort_on_add {
            self.sort();
        }

        match self.list_exchange_rate.iter().position(|e| {
            e.from_code() == from_code
                && e.to_code() == to_code
                && e.exchange_rate() == exchange_rate
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }
        if !self.sort_on_add {
            self.sort_updated = true;
        }

        true
    }

    /// Copy all exchange rates from the exchange rate list
    /// and return a new exchange rate list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy_with_calc_manager(&self) -> ListExchangeRate {
        let mut exch = ListExchangeRate::new();
        let mut index: usize = 0;

        loop {
            if !self.get_element(index) {
                break;
            }

            let from_code = self.from_code();
            let to_code = self.to_code();
            let exchange_rate = self.exchange_rate();

            exch.add_exchange_rate(from_code, to_code, exchange_rate);

            index += 1;
        }

        exch
    }

    /// Copy all exchange rates from the exchange rate list
    /// and return a new exchange rate list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ListExchangeRate {
        self.copy_with_calc_manager()
    }

    /// Convert a value from one currency to another.
    /// Cross rates are used if the exchange rate is unavailable and
    /// the cross rate international currency code is not empty.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert.
    /// * `from_code` - International currency code "from".
    /// * `to_code` - International currency code "to".
    /// * `cross_rate_code` - International currency code used for cross rates.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn convert_currency(
        &self,
        value: Decimal,
        from_code: &str,
        to_code: &str,
        cross_rate_code: &str,
    ) -> Decimal {
        match self.list_exchange_rate.get(self.list_index.get()) {
            None => {
                return dec!(0.0);
            }
            Some(o) => {
                let elem_exchange_rate = o.exchange_rate();

                let mut cross_rate: Decimal = dec!(1.0);
                let exchange_rate: Decimal;
                if !self.get_element_by_name(from_code, to_code) {
                    if !self.get_element_by_name(to_code, from_code) {
                        if cross_rate_code.is_empty() {
                            return dec!(0.0);
                        }
                        if !self.get_element_by_name(from_code, cross_rate_code) {
                            if !self.get_element_by_name(cross_rate_code, from_code) {
                                return dec!(0.0);
                            }
                            exchange_rate = dec!(1.0) / elem_exchange_rate;
                        } else {
                            exchange_rate = elem_exchange_rate;
                        }
                        if !self.get_element_by_name(cross_rate_code, to_code) {
                            if !self.get_element_by_name(to_code, cross_rate_code) {
                                return dec!(0.0);
                            }
                            cross_rate = dec!(1.0) / elem_exchange_rate;
                        } else {
                            cross_rate = elem_exchange_rate;
                        }
                    } else {
                        exchange_rate = dec!(1.0) / elem_exchange_rate;
                    }
                } else {
                    exchange_rate = elem_exchange_rate;
                }
                value * exchange_rate * cross_rate
            }
        }
    }

    /// Get the list of exchange rates.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list(&self) -> &Vec<ElemExchangeRate> {
        &self.list_exchange_rate
    }

    /// Get the mut list of exchange rates.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_mut(&mut self) -> &mut Vec<ElemExchangeRate> {
        &mut self.list_exchange_rate
    }

    /// Get the international currency code "from".
    ///
    /// # Return
    ///
    /// * See description.

    pub fn from_code(&self) -> &str {
        match self.list_exchange_rate.get(self.list_index.get()) {
            None => {
                panic!("Exchange rate list index not set");
            }
            Some(o) => o.from_code(),
        }
    }

    /// Get the international currency code "to".
    ///
    /// # Return
    ///
    /// * See description.

    pub fn to_code(&self) -> &str {
        match self.list_exchange_rate.get(self.list_index.get()) {
            None => {
                panic!("Exchange rate list index not set");
            }
            Some(o) => o.to_code(),
        }
    }

    /// Get the exchange rate in "from" (unit) / "to" (unit).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn exchange_rate(&self) -> Decimal {
        match self.list_exchange_rate.get(self.list_index.get()) {
            None => {
                panic!("Exchange rate list index not set");
            }
            Some(o) => o.exchange_rate(),
        }
    }

    /// Select a exchange rate based upon currency codes "from" and "to".
    ///
    /// # Arguments
    ///
    /// * `from_code` - International currency code "from".
    /// * `to_code` - International currency code "to".
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_name(&self, from_code: &str, to_code: &str) -> bool {
        for (index, elem) in self.list_exchange_rate.iter().enumerate() {
            if from_code == elem.from_code() && to_code == elem.to_code() {
                self.set_index(index);
                return true;
            }
        }
        false
    }

    /// Remove the selected exchange rate from the exchange rate list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_exchange_rate.len() {
            return false;
        }

        self.list_exchange_rate.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }

        true
    }

    /// Set the "from" international currency code.
    ///
    /// # Arguments
    ///
    /// * `from_code_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_from_code(&mut self, from_code_param: &str) -> bool {
        let from_code: String;
        let to_code: String;
        let exchange_rate: Decimal;

        match self.list_exchange_rate.get_mut(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                o.set_from_code(from_code_param);
                from_code = String::from(o.from_code());
                to_code = String::from(o.to_code());
                exchange_rate = o.exchange_rate();
            }
        }

        if self.sort_on_add {
            self.sort();
        }

        match self.list_exchange_rate.iter().position(|e| {
            e.from_code() == from_code
                && e.to_code() == to_code
                && e.exchange_rate() == exchange_rate
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add {
            self.set_sort_updated(true);
        }

        true
    }

    /// Set the "to" international currency code.
    ///
    /// # Arguments
    ///
    /// * `to_code` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_to_code(&mut self, to_code_param: &str) -> bool {
        let from_code: String;
        let to_code: String;
        let exchange_rate: Decimal;

        match self.list_exchange_rate.get_mut(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                o.set_to_code(to_code_param);
                from_code = String::from(o.from_code());
                to_code = String::from(o.to_code());
                exchange_rate = o.exchange_rate();
            }
        }

        if self.sort_on_add {
            self.sort();
        }

        match self.list_exchange_rate.iter().position(|e| {
            e.from_code() == from_code
                && e.to_code() == to_code
                && e.exchange_rate() == exchange_rate
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add {
            self.set_sort_updated(true);
        }

        true
    }

    /// Set the exchange rate.
    ///
    /// # Arguments
    ///
    /// * `exchange_rate_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_exchange_rate(&mut self, exchange_rate_param: Decimal) -> bool {
        match self.list_exchange_rate.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_exchange_rate(exchange_rate_param);
                true
            }
        }
    }

    /// Determines when the exchange rate list is sorted.
    ///
    /// # Arguments
    ///
    /// * `sort_on_add_param` - If true sort when a exchange rate is added, otherwise do not sort (for bulk adds).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_sort_on_add(&mut self, sort_on_add_param: bool) -> bool {
        if self.sort_on_add == sort_on_add_param {
            return false;
        }

        self.sort_on_add = sort_on_add_param;
        if self.sort_on_add && self.sort_updated {
            self.sort();

            match self.list_exchange_rate.get(self.list_index.get()) {
                None => {
                    return false;
                }
                Some(o) => {
                    match self.list_exchange_rate.iter().position(|e| {
                        e.from_code() == o.from_code()
                            && e.to_code() == o.to_code()
                            && e.exchange_rate() == o.exchange_rate()
                    }) {
                        None => {}
                        Some(o2) => {
                            self.list_index.set(o2);
                        }
                    }
                }
            }
        }
        self.sort_updated = false;

        true
    }

    /// Set sort updated.
    ///
    /// # Arguments
    ///
    /// * `sort_updated_param` - If true sort updated otherwise false.

    pub fn set_sort_updated(&mut self, sort_updated_param: bool) -> bool {
        if self.sort_updated == sort_updated_param {
            return false;
        }
        self.sort_updated = sort_updated_param;

        true
    }

    /// Sort the exchange rate list.

    fn sort(&mut self) {
        self.list_exchange_rate
            .sort_by(|a, b| ListExchangeRate::cmp(a, b));
    }

    /// Sort compare function.
    ///
    /// # Arguments
    ///
    /// * `a` - Event element.
    /// * `b` - Event element.
    ///
    /// # Return
    ///
    /// * Sort order.

    fn cmp(a: &ElemExchangeRate, b: &ElemExchangeRate) -> std::cmp::Ordering {
        let result = Ord::cmp(a.from_code(), b.from_code());
        if result != Equal {
            return result;
        }

        let result = Ord::cmp(a.to_code(), b.to_code());
        if result != Equal {
            return result;
        }

        Equal
    }
}
