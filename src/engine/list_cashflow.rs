//! List of currently active cashflows.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::{Cell, Ref, RefCell};
use std::cmp::Ordering::Equal;
use std::rc::Rc;

use super::{CalcCalculate, CalcManager, ElemCashflow, ElemCashflowStats, ElemPreferences};
use crate::core::{ElemBalanceResult, ListAmortization, ListEvent, ListStatisticHelper};
use crate::{ExtensionTrait, ListTrait};

pub struct ListCashflow {
    /// Calculator manager element.
    calc_manager: Option<Rc<RefCell<CalcManager>>>,

    /// The list of cashflows.
    list_cashflow: Vec<ElemCashflow>,

    /// The index of the currently selected cashflow element.
    list_index: Cell<usize>,
}

/// List of currently active cashflows list implementation.

impl ListTrait for ListCashflow {
    /// Clear all cashflows from the cashflow list.

    fn clear(&mut self) {
        self.calc_mgr()
            .mgr()
            .list_locale_mut()
            .select_cashflow_locale("");

        self.list_mut().clear();
        self.list_index.set(usize::MAX);
    }

    /// Get the count of the cashflow list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_cashflow.len()
    }

    /// Get the index of the selected cashflow (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
    }

    /// Select a cashflow based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index value of the cashflow to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_cashflow.len() {
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
        if index_param >= self.list_cashflow.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of currently active cashflows default implementation.

impl Default for ListCashflow {
    /// Create and return a new list cashflow.
    ///
    /// # Return
    ///
    /// * See description.   

    fn default() -> Self {
        ListCashflow::new()
    }
}

/// List of currently active cashflows implementation.

impl ListCashflow {
    /// Create and return a new list cashflow.
    ///
    /// # Return
    ///
    /// * See description.   

    pub fn new() -> ListCashflow {
        ListCashflow {
            calc_manager: None,
            list_cashflow: Vec::new(),
            list_index: Cell::new(usize::MAX),
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

    /// Set the calculation manager.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.

    pub fn set_calc_mgr(&mut self, calc_manager_param: &Rc<RefCell<CalcManager>>) {
        self.calc_manager = Option::from(Rc::clone(calc_manager_param));
    }

    /// Add a new cashflow into the cashflow list.
    /// If the name results in a duplicate entry, an
    /// incrementing number starting from 2 is appended to the
    /// name until a non-duplicate entry is found.
    ///
    /// # Arguments
    ///
    /// * `name_param` - The name of the cashflow.
    /// * `list_event_param` - A newly created event list.
    /// * `elem_preferences_param` - Original existing preferences
    ///     element (or None to initialize all preferences).
    /// * `group_param` - Optional template group name.
    /// * `updating_json` - Updating from Json.
    ///
    /// # Return
    ///
    /// * ERROR_NONE if successful, otherwise error code.

    pub fn add_cashflow(
        &self,
        name_param: &str,
        list_event_param: Option<ListEvent>,
        elem_preferences_param: Option<ElemPreferences>,
        group_param: &str,
    ) -> Result<ElemCashflow, crate::ErrorType> {
        let name: String = String::from(name_param);
        let updating_json = self.calc_mgr().updating_json();

        if self.get_element_by_name(name_param, false) {
            // Check for duplicate name
            let mut name_index: usize = 2;
            loop {
                let name = format!("{}{}", name_param, name_index);
                if !self.get_element_by_name(name.as_str(), false) {
                    break;
                }
                name_index += 1;
            }
        }

        let calculate: CalcCalculate;
        let elem_preferences: Option<ElemPreferences>;

        match elem_preferences_param.as_ref() {
            None => {
                calculate = CalcCalculate::new(self.calc_manager(), None);
                elem_preferences = Option::from(ElemPreferences::new(
                    self.calc_manager(),
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
                    updating_json,
                ));
            }
            Some(o) => {
                let group: String = String::from(if group_param.is_empty() {
                    o.group()
                } else {
                    group_param
                });

                calculate =
                    CalcCalculate::new(self.calc_manager(), Option::from(o.list_descriptor()));

                elem_preferences = Option::from(ElemPreferences::new(
                    self.calc_manager(),
                    "",
                    o.cross_rate_code(),
                    o.default_encoding(),
                    group.as_str(),
                    o.fiscal_year_start(),
                    o.decimal_digits(),
                    o.combine_principal(),
                    o.compress_descriptor(),
                    o.statistic_events(),
                    Option::from(o.list_parameter()),
                    Option::from(o.list_descriptor()),
                    group_param.is_empty(),
                    updating_json,
                ));
            }
        }

        let mut list_event_opt = list_event_param;
        if list_event_opt.is_none() {
            list_event_opt = Option::from(ListEvent::new(true));
        }

        let elem_cashflow: ElemCashflow;
        match elem_preferences {
            None => {
                return Err(crate::ErrorType::Cashflow);
            }
            Some(o) => {
                elem_cashflow = ElemCashflow::new(name.as_str(), o, list_event_opt, calculate);
            }
        }

        Ok(elem_cashflow)
    }

    /// Copy the list cashflow and return a new list cashflow.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy_with_calc_manager(
        &self,
        calc_manager_param: &Rc<RefCell<CalcManager>>,
    ) -> ListCashflow {
        let mut list_cashflow = ListCashflow::new();
        list_cashflow.set_calc_mgr(calc_manager_param);

        let mut index: usize = 0;
        loop {
            if !self.get_element(index) {
                break;
            }

            let preferences: ElemPreferences;
            let group: String;

            match self.preferences() {
                None => {
                    break;
                }
                Some(o) => {
                    preferences = o.copy(true);
                    group = String::from(o.group());
                }
            }

            match self.list_event() {
                None => {
                    break;
                }
                Some(o) => {
                    let new_list_event = o.copy(true);

                    let result = list_cashflow.add_cashflow(
                        self.name(),
                        Option::from(new_list_event),
                        Option::from(preferences),
                        group.as_str(),
                    );

                    match result {
                        Err(_e) => {
                            panic!("Cannot create cashflow");
                        }
                        Ok(o) => {
                            list_cashflow.list_mut().push(o);
                            list_cashflow.sort();

                            match list_cashflow
                                .list()
                                .iter()
                                .position(|e| e.name() == self.name())
                            {
                                None => {}
                                Some(o) => {
                                    list_cashflow.set_index(o);
                                }
                            }
                        }
                    }
                }
            }

            index += 1;
        }
        list_cashflow
    }

    /// Create and return the cashflow statistics.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn create_cashflow_stats(&self) -> ElemCashflowStats {
        let elem_cashflow: &ElemCashflow;
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => {
                elem_cashflow = o;
            }
        }

        let mut current_values: usize = 0;
        let mut interest_changes: usize = 0;
        let mut principal_changes: usize = 0;
        let mut statistic_values: usize = 0;

        for elem_event in elem_cashflow.list_event().list() {
            match elem_event.elem_type() {
                crate::ExtensionType::CurrentValue => {
                    current_values += 1;
                }
                crate::ExtensionType::InterestChange => {
                    interest_changes += 1;
                }
                crate::ExtensionType::StatisticValue => {
                    statistic_values += 1;
                }
                _ => {
                    principal_changes += 1;
                }
            }
        }

        ElemCashflowStats::new(
            current_values,
            interest_changes,
            principal_changes,
            statistic_values,
        )
    }

    /// Create a cashflow for output from the currently selected cashflow.
    /// Create the cashflow by combining principal change
    /// events that are identical except their dates but maintain
    /// a periodic flow according to the original frequencies.
    ///
    /// # Arguments
    ///
    /// * `include_rollups` - Include rollup elements.
    /// * `include_details` - Include detail elements.
    /// * `compress_descriptor` - If true merge ListDescriptor
    ///     objects into a single ListDescriptor object where applicable,
    ///     otherwise do not compress amortization elements that have different
    ///     ListDescriptor objects.
    /// * `omit_statistic_events` - If true the statistic events are
    ///     eliminated from the resulting event list.
    /// * `updating_json` - Updating from Json.
    ///
    /// # Return
    ///
    /// * The resulting amortization list or an error code.

    pub fn create_cashflow_output(
        &self,
        include_rollups: bool,
        include_details: bool,
        compress_descriptor: bool,
        omit_statistic_events: bool,
        updating_json: bool,
    ) -> Result<ListAmortization, crate::ErrorType> {
        let cf_index = self.calc_mgr().list_cashflow().index();

        match self.calc_mgr().list_cashflow().list().get(cf_index) {
            None => Err(crate::ErrorType::Cashflow),
            Some(o) => {
                let result = o.calculate().create_cashflow_output(
                    o.list_amortization(),
                    include_rollups,
                    include_details,
                    compress_descriptor,
                    omit_statistic_events,
                    updating_json,
                );

                match result {
                    Err(e) => Err(e),
                    Ok(o) => Ok(o),
                }
            }
        }
    }

    /// Get the list of cashflows.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list(&self) -> &Vec<ElemCashflow> {
        &self.list_cashflow
    }

    /// Get the mut list of cashflows.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_mut(&mut self) -> &mut Vec<ElemCashflow> {
        &mut self.list_cashflow
    }

    /// Get the name of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => o.name(),
        }
    }

    /// Get the preferences element of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn preferences(&self) -> Option<&ElemPreferences> {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => Option::from(o.preferences()),
        }
    }

    /// Get the mut preferences element of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn preferences_mut(&mut self) -> Option<&mut ElemPreferences> {
        match self.list_cashflow.get_mut(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => Option::from(o.preferences_mut()),
        }
    }

    /// Get the event list of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event(&self) -> Option<&ListEvent> {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => Option::from(o.list_event()),
        }
    }

    /// Get the event list mut of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_mut(&mut self) -> Option<&mut ListEvent> {
        match self.list_cashflow.get_mut(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => Option::from(o.list_event_mut()),
        }
    }

    /// Get the amortization list of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_amortization(&self) -> Option<&ListAmortization> {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => Option::from(o.list_amortization()),
        }
    }

    /// Get the calculate object of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn calculate(&self) -> &CalcCalculate {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => o.calculate(),
        }
    }

    /// Get the statistic helper of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_statistic_helper(&self) -> Option<&ListStatisticHelper> {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => Option::from(o.list_statistic_helper()),
        }
    }

    /// Get the last balance result of the selected cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_balance_result(&self) -> Option<&ElemBalanceResult> {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => Option::from(o.elem_balance_result()),
        }
    }

    /// Get the cashflow is valid, otherwise it must be re-balanced.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_valid(&self) -> bool {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => o.cashflow_valid(),
        }
    }

    /// Get the currently selected cashflow has been updated.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn updated(&self) -> bool {
        match self.list_cashflow.get(self.list_index.get()) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => o.updated(),
        }
    }

    /// Get the name of the cashflow based upon an index value.
    /// The currently selected element is not changed.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index value of the cashflow (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_element_name(&self, index_param: usize) -> &str {
        match self.list_cashflow.get(index_param) {
            None => {
                panic!("Cashflow list index not set");
            }
            Some(o) => o.name(),
        }
    }

    /// Select a cashflow based upon a cashflow name.
    ///
    /// # Arguments
    ///
    /// * `name_param` - The name of the cashflow to select.
    /// * `is_select_param` - If true select element, otherwise restore current element.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_name(&self, name_param: &str, is_select_param: bool) -> bool {
        for (index, elem) in self.list_cashflow.iter().enumerate() {
            if name_param == elem.name() {
                if is_select_param {
                    self.set_index(index);
                }
                return true;
            }
        }

        false
    }

    /// Remove the selected cashflow from the cashflow list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_cashflow.len() {
            return false;
        }

        self.list_cashflow.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }

        true
    }

    /// Reset the updated value.

    pub fn reset_updated(&mut self) {
        match self.list_cashflow.get_mut(self.list_index.get()) {
            None => {}
            Some(o) => {
                o.set_updated(false);
            }
        }
    }

    /// Set the name of the selected cashflow.
    /// Duplicate names are not allowed.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_name(&mut self, name_param: &str) -> bool {
        if self.list_index.get() >= self.list_cashflow.len()
            || self.get_element_by_name(name_param, false)
        {
            return false;
        }

        match self.list_cashflow.get_mut(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                o.set_name(name_param);
            }
        }

        self.sort();

        match self
            .list_cashflow
            .iter()
            .position(|e| e.name() == name_param)
        {
            None => {
                return false;
            }
            Some(o) => {
                self.list_index.set(o);
            }
        }

        true
    }

    /// Set the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `statistic_helper_param` - See description.

    pub fn set_statistic_helper(&mut self, statistic_helper_param: ListStatisticHelper) -> bool {
        match self.list_cashflow.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_list_statistic_helper(statistic_helper_param);
                true
            }
        }
    }

    /// Set the balance result.
    ///
    /// # Arguments
    ///
    /// * `elem_balance_result_param` - See description.

    pub fn set_elem_balance_result(
        &mut self,
        elem_balance_result_param: ElemBalanceResult,
    ) -> bool {
        match self.list_cashflow.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_elem_balance_result(elem_balance_result_param);
                true
            }
        }
    }

    /// Set the updated value.
    ///
    /// # Arguments
    ///
    /// * `updated_param` - See description.

    pub fn set_updated_value(&mut self, updated_param: bool) -> bool {
        match self.list_cashflow.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_updated(updated_param);
                true
            }
        }
    }

    /// Update the "calculate-relative" cashflow preferences
    /// for the currently selected cashflow. Called after cash
    /// flow preferences are updated.

    pub fn update_preferences(&self) -> bool {
        match self.list().get(self.list_index.get()) {
            None => false,
            Some(o) => {
                let prefs = o.preferences();
                prefs.set_fiscal_year_start(self.calc_mgr().fiscal_year_start(true), true);
                prefs.set_decimal_digits(self.calc_mgr().decimal_digits(true), true);
                true
            }
        }
    }

    /// Sort the event list.

    pub fn sort(&mut self) {
        self.list_mut().sort_by(|a, b| ListCashflow::cmp(a, b));
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

    fn cmp(a: &ElemCashflow, b: &ElemCashflow) -> std::cmp::Ordering {
        let result = Ord::cmp(a.name(), b.name());
        if result != Equal {
            return result;
        }

        Equal
    }
}
