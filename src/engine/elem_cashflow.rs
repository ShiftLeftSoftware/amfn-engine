//! The cashflow element definition.
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

use std::rc::Rc;
use std::cell::RefCell;

use crate::core::{CoreManager, ElemBalanceResult, ListEvent, ListAmortization, ListStatisticHelper};
use super::{CalcCalculate, ElemPreferences};

pub struct ElemCashflow {

  /// CoreManager element. 
  core_manager: Rc<RefCell<CoreManager>>,

  /// Name of the cashflow. 
  name: String,
  /// Cashflow preferences element. 
  preferences: ElemPreferences,
  /// Event list for the cashflow. 
  list_event: ListEvent,
  /// If true the cashflow is valid, otherwise it must be re-balanced. 
  cashflow_valid: bool,
  /// Get the currently selected cashflow has been updated. 
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
  last_amortization_index: usize
    
}

/// The cashflow element definition implementation.

impl ElemCashflow {

  /// Create and return a new cashflow element.
  /// 
  /// # Arguments
  ///
  /// * `core_manager_param` - CoreManager element.
  /// * `name_param` - Name of cashflow.
  /// * `preferences_param` - Cashflow preferences.
  /// * `list_event_param` - Event list.
  /// * `calculate_param` - Calculate element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(core_manager_param: &Rc<RefCell<CoreManager>>, name_param: &str, preferences_param: ElemPreferences,
      list_event_param: Option<ListEvent>, calculate_param: CalcCalculate) -> ElemCashflow {
    
    let list_event: ListEvent;
    match list_event_param {
      None => { list_event = ListEvent::new(core_manager_param, true); }
      Some(o) => { list_event = o; }
    }

    return ElemCashflow {
      core_manager: Rc::clone(core_manager_param),
      name: String::from(name_param),
      preferences: preferences_param,
      list_event: list_event,
      list_amortization: ListAmortization::new(),
      cashflow_valid: false,
      updated: false,
      calculate: calculate_param,
      list_statistic_helper: ListStatisticHelper::new(),
      elem_balance_result: ElemBalanceResult::new(),
      last_amortization_index: usize::MAX
    }
  }

  /// Get the name.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the preferences.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn preferences(self: &Self) -> &ElemPreferences {

    return &self.preferences;
  }

  /// Get the mut preferences.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn preferences_mut(self: &mut Self) -> &mut ElemPreferences {

    return &mut self.preferences;
  }

  /// Get the list event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_event(self: &Self) -> &ListEvent {

    return &self.list_event;
  }

  /// Get the list event mutable.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_event_mut(self: &mut Self) -> &mut ListEvent {

    return &mut self.list_event;
  }

  /// Get the list amortization.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_amortization(self: &Self) -> &ListAmortization {

    return &self.list_amortization;
  }

  /// Get the list amortization mutable.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_amortization_mut(self: &mut Self) -> &mut ListAmortization {

    return &mut self.list_amortization;
  }

  /// Get the cashflow valid.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cashflow_valid(self: &Self) -> bool {

    return self.cashflow_valid;
  }

  /// Get the updated value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn updated(self: &Self) -> bool {

    return self.updated;
  }

  /// Get the calculate object.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn calculate(self: &Self) -> &CalcCalculate {

    return &self.calculate;
  }

  /// Get the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_statistic_helper(self: &Self) -> &ListStatisticHelper {

    return &self.list_statistic_helper;
  }

  /// Get the balance result.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn elem_balance_result(self: &Self) -> &ElemBalanceResult {

    return &self.elem_balance_result;
  }

  /// Get the last amortization index.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn last_amortization_index(self: &Self) -> usize {

    return self.last_amortization_index;
  }

  /// Set the name.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - See description.
    
  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the preferences.
  /// 
  /// # Arguments
  ///
  /// * `preferences_param` - See description.
    
  pub fn set_preferences(self: &mut Self, preferences_param: ElemPreferences) -> () {

    self.preferences = preferences_param;
  }

  /// Set the list event.
  /// 
  /// # Arguments
  ///
  /// * `list_event_param` - See description.
    
  pub fn set_list_event(self: &mut Self, list_event_param: ListEvent) -> () {

    self.list_event = list_event_param;
  }

  /// Set the list amortization.
  /// 
  /// # Arguments
  ///
  /// * `list_am_param` - See description.
    
  pub fn set_list_amortization(self: &mut Self, list_am_param: ListAmortization) -> () {

    self.list_amortization = list_am_param;
  }

  /// Set the cashflow valid.
  /// 
  /// # Arguments
  ///
  /// * `cashflow_valid_param` - See description.
    
  pub fn set_cashflow_valid(self: &mut Self, cashflow_valid_param: bool) -> () {

    self.cashflow_valid = cashflow_valid_param;
  }

  /// Set the updated value.
  /// 
  /// # Arguments
  ///
  /// * `updated_param` - See description.
    
  pub fn set_updated(self: &mut Self, updated_param: bool) -> () {

    self.updated = updated_param;
  }

  /// Set the calculate object.
  /// 
  /// # Arguments
  ///
  /// * `calculate_param` - See description.
    
  pub fn set_calculate(self: &mut Self, calculate_param: CalcCalculate) -> () {

    self.calculate = calculate_param;
  }

  /// Set the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `list_statistic_helper_param` - See description.
    
  pub fn set_list_statistic_helper(self: &mut Self, statistic_helper_param: ListStatisticHelper) -> () {

    self.list_statistic_helper = statistic_helper_param;
  }

  /// Set the balance result.
  /// 
  /// # Arguments
  ///
  /// * `elem_balance_result_param` - See description.
    
  pub fn set_elem_balance_result(self: &mut Self, elem_balance_result_param: ElemBalanceResult) -> () {

    self.elem_balance_result = elem_balance_result_param;
  }

  /// Set the last amortization index.
  /// 
  /// # Arguments
  ///
  /// * `last_am_index_param` - See description.
    
  pub fn set_last_amortization_index(self: &mut Self, last_amortization_index_param: usize) -> () {

    self.last_amortization_index = last_amortization_index_param;
  }

}