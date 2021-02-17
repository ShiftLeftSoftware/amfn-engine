//! List of active statistic elements.
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
use rust_decimal::prelude::*;

use crate::ListTrait;
use super::ElemStatisticHelper;

pub struct ListStatisticHelper {

  /// The list of statistics. 
  list_statistic_helper: Vec<ElemStatisticHelper>,

  /// The index of the currently selected statistic element. 
  list_index: Cell<usize>

}

/// List of statistic helper list implementation.

impl ListTrait for ListStatisticHelper {

  /// Clear all statistics from the statistic list.

  fn clear(self: &mut Self) -> () {
    
    self.list_statistic_helper.clear();
    self.list_index.set(usize::MAX);
  }

  /// Get the count of the statistic helper list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_statistic_helper.len();
  }

  /// Get the index of the selected statistic (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
  }

  /// Select a statistic based upon an index value.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - Index value of the statistic to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_statistic_helper.len() {
      return false;
    }

    self.set_index(index_param);

    return true;
  }

  /// Set the list index.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn set_index(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_statistic_helper.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true;
  }

}

/// List of statistic helper elements.

impl ListStatisticHelper {

  /// Create and return a new statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new () -> ListStatisticHelper {
    
    return ListStatisticHelper {
      list_statistic_helper: Vec::new(),
      list_index: Cell::new(usize::MAX)
    }
  }

  /// Add a new statistic into the statistics list.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - Name of the statistic.
  /// * `last_date_param` - The last statistic event date.
  /// * `elem_am_index_param` - Index of the amortization element.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn add_statistic_helper(self: &mut Self, name_param: &str, last_date_param: usize, 
    elem_am_index_param: usize) -> bool {

    let new_elem_stat: ElemStatisticHelper = ElemStatisticHelper::new(
      name_param, dec!(0.0), dec!(0.0), dec!(0.0), dec!(0.0), dec!(0.0), dec!(0.0), 
      last_date_param, elem_am_index_param);

    self.list_statistic_helper.push(new_elem_stat);

    match self.list_statistic_helper.iter().position(|e| e.name() == name_param) {
      None => { return false; }
      Some(o) => {
        self.list_index.set(o);
        return true;    
      }
    }
  }

  /// Performs a deep copy of this statistic helper list and returns a new statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ListStatisticHelper {
    let mut list_statistic_helper = ListStatisticHelper::new();
    let orig_index = self.index();
    let mut index: usize = 0;

    loop {

      if !self.get_element(index) { break; }

      list_statistic_helper.list_statistic_helper.push(ElemStatisticHelper::new(
        self.name(), self.principal_decrease(), self.principal_increase(),
        self.interest(), self.sl_interest(), self.value_to_interest(), 
        self.value_to_principal(), self.last_date(), self.elem_am_index()));

      index += 1;
    }

    self.set_index(orig_index);

    return list_statistic_helper;
  }

  /// Get the name of the statistic.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.name(); }
    }
  }

  /// Get the accumulated principal decrease for statistics period.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn principal_decrease(self: &Self) -> Decimal {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.principal_decrease(); }
    }
  }

  /// Get the accumulated principal increase for statistics period.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn principal_increase(self: &Self) -> Decimal {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.principal_increase(); }
    }
  }

  /// Get the accumulated compounded interest for period.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn interest(self: &Self) -> Decimal {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.interest(); }
    }
  }

  /// Get the accumulated straight-line interest for period.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn sl_interest(self: &Self) -> Decimal {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.sl_interest(); }
    }
  }

  /// Get the accumulated value to interest for period.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn value_to_interest(self: &Self) -> Decimal {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.value_to_interest(); }
    }
  }

  /// Get the accumulated value to principal for period.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn value_to_principal(self: &Self) -> Decimal {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.value_to_principal(); }
    }
  }

  /// Get the last statistic event date.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn last_date(self: &Self) -> usize {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.last_date(); }
    }
  }

  /// Get the index of the ListAmortization.ElemAmortization object.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn elem_am_index(self: &Self) -> usize {

    match self.list_statistic_helper.iter().nth(self.list_index.get()) {
      None => { panic!("Statistic helper list index not set"); }
      Some(o) => { return o.elem_am_index(); }
    }
  }

  /// Select a statistic based upon a name.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - The name of the statistic to select.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn get_element_by_name(self: &Self, name_param: &str) -> bool {

    let mut index: usize = 0;
    
    for elem in self.list_statistic_helper.iter() {
      if name_param == elem.name() {
          self.set_index(index);
          return true;
        }

      index += 1;
    }
    
    return false;
  }

  /// Increment the accumulated principal decrease for statistics period.
  /// 
  /// # Arguments
  ///
  /// * `principal_decrease_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn incr_principal_decrease(self: &mut Self, principal_decrease_param: Decimal) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_principal_decrease(o.principal_decrease() + principal_decrease_param);
        return true;
      }
    }
  }

  /// Increment the accumulated principal increase for statistics period.
  /// 
  /// # Arguments
  ///
  /// * `principal_increase_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn incr_principal_increase(self: &mut Self, principal_increase_param: Decimal) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_principal_increase(o.principal_increase() + principal_increase_param);
        return true;
      }
    }
  }

  /// Increment the accumulated compounded interest for period.
  /// 
  /// # Arguments
  ///
  /// * `interest_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn incr_interest(self: &mut Self, interest_param: Decimal) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_interest(o.interest() + interest_param);
        return true;
      }
    }
  }

  /// Increment the accumulated straight-line interest for period.
  /// 
  /// # Arguments
  ///
  /// * `sl_interest_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn incr_sl_interest(self: &mut Self, sl_interest_param: Decimal) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_sl_interest(o.sl_interest() + sl_interest_param);
        return true;
      }
    }
  }

  /// Increment the accumulated value to interest for period.
  /// 
  /// # Arguments
  ///
  /// * `value_to_interest_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn incr_value_to_interest(self: &mut Self, value_to_interest_param: Decimal) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_value_to_interest(o.value_to_interest() + value_to_interest_param);
        return true;
      }
    }
  }

  /// Increment the accumulated value to principal for period.
  /// 
  /// # Arguments
  ///
  /// * `value_to_principal_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn incr_value_to_principal(self: &mut Self, value_to_principal_param: Decimal) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_value_to_principal(o.value_to_principal() + value_to_principal_param);
        return true;
      }
    }
  }

  /// Remove the selected statistic from the statistic list.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn remove(self: &mut Self) -> bool {

    if self.list_index.get() >= self.list_statistic_helper.len() {
      return false;
    }

    self.list_statistic_helper.remove(self.list_index.get());
    
    if self.list_index.get() > 0 {
      self.list_index.set(self.list_index.get() - 1);
    }
    
    return true;
  }

  /// Reset all statistic accumulators.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn reset(self: &mut Self) -> bool {

    if self.list_index.get() >= self.list_statistic_helper.len() {
      return false;
    }
    
    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_principal_decrease(dec!(0.0));
        o.set_principal_increase(dec!(0.0));
        o.set_interest(dec!(0.0));
        o.set_sl_interest(dec!(0.0));
        o.set_value_to_interest(dec!(0.0));
        o.set_value_to_principal(dec!(0.0));    
        return true;
      }
    }
  }

  /// Set the last statistic event date.
  /// 
  /// # Arguments
  ///
  /// * `last_date_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn set_last_date(self: &mut Self, last_date_param: usize) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_last_date(last_date_param);
        return true;
      }
    }
  }

  /// Set the index of the ListAmortization.ElemAmortization object.
  /// 
  /// # Arguments
  ///
  /// * `elem_am_index_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn set_elem_am_index(self: &mut Self, elem_am_index_param: usize) -> bool {

    match self.list_statistic_helper.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_elem_am_index(elem_am_index_param);
        return true;
      }
    }
  }

}