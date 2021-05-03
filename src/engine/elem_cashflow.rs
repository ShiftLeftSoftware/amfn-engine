//! The cashflow element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{CalcCalculate, ElemPreferences};
use crate::core::{ElemBalanceResult, ListAmortization, ListEvent, ListStatisticHelper};

pub struct ElemCashflow {
    /// Name of the cashflow.
    name: String,
    /// Cashflow preferences element.
    preferences: ElemPreferences,
    /// Event list for the cashflow.
    list_event: ListEvent,
    /// If true the cashflow is valid, otherwise it must be re-balanced.
    cashflow_valid: bool,
    /// The currently selected cashflow has been updated.
    updated: bool,

    /// Cashflow calculation object.
    calculate: CalcCalculate,
    /// The list of active statistic elements.
    list_statistic_helper: ListStatisticHelper,
    /// Last balance calculation results.
    elem_balance_result: ElemBalanceResult,
    /// Amortization list for the cashflow.
    list_amortization: ListAmortization,
    /// The last amortization list index if valid or -1 if not valid.
    last_amortization_index: usize,
}

/// The cashflow element definition implementation.

impl ElemCashflow {
    /// Create and return a new cashflow element.
    ///
    /// # Arguments
    ///
    /// * `name_param` - Name of cashflow.
    /// * `preferences_param` - Cashflow preferences.
    /// * `list_event_param` - Event list.
    /// * `calculate_param` - Calculate element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        name_param: &str,
        preferences_param: ElemPreferences,
        list_event_param: Option<ListEvent>,
        calculate_param: CalcCalculate,
    ) -> ElemCashflow {
        let tlist_event: ListEvent;
        match list_event_param {
            None => {
                tlist_event = ListEvent::new(true);
            }
            Some(o) => {
                tlist_event = o;
            }
        }

        ElemCashflow {
            name: String::from(name_param),
            preferences: preferences_param,
            list_event: tlist_event,
            list_amortization: ListAmortization::new(),
            cashflow_valid: false,
            updated: false,
            calculate: calculate_param,
            list_statistic_helper: ListStatisticHelper::new(),
            elem_balance_result: ElemBalanceResult::new(),
            last_amortization_index: usize::MAX,
        }
    }

    /// Get the name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn preferences(&self) -> &ElemPreferences {
        &self.preferences
    }

    /// Get the mut preferences.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn preferences_mut(&mut self) -> &mut ElemPreferences {
        &mut self.preferences
    }

    /// Get the list event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event(&self) -> &ListEvent {
        &self.list_event
    }

    /// Get the list event mutable.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_mut(&mut self) -> &mut ListEvent {
        &mut self.list_event
    }

    /// Get the list amortization.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_amortization(&self) -> &ListAmortization {
        &self.list_amortization
    }

    /// Get the list amortization mutable.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_amortization_mut(&mut self) -> &mut ListAmortization {
        &mut self.list_amortization
    }

    /// Get the cashflow valid.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cashflow_valid(&self) -> bool {
        self.cashflow_valid
    }

    /// Get the updated value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn updated(&self) -> bool {
        self.updated
    }

    /// Get the calculate object.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn calculate(&self) -> &CalcCalculate {
        &self.calculate
    }

    /// Get the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_statistic_helper(&self) -> &ListStatisticHelper {
        &self.list_statistic_helper
    }

    /// Get the balance result.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_balance_result(&self) -> &ElemBalanceResult {
        &self.elem_balance_result
    }

    /// Get the last amortization index.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn last_amortization_index(&self) -> usize {
        self.last_amortization_index
    }

    /// Set the name.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.

    pub fn set_name(&mut self, name_param: &str) {
        self.name = String::from(name_param);
    }

    /// Set the preferences.
    ///
    /// # Arguments
    ///
    /// * `preferences_param` - See description.

    pub fn set_preferences(&mut self, preferences_param: ElemPreferences) {
        self.preferences = preferences_param;
    }

    /// Set the list event.
    ///
    /// # Arguments
    ///
    /// * `list_event_param` - See description.

    pub fn set_list_event(&mut self, list_event_param: ListEvent) {
        self.list_event = list_event_param;
    }

    /// Set the list amortization.
    ///
    /// # Arguments
    ///
    /// * `list_am_param` - See description.

    pub fn set_list_amortization(&mut self, list_am_param: ListAmortization) {
        self.list_amortization = list_am_param;
    }

    /// Set the cashflow valid.
    ///
    /// # Arguments
    ///
    /// * `cashflow_valid_param` - See description.

    pub fn set_cashflow_valid(&mut self, cashflow_valid_param: bool) {
        self.cashflow_valid = cashflow_valid_param;
    }

    /// Set the updated value.
    ///
    /// # Arguments
    ///
    /// * `updated_param` - See description.

    pub fn set_updated(&mut self, updated_param: bool) {
        self.updated = updated_param;
    }

    /// Set the calculate object.
    ///
    /// # Arguments
    ///
    /// * `calculate_param` - See description.

    pub fn set_calculate(&mut self, calculate_param: CalcCalculate) {
        self.calculate = calculate_param;
    }

    /// Set the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `list_statistic_helper_param` - See description.

    pub fn set_list_statistic_helper(&mut self, statistic_helper_param: ListStatisticHelper) {
        self.list_statistic_helper = statistic_helper_param;
    }

    /// Set the balance result.
    ///
    /// # Arguments
    ///
    /// * `elem_balance_result_param` - See description.

    pub fn set_elem_balance_result(&mut self, elem_balance_result_param: ElemBalanceResult) {
        self.elem_balance_result = elem_balance_result_param;
    }

    /// Set the last amortization index.
    ///
    /// # Arguments
    ///
    /// * `last_am_index_param` - See description.

    pub fn set_last_amortization_index(&mut self, last_amortization_index_param: usize) {
        self.last_amortization_index = last_amortization_index_param;
    }
}
