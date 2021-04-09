//! List of amortization elements in the cashflow.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering::Equal;

use super::{ElemAmortization, ElemExtension, ListDescriptor, ListParameter};
use crate::{ExtensionTrait, ListTrait};

pub struct ListAmortization {
    /// The list of amortization elements.
    list_am: Vec<ElemAmortization>,

    /// The index of the currently selected amortization element.
    list_index: Cell<usize>,
}

/// List of amortization elements list implementation.

impl ListTrait for ListAmortization {
    /// Clear all elements from the amortization list.

    fn clear(&mut self) {
        self.list_am.clear();
        self.list_index.set(usize::MAX);
    }

    /// Get the count of the amortization list.
    ///
    /// # Return
    ///
    /// * See description.
    fn count(&self) -> usize {
        self.list_am.len()
    }

    /// Get the index of the selected amortization element (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.
    fn index(&self) -> usize {
        self.list_index.get()
    }
    /// Select an amortization element based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - Index value of the amortization element to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_am.len() {
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
        if index_param >= self.list_am.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of amortization elements default implementation.

impl Default for ListAmortization {
    /// Create and return a new list of amortization elements.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListAmortization::new()
    }
}

/// List of amortization elements implementation.

impl ListAmortization {
    /// Create and return a new list of amortization elements.
    ///
    /// # Return
    ///
    /// * See description.
    pub fn new() -> ListAmortization {
        ListAmortization {
            list_am: Vec::new(),
            list_index: Cell::new(usize::MAX),
        }
    }

    /// Add a new amortization element into the amortization list.
    ///
    /// # Arguments
    ///
    /// * `event_type_param` - Event type.
    /// * `orig_date_param` - Original date.
    /// * `event_date_param` - Event date.
    /// * `sort_order_param` - Sort order.
    /// * `value_param` - Value parameter.
    /// * `value_expr_param` - Value expression parameter.
    /// * `periods_param` - Periods parameter.
    /// * `intervals_param` - Intervals parameter.
    /// * `frequency_param` - Frequency parameter.
    /// * `principal_decrease_param` - Principal decrease.
    /// * `principal_increase_param` - Principal increase.
    /// * `list_event_index_param` - List event index.
    /// * `event_sequence_param` - Event sequence.
    /// * `stat_sequence_param` - Event stat sequence.
    /// * `extension_param` - Extension (current value, interest change,
    ///   principal change, statistic value).
    /// * `list_parameter_param` - List parameter.
    /// * `list_descriptor_param` - List descriptor.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.
    #[allow(clippy::too_many_arguments)]

    pub fn add_amortization(
        &mut self,
        event_type_param: &str,
        orig_date_param: usize,
        event_date_param: usize,
        sort_order_param: usize,
        value_param: Decimal,
        value_expr_param: &str,
        periods_param: usize,
        intervals_param: usize,
        frequency_param: crate::FrequencyType,
        principal_decrease_param: Decimal,
        principal_increase_param: Decimal,
        list_event_index_param: usize,
        event_sequence_param: usize,
        stat_sequence_param: usize,
        elem_extension_param: ElemExtension,
        list_parameter_param: Option<ListParameter>,
        list_descriptor_param: Option<ListDescriptor>,
    ) -> bool {
        self.add_amortization_ex(
            event_type_param,
            orig_date_param,
            event_date_param,
            sort_order_param,
            value_param,
            value_expr_param,
            periods_param,
            intervals_param,
            frequency_param,
            principal_decrease_param,
            principal_increase_param,
            dec!(0.0),
            dec!(0.0),
            dec!(0.0),
            dec!(0.0),
            dec!(0.0),
            dec!(0.0),
            list_event_index_param,
            event_sequence_param,
            stat_sequence_param,
            elem_extension_param,
            list_parameter_param,
            list_descriptor_param,
        )
    }

    /// Add a new amortization element into the amortization list.
    ///
    /// # Arguments
    ///
    /// * `event_type_param` - Event type.
    /// * `orig_date_param` - Original date.
    /// * `event_date_param` - Event date.
    /// * `sort_order_param` - Sort order.
    /// * `value_param` - Value parameter.
    /// * `value_expr_param` - Value expression parameter.
    /// * `periods_param` - Periods parameter.
    /// * `intervals_param` - Intervals parameter.
    /// * `frequency_param` - Frequency parameter.
    /// * `principal_decrease_param` - Principal decrease.
    /// * `principal_increase_param` - Principal increase.
    /// * `interest_param` - Compounded interest.
    /// * `sl_interest_param` - Straight-line interest.
    /// * `value_to_interest_param` - Value to interest.
    /// * `value_to_principal_param` - Value to principal.
    /// * `acc_balance_param` - Accrued interest balance at start of event.
    /// * `balance_param` - Beginning active balance at start of event.
    /// * `list_event_index_param` - List event index.
    /// * `event_sequence_param` - Event sequence.
    /// * `stat_sequence_param` - Event stat sequence.
    /// * `extension_param` - Extension (current value, interest change, principal change,
    ///   statistic value).
    /// * `list_parameter_param` - List parameter.
    /// * `list_descriptor_param` - List descriptor.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.
    #[allow(clippy::too_many_arguments)]

    pub fn add_amortization_ex(
        &mut self,
        event_type_param: &str,
        orig_date_param: usize,
        event_date_param: usize,
        sort_order_param: usize,
        value_param: Decimal,
        value_expr_param: &str,
        periods_param: usize,
        intervals_param: usize,
        frequency_param: crate::FrequencyType,
        principal_decrease_param: Decimal,
        principal_increase_param: Decimal,
        interest_param: Decimal,
        sl_interest_param: Decimal,
        value_to_interest_param: Decimal,
        value_to_principal_param: Decimal,
        acc_balance_param: Decimal,
        balance_param: Decimal,
        list_event_index_param: usize,
        event_sequence_param: usize,
        stat_sequence_param: usize,
        elem_extension_param: ElemExtension,
        list_parameter_param: Option<ListParameter>,
        list_descriptor_param: Option<ListDescriptor>,
    ) -> bool {
        let new_elem: ElemAmortization = ElemAmortization::new(
            event_type_param,
            orig_date_param,
            event_date_param,
            sort_order_param,
            value_param,
            value_param,
            value_expr_param,
            periods_param,
            intervals_param,
            frequency_param,
            principal_decrease_param,
            principal_increase_param,
            interest_param,
            sl_interest_param,
            value_to_interest_param,
            value_to_principal_param,
            acc_balance_param,
            balance_param,
            list_event_index_param,
            event_sequence_param,
            stat_sequence_param,
            elem_extension_param,
            list_parameter_param,
            list_descriptor_param,
        );

        let new_index: usize = self.list_am.len();
        self.list_am.push(new_elem);

        self.list_index.set(new_index);

        true
    }

    /// Copy this list of amortization elements and return a new list of amortization elements.
    ///
    /// # Arguments
    ///
    /// * `updating_json` - Updating from json.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self, updating_json: bool) -> ListAmortization {
        let mut list_am = ListAmortization::new();

        for elem in self.list_am.iter() {
            let new_extension = elem.elem_extension().copy();

            let mut list_parameter_opt: Option<ListParameter> = None;
            let mut list_descriptor_opt: Option<ListDescriptor> = None;

            match elem.list_parameter().as_ref() {
                None => {}
                Some(o) => {
                    list_parameter_opt = Option::from(o.copy(updating_json));
                }
            }

            match elem.list_descriptor().as_ref() {
                None => {}
                Some(o) => {
                    list_descriptor_opt = Option::from(o.copy(false, updating_json));
                }
            }

            list_am.add_amortization_ex(
                elem.event_type(),
                elem.orig_date(),
                elem.event_date(),
                elem.sort_order(),
                elem.value(),
                elem.value_expr(),
                elem.periods(),
                elem.intervals(),
                elem.frequency(),
                elem.principal_decrease(),
                elem.principal_increase(),
                elem.interest(),
                elem.sl_interest(),
                elem.value_to_interest(),
                elem.value_to_principal(),
                elem.acc_balance(),
                elem.balance(),
                elem.list_event_index(),
                elem.event_sequence(),
                elem.stat_sequence(),
                new_extension,
                list_parameter_opt,
                list_descriptor_opt,
            );
        }

        list_am
    }

    /// Get the type of the amortization element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_type(&self) -> crate::ExtensionType {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.elem_type(),
        }
    }

    /// Get the event type information.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_type(&self) -> &str {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.event_type(),
        }
    }

    /// Get the date of the original event in YYYYMMDD format.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn orig_date(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.orig_date(),
        }
    }

    /// Get the date of the amortization element in YYYYMMDD format (major sort key).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_date(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.event_date(),
        }
    }

    /// Get the sort order within the amortization element date (minor sort key).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sort_order(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.sort_order(),
        }
    }

    /// Get the value of the original event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn orig_value(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.orig_value(),
        }
    }

    /// Get the constant value or the result of an expression.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.value(),
        }
    }

    /// Get the optional value expression evaluated when the amortization list is balanced.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_expr(&self) -> &str {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.value_expr(),
        }
    }

    /// Get the periods of the amortization element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn periods(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.periods(),
        }
    }

    /// Get the optional intervals of frequency between periods.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn intervals(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.intervals(),
        }
    }

    /// Get the frequency of the amortization element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn frequency(&self) -> crate::FrequencyType {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.frequency(),
        }
    }

    /// Get the principal decrease for period.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn principal_decrease(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.principal_decrease(),
        }
    }

    /// Get the principal increase for period.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn principal_increase(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.principal_increase(),
        }
    }

    /// Get the compounded interest for period.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn interest(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.interest(),
        }
    }

    /// Get the straight-line interest for period.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sl_interest(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.sl_interest(),
        }
    }

    /// Get the value to interest for period.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_to_interest(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.value_to_interest(),
        }
    }

    /// Get the value to principal for period.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_to_principal(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.value_to_principal(),
        }
    }

    /// Get the accrued interest balance.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn acc_balance(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.acc_balance(),
        }
    }

    /// Get the active balance.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn balance(&self) -> Decimal {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.balance(),
        }
    }

    /// Get the index of the original event within the event list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_index(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.list_event_index(),
        }
    }

    /// Get the sequence number within the event starting from 1.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_sequence(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.event_sequence(),
        }
    }

    /// Get the sequence number of principal change with statistics set or 0 (if not applicable).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn stat_sequence(&self) -> usize {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.stat_sequence(),
        }
    }

    /// Get the extension.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_extension(&self) -> &ElemExtension {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.elem_extension(),
        }
    }

    /// Get the amortization parameter list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_parameter(&self) -> Option<&ListParameter> {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.list_parameter(),
        }
    }

    /// Get the amortization descriptor list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_descriptor(&self) -> Option<&ListDescriptor> {
        match self.list_am.get(self.list_index.get()) {
            None => {
                panic!("Amortization list index not set");
            }
            Some(o) => o.list_descriptor(),
        }
    }

    /// Remove the selected event from the amortization list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_am.len() {
            return false;
        }

        self.list_am.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }
        true
    }

    /// Set the value of all amortization elements having a specified original event index.
    ///
    /// # Arguments
    ///
    /// * `index_param` - Original event index.
    /// * `value_param` - New value.

    pub fn set_all_index_values(&mut self, event_index_param: usize, value_param: Decimal) {
        for elem in self.list_am.iter_mut() {
            if event_index_param == elem.list_event_index() {
                elem.set_orig_value(value_param);
                elem.set_value(value_param);
            }
        }
    }

    /// Set the value of all amortization interest events.
    ///
    /// # Arguments
    ///
    /// * `value_param` - New value.

    pub fn set_all_interest_values(&mut self, value_param: Decimal) {
        for elem in self.list_am.iter_mut() {
            if elem.elem_type() == crate::ExtensionType::InterestChange {
                elem.set_orig_value(value_param);
                elem.set_value(value_param);
                elem.set_value_expr("");
            }
        }
    }

    /// Set the constant value or the result of an expression.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value(&mut self, value_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value(value_param);
                if o.elem_type() != crate::ExtensionType::PrincipalChange {
                    return false;
                }
                match o.elem_extension().pc_type() {
                    crate::PrincipalType::Negative | crate::PrincipalType::Decrease => {
                        self.set_principal_decrease(value_param);
                    }
                    _ => {
                        self.set_principal_increase(value_param);
                    }
                }

                true
            }
        }
    }

    /// Set the principal decrease for period.
    ///
    /// # Arguments
    ///
    /// * `principal_decrease_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_principal_decrease(&mut self, principal_decrease_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_principal_decrease(principal_decrease_param);
                true
            }
        }
    }

    /// Set the principal increase for period.
    ///
    /// # Arguments
    ///
    /// * `principal_increase_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_principal_increase(&mut self, principal_increase_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_principal_increase(principal_increase_param);
                true
            }
        }
    }

    /// Set the compounded interest for period.
    ///
    /// # Arguments
    ///
    /// * `interest_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_interest(&mut self, interest_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_interest(interest_param);
                true
            }
        }
    }

    /// Set the straight-line interest for period.
    ///
    /// # Arguments
    ///
    /// * `sl_interest_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_sl_interest(&mut self, sl_interest_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_sl_interest(sl_interest_param);
                true
            }
        }
    }

    /// Set the value to interest for period.
    ///
    /// # Arguments
    ///
    /// * `value_to_interest_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_to_interest(&mut self, value_to_interest_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value_to_interest(value_to_interest_param);
                true
            }
        }
    }

    /// Set the value to principal for period.
    ///
    /// # Arguments
    ///
    /// * `value_to_principal_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_to_principal(&mut self, value_to_principal_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value_to_principal(value_to_principal_param);
                true
            }
        }
    }

    /// Set the accrued interest balance.
    ///
    /// # Arguments
    ///
    /// * `acc_balance_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_acc_balance(&mut self, acc_balance_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_acc_balance(acc_balance_param);
                true
            }
        }
    }

    /// Set the active balance.
    ///
    /// # Arguments
    ///
    /// * `balance_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_balance(&mut self, balance_param: Decimal) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_balance(balance_param);
                true
            }
        }
    }

    /// Set the sequence number of principal change with statistics set or 0 (if not applicable).
    ///
    /// # Arguments
    ///
    /// * `stat_sequence_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_stat_sequence(&mut self, stat_sequence_param: usize) -> bool {
        match self.list_am.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_stat_sequence(stat_sequence_param);
                true
            }
        }
    }

    /// Sort the amortization list.

    pub fn sort(&mut self) {
        self.list_am.sort_by(|a, b| ListAmortization::cmp(a, b));
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

    fn cmp(a: &ElemAmortization, b: &ElemAmortization) -> std::cmp::Ordering {
        let result = Ord::cmp(&a.event_date(), &b.event_date());
        if result != Equal {
            return result;
        }

        let result = Ord::cmp(&a.sort_order(), &b.sort_order());
        if result != Equal {
            return result;
        }

        Equal
    }
}
