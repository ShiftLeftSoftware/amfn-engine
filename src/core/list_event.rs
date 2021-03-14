//! List of events comprising a cashflow.
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

use super::{
    ElemCurrentValue, ElemEvent, ElemExtension, ElemInterestChange,
    ElemPrincipalChange, ElemStatisticValue, ListDescriptor, ListParameter,
};
use crate::{ExtensionTrait, ListTrait};

pub struct ListEvent {
    /// The list of events.
    list_event: Vec<ElemEvent>,
    /// The index of the currently selected event element.
    list_index: Cell<usize>,
    /// If true, sort when an event is added otherwise do not sort (for bulk adds).
    sort_on_add: Cell<bool>,
    /// Updated while sort_on_add was false.
    sort_updated: Cell<bool>,
    /// Originated from cashflow.
    cashflow: bool,
}

/// List of events list implementation.

impl ListTrait for ListEvent {
    /// Clear all events from the event list.

    fn clear(&mut self) {
        self.list_event.clear();
        self.list_index.set(usize::MAX);
        self.sort_on_add.set(true);
        self.sort_updated.set(false);
    }

    /// Get the count of the event list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_event.len()
    }

    /// Get the index of the selected event (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
    }

    /// Select an event based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index value of the event to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_event.len() {
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
        if index_param >= self.list_event.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of events implementation.

impl ListEvent {
    /// Create and return a new list event element.
    ///
    /// # Arguments
    ///
    /// * `cashflow_param` - Originated from cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(cashflow_param: bool) -> ListEvent {
        ListEvent {
            list_event: Vec::new(),
            list_index: Cell::new(usize::MAX),
            sort_on_add: Cell::new(true),
            sort_updated: Cell::new(false),
            cashflow: cashflow_param,
        }
    }

    /// Add a new event into the event list.
    ///
    /// # Arguments
    ///
    /// * `event_type_param` - Event type.
    /// * `event_date_param` - Event date.
    /// * `date_expr_param` - Date expression.
    /// * `sort_order_param` - Sort order.
    /// * `value_param` - Value parameter.
    /// * `value_expr_param` - Value expression parameter.
    /// * `value_expr_balance_param` - Value expression balance.
    /// * `periods_param` - Periods parameter.
    /// * `periods_expr_param` - Periods expression.
    /// * `skip_mask_len_param` - Skip mask length.
    /// * `skip_mask_param` - Skip mask.
    /// * `intervals_param` - Intervals parameter.
    /// * `frequency_param` - Frequency parameter.
    /// * `extension_param` - Extension (current value, interest change, principal change, statistic value).
    /// * `list_parameter_param` - List parameter.
    /// * `list_descriptor_param` - List descriptor.
    /// * `event_name_param` - Optional event name.
    /// * `next_name_param` - Optional next event name.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.
    #[allow(clippy::too_many_arguments)]

    pub fn add_event(
        &mut self,
        event_date_param: usize,
        date_expr_param: &str,
        sort_order_param: usize,
        value_param: Decimal,
        value_expr_param: &str,
        value_expr_balance_param: bool,
        periods_param: usize,
        periods_expr_param: &str,
        skip_mask_len_param: usize,
        skip_mask_param: u128,
        intervals_param: usize,
        frequency_param: crate::FrequencyType,
        elem_extension_param: ElemExtension,
        mut list_parameter_param: Option<ListParameter>,
        mut list_descriptor_param: Option<ListDescriptor>,
        event_name_param: &str,
        next_name_param: &str,
    ) -> bool {
        let elem_extension = &elem_extension_param;
        let extension_type = elem_extension.extension_type();
        if list_parameter_param.is_none() {
            list_parameter_param =
                Option::from(ListParameter::new());
        }

        if list_descriptor_param.is_none() {
            list_descriptor_param = Option::from(ListDescriptor::new());
        }

        let new_elem_event: ElemEvent = ElemEvent::new(
            "",
            event_date_param,
            date_expr_param,
            sort_order_param,
            value_param,
            value_expr_param,
            value_expr_balance_param,
            periods_param,
            periods_expr_param,
            skip_mask_len_param,
            skip_mask_param,
            intervals_param,
            frequency_param,
            elem_extension_param,
            list_parameter_param,
            list_descriptor_param,
            event_name_param,
            next_name_param,
        );

        self.list_event.push(new_elem_event);
        if self.sort_on_add.get() {
            self.sort();
        }

        match self
            .list_event
            .iter()
            .position(|e| e.elem_type() == extension_type && e.event_date() == event_date_param)
        {
            None => false,
            Some(o) => {
                self.list_index.set(o);
                if !self.sort_on_add.get() {
                    self.sort_updated.set(true);
                }

                true
            }
        }
    }

    /// Performs a deep copy of this event list and return a new event list.
    ///
    /// # Arguments
    ///
    /// * `updating_json_param` - Updating from json.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self, updating_json_param: bool) -> ListEvent {
        let mut list_event = ListEvent::new(self.cashflow);

        let result = self.copy_list_event(&mut list_event, updating_json_param);

        match result {
            Err(_e) => {
                panic!("Copy event failed");
            }
            Ok(_o) => list_event,
        }
    }

    /// Performs a deep copy of this event list into the event list parameter.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to copy into.
    /// * `updating_json_param` - Updating from json.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful.

    pub fn copy_list_event(
        &self,
        list_event: &mut ListEvent,
        updating_json_param: bool,
    ) -> Result<(), crate::ErrorType> {
        let orig_list_index: usize = self.index();
        list_event.set_sort_on_add(false);

        for index in 0..self.count() {
            if !self.get_element(index) {
                break;
            }
            let result = self.copy_event(list_event, index, updating_json_param);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
        }
        list_event.set_sort_on_add(true); // Sorts list
        self.get_element(orig_list_index);

        Ok(())
    }

    /// Performs a deep copy of selected events.
    /// Copies selected events from this event list into the event
    /// list parameter.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to copy into.
    /// * `count` - The direction (i.e., positive or negative)
    ///     and the number of events to copy.
    ///     as they are copied.
    /// * `updating_json_param` - Updating from json.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful.

    pub fn copy_list_event_selected(
        &self,
        list_event: &mut ListEvent,
        mut count: i32,
        updating_json_param: bool,
    ) -> Result<(), crate::ErrorType> {
        let mut index = self.index() as i32;
        list_event.set_sort_on_add(false);

        while count != 0 {
            let result = self.copy_event(list_event, index as usize, updating_json_param);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
            if count > 0 {
                index += 1;

                if index >= (self.list_event.len() as i32) {
                    break;
                }

                count -= 1;
            } else {
                index -= 1;

                if index < 0 {
                    break;
                }

                count += 1;
            }
        }
        list_event.set_sort_on_add(true); // Sorts list

        Ok(())
    }

    /// Performs a deep copy of the event index into the event list parameter.
    ///
    /// # Arguments
    ///
    /// * `list_event` - The event list to copy into.
    /// * `index` - The event index to copy.
    /// * `updating_json_param` - Updating from json.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful.

    pub fn copy_event(
        &self,
        list_event: &mut ListEvent,
        index: usize,
        updating_json_param: bool,
    ) -> Result<(), crate::ErrorType> {
        match self.list_event.get(index) {
            None => {
                return Err(crate::ErrorType::Index);
            }
            Some(o) => {
                let new_extension = o.elem_extension().copy();

                let mut list_parameter_opt: Option<ListParameter> = None;
                match o.list_parameter().as_ref() {
                    None => {}
                    Some(o2) => {
                        list_parameter_opt =
                            Option::from(o2.copy(updating_json_param));
                    }
                }

                let mut list_descriptor_opt: Option<ListDescriptor> = None;
                match o.list_descriptor().as_ref() {
                    None => {}
                    Some(o2) => {
                        list_descriptor_opt =
                            Option::from(o2.copy(false, updating_json_param));
                    }
                }
                list_event.add_event(
                    o.event_date(),
                    o.date_expr(),
                    o.sort_order(),
                    o.value(),
                    o.value_expr(),
                    o.value_expr_balance(),
                    o.periods(),
                    o.periods_expr(),
                    o.skip_mask_len(),
                    o.skip_mask(),
                    o.intervals(),
                    o.frequency(),
                    new_extension,
                    list_parameter_opt,
                    list_descriptor_opt,
                    o.event_name(),
                    o.next_name(),
                );
            }
        }

        Ok(())
    }

    /// Returns if the event was from a cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow(&self) -> bool {
        self.cashflow
    }

    /// Get the vector of events.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list(&self) -> &Vec<ElemEvent> {
        &self.list_event
    }

    /// Get the mutable vector of events.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_mut(&mut self) -> &mut Vec<ElemEvent> {
        &mut self.list_event
    }

    /// Get the eom of the event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn eom(&self) -> bool {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.elem_extension().extension_eom(),
        }
    }

    /// Get the type of the event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_type(&self) -> crate::ExtensionType {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
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
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.event_type(),
        }
    }

    /// Get the date of the event in YYYYMMDD format.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_date(&self) -> usize {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.event_date(),
        }
    }

    /// Get the optional date expression evaluated when created from a template.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn date_expr(&self) -> &str {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.date_expr(),
        }
    }

    /// Get the sort order within the event date.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sort_order(&self) -> usize {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.sort_order(),
        }
    }

    /// Get the constant value or the result of an expression.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value(&self) -> Decimal {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.value(),
        }
    }

    /// Get the optional value expression evaluated when the event list is amortization or balanced.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_expr(&self) -> &str {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.value_expr(),
        }
    }

    /// Get the expression is evalulated when balanced, otherwise when amortization.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_expr_balance(&self) -> bool {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.value_expr_balance(),
        }
    }

    /// Get the constant periods or the result of an expression.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn periods(&self) -> usize {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.periods(),
        }
    }

    /// Get the optional periods expression evaluated when the event list is amortization.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn periods_expr(&self) -> &str {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.periods_expr(),
        }
    }

    /// Get the optional length of the periods to skip when the event list is amortization.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn skip_mask_len(&self) -> usize {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.skip_mask_len(),
        }
    }

    /// Get the optional periods to skip when the event list is amortization.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn skip_mask(&self) -> u128 {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.skip_mask(),
        }
    }

    /// Get the optional intervals of frequency between periods.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn intervals(&self) -> usize {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.intervals(),
        }
    }

    /// Get the frequency of the event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn frequency(&self) -> crate::FrequencyType {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.frequency(),
        }
    }

    /// Get the extension.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_extension(&self) -> &ElemExtension {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.elem_extension(),
        }
    }

    /// Get the event parameter list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_parameter(&self) -> Option<&ListParameter> {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.list_parameter(),
        }
    }

    /// Get the event descriptor list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_descriptor(&self) -> Option<&ListDescriptor> {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.list_descriptor(),
        }
    }

    /// Get the optional event name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn event_name(&self) -> &str {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.event_name(),
        }
    }

    /// Get the optional next name of the template.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn next_name(&self) -> &str {
        match self.list_event.get(self.list_index.get()) {
            None => {
                panic!("Event list index not set");
            }
            Some(o) => o.next_name(),
        }
    }

    /// Select an event based upon an event name.
    ///
    /// # Arguments
    ///
    /// * `name_param` - The event name to select.
    /// * `is_select_param` - Select If true select element, otherwise restore current element.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_name(&self, name_param: &str, is_select_param: bool) -> bool {
        for (index, elem) in self.list_event.iter().enumerate() {
            if name_param == elem.event_name() {
                if is_select_param {
                    self.set_index(index);
                }
                return true;
            }
        }

        false
    }

    /// Select an event based upon a statistic value name.
    ///
    /// # Arguments
    ///
    /// * `stat_name_param` - The statistic value name to select.
    /// * `is_select_param` - If true select element, otherwise restore current element.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_stat_name(&self, stat_name_param: &str, is_select_param: bool) -> bool {
        let mut index: usize = 0;

        for elem in self.list_event.iter() {
            if elem.elem_type() != crate::ExtensionType::StatisticValue {
                index += 1;
                continue;
            }

            if stat_name_param == elem.elem_extension().sv_name() {
                if is_select_param {
                    self.set_index(index);
                }
                return true;
            }

            index += 1;
        }

        false
    }

    /// Select an event based upon the extension type.
    ///
    /// # Arguments
    ///
    /// * `extension_type_param` - The extension type to select.
    /// * `iteration` - Select the nth iteration of the extension type.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_type(
        &self,
        extension_type_param: crate::ExtensionType,
        iteration_param: usize,
    ) -> bool {
        let mut iteration = iteration_param;
        let mut index: usize = 0;

        for elem in self.list_event.iter() {
            if elem.elem_type() != extension_type_param {
                index += 1;
                continue;
            }

            iteration -= 1;

            if iteration == 0 {
                self.set_index(index);
                return true;
            }

            index += 1;
        }

        false
    }

    /// Remove the selected event from the event list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_event.len() {
            return false;
        }

        self.list_event.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }

        true
    }

    /// Set the event type information.
    ///
    /// # Arguments
    ///
    /// * `event_type_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_event_type(&mut self, event_type_param: &str) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_event_type(event_type_param);
                true
            }
        }
    }

    /// Set the constant date of the event in YYYYMMDD format (major sort key).
    ///
    /// # Arguments
    ///
    /// * `event_date_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_event_date(&mut self, event_date_param: usize) -> bool {
        let elem_type: crate::ExtensionType;
        let event_date: usize;

        match self.list_event.get(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                elem_type = o.elem_type();
                event_date = o.event_date();
            }
        }

        if !self.set_date_result(event_date_param) {
            return false;
        }
        if self.sort_on_add.get() {
            self.sort();
        }

        match self
            .list_event
            .iter()
            .position(|e| e.elem_type() == elem_type && e.event_date() == event_date)
        {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add.get() {
            self.sort_updated.set(true);
        }
        true
    }

    /// Set the expression date of the event in YYYYMMDD format (major sort key).
    ///
    /// # Arguments
    ///
    /// * `event_date_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_date_result(&self, event_date_param: usize) -> bool {
        match self.list_event.get(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_event_date(event_date_param);
                true
            }
        }
    }

    /// Set the optional date expression evaluated when created from a template.
    ///
    /// # Arguments
    ///
    /// * `date_expr_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_date_expr(&mut self, date_expr_param: &str) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_date_expr(date_expr_param);
                true
            }
        }
    }

    /// Set the sort order within the event date (minor sort key).
    ///
    /// # Arguments
    ///
    /// * `sort_order_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_sort_order(&mut self, sort_order_param: usize) -> bool {
        let elem_type: crate::ExtensionType;
        let event_date: usize;

        match self.list_event.get_mut(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                elem_type = o.elem_type();
                event_date = o.event_date();
                o.set_sort_order(sort_order_param);
            }
        }
        if self.sort_on_add.get() {
            self.sort();
        }

        match self
            .list_event
            .iter()
            .position(|e| e.elem_type() == elem_type && e.event_date() == event_date)
        {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add.get() {
            self.sort_updated.set(true);
        }
        true
    }

    /// Set the constant value.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value(&self, value_param: Decimal) -> bool {
        if !self.set_value_result(value_param) {
            return false;
        }

        true
    }

    /// Set the value result of an expression.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_result(&self, value_param: Decimal) -> bool {
        match self.list_event.get(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value(value_param);
                true
            }
        }
    }

    /// Set the optional value expression evaluated when the event list is amortization or balanced.
    ///
    /// # Arguments
    ///
    /// * `value_expr_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_expr(&mut self, value_expr_param: &str) -> bool {
        if !self.set_value_expr_result(value_expr_param) {
            return false;
        }

        true
    }

    /// Set the value expression result.
    ///
    /// # Arguments
    ///
    /// * `value_expr_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_expr_result(&mut self, value_expr_param: &str) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value_expr(value_expr_param);
                true
            }
        }
    }

    /// Set the expression is evalulated when balanced, otherwise when amortization.
    ///
    /// # Arguments
    ///
    /// * `value_expr_balance_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_expr_balance(&mut self, value_expr_balance_param: bool) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value_expr_balance(value_expr_balance_param);
                true
            }
        }
    }

    /// Set the constant periods.
    ///
    /// # Arguments
    ///
    /// * `periods_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_periods(&self, periods_param: usize) -> bool {
        if !self.set_periods_result(periods_param) {
            return false;
        }

        true
    }

    /// Set the periods result of an expression.
    ///
    /// # Arguments
    ///
    /// * `periods_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_periods_result(&self, periods_param: usize) -> bool {
        match self.list_event.get(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_periods(periods_param);
                true
            }
        }
    }

    /// Set the optional periods expression evaluated when the event list is amortization.
    ///
    /// # Arguments
    ///
    /// * `periods_expr_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_periods_expr(&mut self, periods_expr_param: &str) -> bool {
        if !self.set_periods_expr_result(periods_expr_param) {
            return false;
        }

        true
    }

    /// Set the periods expression result.
    ///
    /// # Arguments
    ///
    /// * `periods_expr_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_periods_expr_result(&mut self, periods_expr_param: &str) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_periods_expr(periods_expr_param);
                true
            }
        }
    }

    /// Set the optional periods to skip when the event list is amortization.
    ///
    /// # Arguments
    ///
    /// * `skip_mask_len_param` - Skip mask length.
    /// * `skip_mask_param` - Skip mask.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_skip_mask(&self, skip_mask_len_param: usize, skip_mask_param: u128) -> bool {
        match self.list_event.get(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_skip_mask(skip_mask_len_param, skip_mask_param);
                true
            }
        }
    }

    /// Set the optional intervals of frequency between periods.
    ///
    /// # Arguments
    ///
    /// * `intervals_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_intervals(&mut self, intervals_param: usize) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_intervals(intervals_param);
                true
            }
        }
    }

    /// Set the frequency of the event.
    ///
    /// # Arguments
    ///
    /// * `frequency_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_frequency(&mut self, frequency_param: crate::FrequencyType) -> bool {
        if !self.set_frequency_result(frequency_param) {
            return false;
        }

        true
    }

    /// Set the frequency of the event.
    ///
    /// # Arguments
    ///
    /// * `frequency_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_frequency_result(&mut self, frequency_param: crate::FrequencyType) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_frequency(frequency_param);
                true
            }
        }
    }

    /// Set the ElemPrincipalChange object.
    ///
    /// # Arguments
    ///
    /// * `principal_change_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_principal_change(&mut self, principal_change_param: ElemPrincipalChange) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.elem_extension_mut()
                    .set_principal_change(principal_change_param);
                true
            }
        }
    }

    /// Set the ElemCurrentValue object.
    ///
    /// # Arguments
    ///
    /// * `current_value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_current_value(&mut self, current_value_param: ElemCurrentValue) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.elem_extension_mut()
                    .set_current_value(current_value_param);
                true
            }
        }
    }

    /// Set the ElemInterestChange object.
    ///
    /// # Arguments
    ///
    /// * `interest_change_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_interest_change(&mut self, interest_change_param: ElemInterestChange) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.elem_extension_mut()
                    .set_interest_change(interest_change_param);
                true
            }
        }
    }

    /// Set the ElemStatisticValue object.
    ///
    /// # Arguments
    ///
    /// * `statistic_value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_statistic_value(&mut self, statistic_value_param: ElemStatisticValue) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.elem_extension_mut()
                    .set_statistic_value(statistic_value_param);
                true
            }
        }
    }

    /// Set the optional event name.
    ///
    /// # Arguments
    ///
    /// * `event_name_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_event_name(&mut self, event_name_param: &str) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_event_name(event_name_param);
                true
            }
        }
    }

    /// Set the optional next name of the template.
    ///
    /// # Arguments
    ///
    /// * `next_name_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_next_name(&mut self, next_name_param: &str) -> bool {
        match self.list_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_next_name(next_name_param);
                true
            }
        }
    }

    /// Determines when the event list is sorted.
    ///
    /// # Arguments
    ///
    /// * `sort_on_add_param` - If true sort when an event is added, otherwise do not sort (for bulk adds).

    pub fn set_sort_on_add(&mut self, sort_on_add_param: bool) -> bool {
        if self.sort_on_add.get() == sort_on_add_param {
            return false;
        }
        self.sort_on_add.set(sort_on_add_param);
        if self.sort_on_add.get() && self.sort_updated.get() {
            self.sort();

            match self.list_event.get(self.list_index.get()) {
                None => {
                    return false;
                }
                Some(o) => {
                    match self.list_event.iter().position(|e| {
                        e.elem_type() == o.elem_type() && e.event_date() == o.event_date()
                    }) {
                        None => {}
                        Some(o2) => {
                            self.list_index.set(o2);
                        }
                    }
                }
            }
        }
        self.sort_updated.set(false);

        true
    }

    /// Determines the sort updated.
    ///
    /// # Arguments
    ///
    /// * `sort_updated_param` - Sort updated parameter.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_sort_updated(&self, sort_updated_param: bool) -> bool {
        if sort_updated_param == self.sort_updated.get() {
            return false;
        }

        self.sort_updated.set(sort_updated_param);

        true
    }

    /// Sort the event list.

    fn sort(&mut self) {
        self.list_event.sort_by(|a, b| ListEvent::cmp(a, b));
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

    fn cmp(a: &ElemEvent, b: &ElemEvent) -> std::cmp::Ordering {
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
