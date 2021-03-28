//! The manager of the AmFn engine component.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::rc::Rc;

use super::{
    CalcExpression, CalcUtility, ElemPreferences, ListCashflow, ListExchangeRate, ListTemplateGroup,
};
use crate::core::{
    CoreManager, CoreUtility, ElemColumn, ElemSymbol, ListAmortization, ListColumn, ListDescriptor,
    ListEvent, ListSummary,
};
use crate::ListTrait;

pub struct CalcManager {
    /// Calc manager element (injected from the engine and cloned).
    calc_manager: Option<Rc<RefCell<CalcManager>>>,
    /// Core manager element.
    core_manager: Rc<RefCell<CoreManager>>,

    /// User preferences element.
    elem_preferences: Option<ElemPreferences>,
    /// List of cashflows.
    list_cashflow: ListCashflow,
    /// List of events representing the paste buffer.
    list_event_pb: Option<ListEvent>,

    /// List of exchange rates.
    list_exchange_rate: Option<ListExchangeRate>,
    /// List of template groups.
    list_template_group: ListTemplateGroup,

    /// Currently updating while loading a JSON source.
    updating_json: Cell<bool>,
}

/// The manager implementation of the AmFn engine component.

impl CalcManager {
    /// Create and return a new calc manager element.
    ///
    /// # Arguments
    ///
    /// * `core_manager_param` - CoreManager element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(core_manager_param: CoreManager) -> CalcManager {
        CalcManager {
            calc_manager: None,
            core_manager: Rc::new(RefCell::new(core_manager_param)),
            elem_preferences: None,
            list_cashflow: ListCashflow::new(),
            list_event_pb: None,
            list_exchange_rate: None,
            list_template_group: ListTemplateGroup::new(),
            updating_json: Cell::new(false),
        }
    }

    /// Initialize the calc manager element.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager element.

    pub fn init_calc_manager(&mut self, calc_manager_param: &Rc<RefCell<CalcManager>>) {
        // Clone the engine's calculation manager
        self.calc_manager = Option::from(Rc::clone(calc_manager_param));

        match self.calc_manager.as_ref() {
            None => {
                panic!("Calculator manager not set");
            }
            Some(o) => {
                self.elem_preferences = Option::from(ElemPreferences::new(
                    o,
                    "",
                    "",
                    "",
                    "",
                    0,
                    crate::DEFAULT_DECIMAL_DIGITS,
                    -1,
                    -1,
                    -1,
                    None,
                    None,
                    false,
                    false,
                ));
                self.list_event_pb = Option::from(ListEvent::new(false));
                self.list_exchange_rate = Option::from(ListExchangeRate::new());
            }
        }
    }

    /// Returns the calculation manager element.
    ///
    /// # Return
    ///
    /// * See description.

    fn calc_manager(&self) -> &Rc<RefCell<CalcManager>> {
        match self.calc_manager.as_ref() {
            None => {
                panic!("Missing calc manager");
            }
            Some(o) => o,
        }
    }

    /// Returns the calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    fn calc_mgr(&self) -> Ref<CalcManager> {
        match self.calc_manager.as_ref() {
            None => {
                panic!("Missing calc manager");
            }
            Some(o) => o.borrow(),
        }
    }

    /// Returns the mutable calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    fn calc_mgr_mut(&self) -> RefMut<CalcManager> {
        match self.calc_manager.as_ref() {
            None => {
                panic!("Missing calc manager");
            }
            Some(o) => o.borrow_mut(),
        }
    }

    /// Get the core manager.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn core_manager(&self) -> &Rc<RefCell<CoreManager>> {
        &self.core_manager
    }

    /// Get the core manager reference.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn mgr(&self) -> Ref<CoreManager> {
        self.core_manager.borrow()
    }

    /// Get the mutable core manager reference.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn mgr_mut(&mut self) -> RefMut<CoreManager> {
        self.core_manager.borrow_mut()
    }

    /// Clear all engine elements and lists.

    pub fn clear(&mut self) {
        self.preferences_mut().clear();
        self.list_template_group_mut().clear();
        self.list_cashflow_mut().clear();
        self.list_exchange_rate_mut().clear();
        self.list_event_pb_mut().clear();
    }

    /// Copies the event list from the currently selected template event into
    /// a new event list.
    ///
    /// # Arguments
    ///
    /// * `date_param` - Base starting date for the new event(s).
    /// * `end_date_param` - Base ending date for the new event(s).
    /// * `new_date_param` - Next date for the new event(s) (i.e.,
    ///     normally end_date_param plus one period).
    /// * `frequency_param` - Next frequency for the new event(s).
    ///
    /// # Return
    ///
    /// * The new event list, otherwise an error code.

    pub fn copy_template_events(
        &self,
        date_param: usize,
        end_date_param: usize,
        new_date_param: usize,
        frequency_param: crate::FrequencyType,
    ) -> Result<ListEvent, crate::ErrorType> {
        let updating_json = self.updating_json();
        let fiscal_year_start = self.fiscal_year_start(false);
        let decimal_digits = self.decimal_digits(false);
        let list_template_event_list_event = self
            .list_template_group()
            .list_template_event()
            .list_event();
        if list_template_event_list_event.count() == 0 {
            return Err(crate::ErrorType::Element);
        }

        let mut new_list_event = ListEvent::new(true);

        let orig_index = list_template_event_list_event.index();

        new_list_event.set_sort_on_add(false);
        for index in 0..list_template_event_list_event.count() {
            if !list_template_event_list_event.get_element(index) {
                break;
            }

            let result = list_template_event_list_event.copy_event(
                &mut new_list_event,
                index,
                updating_json,
            );
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
            let event_date = date_param;
            let end_date = end_date_param;
            let mut new_date = new_date_param;
            let intervals = new_list_event.intervals();
            let frequency = frequency_param;
            let eom = new_list_event.elem_extension().extension_eom();

            if !new_list_event.date_expr().is_empty() {
                let mut calc_expression =
                    CalcExpression::new(self.calc_manager(), fiscal_year_start, decimal_digits);

                calc_expression.init_expression(
                    None,
                    None,
                    list_template_event_list_event.list_parameter(),
                    new_list_event.date_expr(),
                );
                calc_expression.set_symbol_integer("intDate", event_date);
                calc_expression.set_symbol_integer("intEndDate", end_date);
                calc_expression.set_symbol_integer("intNewDate", new_date);
                calc_expression.set_symbol_integer("intIntervals", intervals);
                calc_expression.set_symbol_string(
                    "strFrequency",
                    CoreUtility::get_frequency_mnemonic(frequency).as_str(),
                );
                calc_expression.set_symbol_integer("intEOM", if eom { 1 } else { 0 });

                let elem_result_symbol: ElemSymbol;
                let result = calc_expression.evaluate(None, None);
                match result {
                    Err(e) => {
                        list_template_event_list_event.get_element(orig_index);
                        return Err(e);
                    }
                    Ok(o) => {
                        elem_result_symbol = o;
                    }
                }

                match elem_result_symbol.sym_type() {
                    crate::TokenType::Integer => {
                        new_date = elem_result_symbol.sym_integer();
                    }
                    crate::TokenType::Decimal => {
                        match elem_result_symbol.sym_decimal().to_usize() {
                            None => {
                                new_date = 0;
                            }
                            Some(o) => {
                                new_date = o;
                            }
                        }
                    }
                    crate::TokenType::String => {
                        new_date = CoreUtility::parse_integer(elem_result_symbol.sym_string());
                    }
                    _ => {}
                }
                if new_date == 0 {
                    list_template_event_list_event.get_element(orig_index);
                    return Err(crate::ErrorType::Date);
                }
            }
            new_list_event.set_date_result(new_date);
            if new_list_event.elem_type() != crate::ExtensionType::StatisticValue {
                new_list_event.set_frequency_result(frequency);
            }
        }

        new_list_event.set_sort_on_add(true); // Sorts list

        list_template_event_list_event.get_element(orig_index);
        new_list_event.get_element(0);

        Ok(new_list_event)
    }

    /// Get the combine principal change events.
    /// The cashflow and user preferences are
    /// searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn combine_principal(&self, cashflow: bool) -> bool {
        if cashflow {
            let cashflow_preferences = self.list_cashflow().preferences();
            match cashflow_preferences.as_ref() {
                None => {
                    return false;
                }
                Some(o) => {
                    if o.combine_principal() >= 0 {
                        return o.combine_principal() > 0;
                    }
                }
            }
        }
        if self.preferences().combine_principal() >= 0 {
            return self.preferences().combine_principal() > 0;
        }

        crate::DEFAULT_COMBINE_PRINCIPAL
    }

    /// Get the After balancing and during compression.
    /// The cashflow and user preferences are searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn compress_descriptor(&self, cashflow: bool) -> bool {
        if cashflow {
            let cashflow_preferences = self.list_cashflow().preferences();
            match cashflow_preferences.as_ref() {
                None => {
                    return false;
                }
                Some(o) => {
                    if o.compress_descriptor() >= 0 {
                        return o.compress_descriptor() > 0;
                    }
                }
            }
        }
        if self.preferences().compress_descriptor() >= 0 {
            return self.preferences().compress_descriptor() > 0;
        }

        crate::DEFAULT_COMPRESS_DESCRIPTOR
    }

    /// Get the resolved cross rate code.
    /// The cashflow and user preferences are
    /// searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cross_rate_code(&self, cashflow: bool) -> &str {
        if cashflow {
            let cashflow_preferences = self.list_cashflow().preferences();
            match cashflow_preferences.as_ref() {
                None => {
                    return "";
                }
                Some(o) => {
                    if !o.cross_rate_code().is_empty() {
                        return o.cross_rate_code();
                    }
                }
            }
        }
        self.preferences().cross_rate_code()
    }

    /// Get the resolved start of fiscal year.
    /// The cashflow and user preferences are
    /// searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn fiscal_year_start(&self, cashflow: bool) -> usize {
        if cashflow {
            let cashflow_preferences = self.list_cashflow().preferences();
            match cashflow_preferences.as_ref() {
                None => {
                    return crate::DEFAULT_FISCAL_YEAR_START;
                }
                Some(o) => {
                    if o.fiscal_year_start() != usize::MAX {
                        return o.fiscal_year_start();
                    }
                }
            }
        }

        if self.preferences().fiscal_year_start() > 0 {
            return self.preferences().fiscal_year_start();
        }

        crate::DEFAULT_FISCAL_YEAR_START
    }

    /// Get the resolved number of significant decimal digits.
    /// The cashflow and user preferences are
    /// searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_digits(&self, cashflow: bool) -> usize {
        if cashflow {
            let cashflow_preferences = self.list_cashflow().preferences();
            match cashflow_preferences.as_ref() {
                None => {
                    return crate::DEFAULT_DECIMAL_DIGITS;
                }
                Some(o) => {
                    if o.decimal_digits() != usize::MAX {
                        return o.decimal_digits();
                    }
                }
            }
        }

        if self.preferences().decimal_digits() != usize::MAX {
            return self.preferences().decimal_digits();
        }

        crate::DEFAULT_DECIMAL_DIGITS
    }

    /// Get the resolved default encoding.
    /// The cashflow and user preferences are
    /// searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn default_encoding(&self, cashflow: bool) -> &str {
        if cashflow {
            let cashflow_preferences = self.list_cashflow().preferences();
            match cashflow_preferences.as_ref() {
                None => {
                    return crate::DEFAULT_ENCODING;
                }
                Some(o) => {
                    if !o.default_encoding().is_empty() {
                        return o.default_encoding();
                    }
                }
            }
        }
        if !self.preferences().default_encoding().is_empty() {
            return self.preferences().default_encoding();
        }

        crate::DEFAULT_ENCODING
    }

    /// Searches the various descriptor lists, from lowest
    /// to highest, and returns the constant value for a
    /// group, name, type, and code.
    ///
    /// # Arguments
    ///
    /// * `group` - The group name of the descriptor.
    /// * `name` - The name of the descriptor.
    /// * `desc_type` - The type of the descriptor.
    /// * `code` - The code of the descriptor.
    /// * `cashflow` - Search the cashflow descriptor list.
    /// * `is_event` - Search the event descriptor list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn descriptor_value(
        &self,
        group: &str,
        name: &str,
        desc_type: &str,
        code: &str,
        cashflow: bool,
        is_event: bool,
    ) -> String {
        let mut list_descriptor_cashflow: Option<&ListDescriptor> = None;
        let mut list_descriptor_event: Option<&ListDescriptor> = None;

        if cashflow {
            match self.list_cashflow().preferences().as_ref() {
                None => {}
                Some(o) => {
                    list_descriptor_cashflow = Option::from(o.list_descriptor());
                }
            }
            if is_event {
                match self.list_cashflow().list_event().as_ref() {
                    None => {}
                    Some(o2) => {
                        list_descriptor_event = o2.list_descriptor();
                    }
                }
            }
        }
        CoreUtility::get_descriptor_value(
            Option::from(self.preferences().list_descriptor()),
            list_descriptor_cashflow,
            list_descriptor_event,
            group,
            name,
            desc_type,
            code,
        )
    }

    /// Get the resolved template group name.
    /// The cashflow and user preferences are
    /// searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn group(&self, cashflow: bool) -> &str {
        if cashflow {
            let cashflow_preferences = self.list_cashflow().preferences();
            match cashflow_preferences.as_ref() {
                None => {}
                Some(o) => {
                    if !o.group().is_empty() {
                        return o.group();
                    }
                }
            }
        }
        if !self.preferences().group().is_empty() {
            return self.preferences().group();
        }

        ""
    }

    /// Get the user preferences element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn preferences(&self) -> &ElemPreferences {
        match self.elem_preferences.as_ref() {
            None => {
                panic!("Missing elem preferences");
            }
            Some(o) => o,
        }
    }

    /// Get the mutable user preferences element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn preferences_mut(&mut self) -> &mut ElemPreferences {
        match self.elem_preferences.as_mut() {
            None => {
                panic!("Missing elem preferences");
            }
            Some(o) => o,
        }
    }

    /// Get the list of cashflows.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_cashflow(&self) -> &ListCashflow {
        &self.list_cashflow
    }

    /// Get the mutable list of cashflows.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_cashflow_mut(&mut self) -> &mut ListCashflow {
        &mut self.list_cashflow
    }

    /// Get the list of exchange rates.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_exchange_rate(&self) -> &ListExchangeRate {
        match self.list_exchange_rate.as_ref() {
            None => {
                panic!("Missing list exchange rate")
            }
            Some(o) => o,
        }
    }

    /// Get the mutable list of exchange rates.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_exchange_rate_mut(&mut self) -> &mut ListExchangeRate {
        match self.list_exchange_rate.as_mut() {
            None => {
                panic!("Missing list exchange rate")
            }
            Some(o) => o,
        }
    }

    /// Get the list of events representing the paste buffer.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_pb(&self) -> &ListEvent {
        match self.list_event_pb.as_ref() {
            None => {
                panic!("Missing list event paste buffer")
            }
            Some(o) => o,
        }
    }

    /// Get the mutable list of events representing the paste buffer.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_pb_mut(&mut self) -> &mut ListEvent {
        match self.list_event_pb.as_mut() {
            None => {
                panic!("Missing list event paste buffer")
            }
            Some(o) => o,
        }
    }

    /// Get the list of template groups.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_template_group(&self) -> &ListTemplateGroup {
        &self.list_template_group
    }

    /// Get the mutable list of template groups.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_template_group_mut(&mut self) -> &mut ListTemplateGroup {
        &mut self.list_template_group
    }

    /// Get the error text corresponding to an error value.
    ///
    /// # Arguments
    ///
    /// * `error` - The value of the error.
    ///
    /// # Return
    ///
    /// * See description.
    pub fn get_error_string(&self, error: crate::ErrorType) -> String {
        let mgr = self.mgr();

        let error_index = error as usize;
        if !mgr.map_error().get_element_by_value(error_index) {
            return format!("{}{}", crate::ERROR_PREFIX, error_index);
        }

        let key = mgr.map_error().key();
        let fs = String::from(mgr.list_locale().get_resource(key));

        fs
    }

    /// Get the resolved locale.
    /// The cashflow and user preferences are searched
    /// in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * ISO language code (ISO 639)_ISO country code (ISO 3166).

    pub fn locale(&self, cashflow: bool) -> String {
        let cashflow_preferences = self.list_cashflow().preferences();

        match cashflow_preferences.as_ref() {
            None => {}
            Some(o) => {
                if cashflow && !o.locale_str().is_empty() {
                    return String::from(o.locale_str());
                }
            }
        }

        if !self.preferences().locale_str().is_empty() {
            return String::from(self.preferences().locale_str());
        }

        String::from(
            self.core_manager
                .borrow()
                .list_locale()
                .get_locale_str(false),
        )
    }

    /// Get the statistic events from the compressed
    /// event list.
    /// The cashflow and user preferences are
    /// searched in that order.
    ///
    /// # Arguments
    ///
    /// * `cashflow` - Search the cashflow preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn statistic_events(&self, cashflow: bool) -> bool {
        let cashflow_preferences = self.list_cashflow().preferences();

        match cashflow_preferences.as_ref() {
            None => {}
            Some(o) => {
                if cashflow && o.statistic_events() >= 0 {
                    return o.statistic_events() > 0;
                }
            }
        }

        if self.preferences().statistic_events() >= 0 {
            return self.preferences().statistic_events() > 0;
        }

        crate::DEFAULT_OMIT_STATISTIC_EVENTS
    }

    /// Get the updating json.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn updating_json(&self) -> bool {
        self.updating_json.get()
    }

    /// Append to the list cashflow.
    ///
    /// # Arguments
    ///
    /// * `list_cashflow` - See description.

    pub fn append_cashflows(&mut self, mut list_cashflow: ListCashflow) {
        let cfl = list_cashflow.list_mut();

        loop {
            match cfl.pop() {
                None => {
                    break;
                }
                Some(o) => {
                    self.list_cashflow.list_mut().push(o);
                }
            }
        }
    }

    /// Append to the list template group.
    ///
    /// # Arguments
    ///
    /// * `list_template_group` - See description.

    pub fn append_list_template_group(&mut self, mut list_template_group: ListTemplateGroup) {
        let tl = list_template_group.list_mut();

        loop {
            match tl.pop() {
                None => {
                    break;
                }
                Some(o) => {
                    self.list_template_group.list_mut().push(o);
                }
            }
        }
    }

    /// Set the preferences.
    ///
    /// # Arguments
    ///
    /// * `preferences` - See description.

    pub fn set_preferences(&mut self, preferences: ElemPreferences) {
        self.elem_preferences = Option::from(preferences);
    }

    /// Set the list cashflow.
    ///
    /// # Arguments
    ///
    /// * `list_cashflow` - See description.

    pub fn set_list_cashflow(&mut self, list_cashflow: ListCashflow) {
        self.list_cashflow = list_cashflow;
    }

    /// Set the list exchange rate.
    ///
    /// # Arguments
    ///
    /// * `list_exchange_rate` - See description.

    pub fn set_list_exchange_rate(&mut self, list_exchange_rate: ListExchangeRate) {
        self.list_exchange_rate = Option::from(list_exchange_rate);
    }

    /// Set the list template group.
    ///
    /// # Arguments
    ///
    /// * `list_template_group` - See description.

    pub fn set_list_template_group(&mut self, list_template_group: ListTemplateGroup) {
        self.list_template_group = list_template_group;
    }

    /// Set the updating json.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.

    pub fn set_updating_json(&self, value_param: bool) {
        self.updating_json.set(value_param);
    }

    /// Performs a deep copy of the selected events into the event list paste buffer.
    ///
    /// # Arguments
    ///
    /// * `count` - The direction (i.e., positive or negative)
    ///     and the number of events to copy.

    pub fn pb_copy(&self, count: i32) -> Result<(), crate::ErrorType> {
        let updating_json = self.updating_json.get();

        let mgr = self.calc_mgr();
        let list_event_opt = mgr.list_cashflow().list_event();
        let list_event: &ListEvent;
        match list_event_opt.as_ref() {
            None => {
                panic!("Missing list event");
            }
            Some(o) => {
                list_event = &o;
            }
        }

        let mut reg_mut = self.calc_mgr_mut();
        let list_event_pb = reg_mut.list_event_pb_mut();

        list_event_pb.clear();

        list_event.copy_list_event_selected(list_event_pb, count, updating_json)
    }

    /// Performs a deep copy of the event list paste buffer into the selected event list.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise an error value.

    pub fn pb_paste(&self) -> Result<(), crate::ErrorType> {
        let updating_json = self.updating_json.get();

        let mut mgr = self.calc_mgr_mut();

        match mgr.list_cashflow_mut().list_event_mut() {
            None => {
                panic!("Missing list event");
            }
            Some(mut o) => self.list_event_pb().copy_list_event(&mut o, updating_json),
        }
    }

    /// Get the column name resource key.
    ///
    /// # Arguments
    ///
    /// * `column_value` - Column name value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn col_name_resource_key(column_value: crate::ColumnType) -> String {
        CoreUtility::get_col_name_resource_key(column_value)
    }

    /// Convert a value from the cashflow code to the event code.
    /// Cross rates are used if the exchange rate is unavailable and
    /// the cross rate international currency code is not empty.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_convert_currency_event(&self, value: Decimal) -> Decimal {
        let mgr = self.mgr();
        let list_locale = mgr.list_locale();
        let cashflow_currency_code = list_locale.cashflow_currency_code();
        let event_currency_code = list_locale.event_currency_code();

        CalcUtility::convert_currency_event(
            &self.calc_mgr(),
            cashflow_currency_code,
            event_currency_code,
            value,
        )
    }

    /// Convert a value from a currency code to another currency code.
    /// Cross rates are used if the exchange rate is unavailable and
    /// the cross rate international currency code is not empty.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert.
    /// * `from_code` - The current currency code.
    /// * `to_code` - The new currency code.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_convert_currency_value(
        &self,
        value: Decimal,
        from_code: &str,
        to_code: &str,
    ) -> Decimal {
        if from_code.is_empty() || from_code == to_code {
            return value;
        }
        self.list_exchange_rate().convert_currency(
            value,
            from_code,
            to_code,
            self.cross_rate_code(true),
        )
    }

    /// Get the appropriate event list value as a string.
    ///
    /// # Arguments
    ///
    /// * `elem_column` - Column element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_event_value(&self, elem_column: &ElemColumn) -> String {
        CalcUtility::get_event_value(self.calc_manager(), elem_column)
    }

    /// Get the appropriate amortization list value as a string.
    ///
    /// # Arguments
    ///
    /// * `elem_column` - Column element.
    /// * `list_am_opt` - Amortization list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_am_value(
        &self,
        elem_column: &ElemColumn,
        list_am_opt: &ListAmortization,
    ) -> String {
        CalcUtility::get_am_value(self.calc_manager(), elem_column, list_am_opt)
    }

    /// Determine if the event column is empty.
    ///
    /// # Arguments
    ///
    /// * `elem_column` - Column element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_is_event_column_empty(&self, elem_column: &ElemColumn) -> bool {
        CalcUtility::is_event_column_empty(self.calc_manager(), elem_column)
    }

    /// Create and return a column list object.
    ///
    /// # Arguments
    ///
    /// * `event_type` - The type of table.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_parse_columns(&self, event_type: crate::TableType) -> ListColumn {
        CalcUtility::parse_columns(self.calc_manager(), event_type, true)
    }

    /// Create and return a summary list object.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_parse_summary(&self) -> ListSummary {
        CalcUtility::parse_summary(self.calc_manager())
    }

    /// Calculates number of intervals between two dates.
    /// If intDate2 is greater than or equal to intDate1,
    /// the result will be positive, otherwise the result
    /// will be negative.
    ///
    /// # Arguments
    ///
    /// * `date1` - First date in YYYYMMDD format.
    /// * `date2` - Second date in YYYYMMDD format.
    /// * `frequency` - Date frequency.
    /// * `intervals` - Number of intervals of frequency.
    /// * `eom` - Adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * Number of intervals (positive or negative).

    pub fn util_date_diff(
        date1: usize,
        date2: usize,
        frequency: crate::FrequencyType,
        intervals: usize,
        eom: bool,
    ) -> i32 {
        CoreUtility::date_diff(
            date1,
            date2,
            frequency,
            if intervals > 0 { intervals } else { 1 },
            eom,
        )
    }

    /// Calculates a new date based upon a given date and number of intervals.
    /// If intervals is positive, the resulting date will be greater
    /// than event_date, otherwise the resulting date will be less than
    /// event_date.
    ///
    /// # Arguments
    ///
    /// * `event_date` - Date in YYYYMMDD format.
    /// * `periods` - Number of periods.
    /// * `frequency` - Date frequency.
    /// * `intervals` - Number of intervals of frequency.
    /// * `eom` - Adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * New date in YYYYMMDD format.

    pub fn util_date_newi(
        event_date: usize,
        mut periods: usize,
        frequency: crate::FrequencyType,
        mut intervals: i32,
        eom: bool,
    ) -> usize {
        let mut new_date = event_date;
        let orig_date = event_date;
        periods = if periods > 0 { periods } else { 1 };
        intervals = if intervals != 0 { intervals } else { 1 };
        while periods > 0 {
            new_date = CoreUtility::date_newi(orig_date, new_date, frequency, intervals, eom);
            periods -= 1;
        }
        new_date
    }

    /// Calculates a new date based upon a given date and number of intervals.
    ///
    /// # Arguments
    ///
    /// * `event_date` - Date in YYYYMMDD format.
    /// * `periods` - Number of periods.
    /// * `frequency` - Date frequency.
    /// * `intervals` - Number of intervals of frequency.
    /// * `eom` - Adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * New date in YYYYMMDD format.

    pub fn util_date_new(
        event_date: usize,
        periods: usize,
        frequency: crate::FrequencyType,
        intervals: usize,
        eom: bool,
    ) -> usize {
        CalcManager::util_date_newi(event_date, periods, frequency, intervals as i32, eom)
    }

    /// Returns the current date in YYYYMMDD format.
    ///
    /// # Return
    ///
    /// * Current date in YYYYMMDD format.

    pub fn util_date_now() -> usize {
        CoreUtility::date_now()
    }

    /// Return the number of intervals in a year for a frequency.
    ///
    /// # Arguments
    ///
    /// * `frequency` - Frequency value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_intervals_in_year(frequency: crate::FrequencyType) -> usize {
        CoreUtility::intervals_in_year(frequency, crate::DEFAULT_DAYS_IN_YEAR)
    }

    /// Return the number of intervals in a year for a frequency mnemonic.
    ///
    /// # Arguments
    ///
    /// * `frequency` - The frequency mnemonic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn util_intervals_in_year_by_name(frequency: &str) -> usize {
        CoreUtility::intervals_in_year(
            CoreUtility::get_frequency(frequency),
            crate::DEFAULT_DAYS_IN_YEAR,
        )
    }

    /// Converts a Daily Rate (DR) into a Nominal Annual Rate (NAR).
    ///
    /// # Arguments
    ///
    /// * `dr` - The daily interest rate expressed as a percentage.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The nominal annual rate as a percentage.

    pub fn util_rate_dr_to_nar(dr: Decimal, days_in_year: usize) -> Decimal {
        CoreUtility::rate_dr_to_nar(dr / dec!(100.0), days_in_year) * dec!(100.0)
    }

    /// Convert an Effective Annual Rate (EAR) into a Nominal Annual Rate (NAR).
    ///
    /// # Arguments
    ///
    /// * `ear` - The effective annual interest rate expressed as a percentage.
    /// * `compound_frequency` - Actual compounding frequency.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The nominal annual rate as a percentage.

    pub fn util_rate_ear_to_nar(
        ear: Decimal,
        compound_frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        CoreUtility::rate_ear_to_nar(ear / dec!(100.0), compound_frequency, days_in_year)
            * dec!(100.0)
    }

    /// Converts a Nominal Annual Rate (NAR) into a Daily Rate (DR).
    ///
    /// # Arguments
    ///
    /// * `nar` - The nominal annual interest rate expressed as a percentage.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The daily rate as a percentage.

    pub fn util_rate_nar_to_dr(nar: Decimal, days_in_year: usize) -> Decimal {
        CoreUtility::rate_nar_to_dr(nar / dec!(100.0), days_in_year) * dec!(100.0)
    }

    /// Convert a Nominal Annual Rate (NAR) into an Effective Annual Rate (EAR).
    ///
    /// # Arguments
    ///
    /// * `nar` - The nominal annual interest rate expressed as a percentage.
    /// * `compound_frequency` - Actual compounding frequency.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The effective annual rate as a percentage.

    pub fn util_rate_nar_to_ear(
        nar: Decimal,
        compound_frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        CoreUtility::rate_nar_to_ear(nar / dec!(100.0), compound_frequency, days_in_year)
            * dec!(100.0)
    }

    /// Convert a Nominal Annual Rate (NAR) into a Periodic Rate (PR).
    ///
    /// # Arguments
    ///
    /// * `nar` - The nominal annual interest rate expressed as a percentage.
    /// * `frequency` - Frequency value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The periodic rate as a percentage.

    pub fn util_rate_nar_to_pr(
        nar: Decimal,
        frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        CoreUtility::rate_nar_to_pr(nar / dec!(100.0), frequency, days_in_year) * dec!(100.0)
    }

    /// Convert a Periodic Rate (PR) into a Nominal Annual Rate (NAR).
    ///
    /// # Arguments
    ///
    /// * `pr` - The periodic interest rate expressed as a percentage.
    /// * `frequency` - Frequency value.
    /// * `days_in_year` - Number of days in the year.
    ///
    /// # Return
    ///
    /// * The nominal annual rate as a percentage.

    pub fn util_rate_pr_to_nar(
        pr: Decimal,
        frequency: crate::FrequencyType,
        days_in_year: usize,
    ) -> Decimal {
        CoreUtility::rate_pr_to_nar(pr / dec!(100.0), frequency, days_in_year) * dec!(100.0)
    }

    /// Round the value to the appropriate number of decimal digits.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to round.
    ///
    /// # Return
    ///
    /// * The rounded value.

    pub fn util_round(&self, value: Decimal) -> Decimal {
        CoreUtility::util_round(value, self.decimal_digits(true))
    }
}
