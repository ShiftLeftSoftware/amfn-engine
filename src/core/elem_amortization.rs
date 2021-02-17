//! The amortization cashflow element.
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

use crate::ExtensionTrait;
use super::{ElemExtension, ListDescriptor, ListParameter};

pub struct ElemAmortization {

  /// Event type information. 
  event_type: String,
  /// Date of the original event in YYYYMMDD format. 
  orig_date: usize,
  /// Date of the element in YYYYMMDD format (major sort key). 
  event_date: usize,
  /// Sort order within the element date (minor sort key). 
  sort_order: usize,
  /// Value of the original event. 
  orig_value: Decimal,
  /// Value of the element. 
  value: Decimal,
  /// Optional value expression evaluated when the amortization list is balanced. 
  value_expr: String,
  /// Number of periods. 
  periods: usize,
  /// Intervals of frequency between periods (default 1). 
  intervals: usize,
  /// Frequency of the element. 
  frequency: crate::FrequencyType,
  /// Principal decrease. 
  principal_decrease: Decimal,
  /// Principal increase. 
  principal_increase: Decimal,
  /// Compounded interest. 
  interest: Decimal,
  /// Straight-line interest. 
  sl_interest: Decimal,
  /// Value to interest. 
  value_to_interest: Decimal,
  /// Value to principal. 
  value_to_principal: Decimal,
  /// Accrued interest balance at start of element. 
  acc_balance: Decimal,
  /// Beginning active balance at start of element. 
  balance: Decimal,
  /// Index of the original event within the event list. 
  list_event_index: Cell<usize>,
  /// Sequence number within the event starting from 1. 
  event_sequence: usize,
  /// Sequence number of principal change with statistics set or 0 (if not applicable). 
  stat_sequence: usize,
  /// An ElemExtension. 
  elem_extension: ElemExtension,
  /// Compressed cashflow parameter list. 
  list_parameter: Option<ListParameter>,
  /// Compressed cashflow descriptor list. 
  list_descriptor: Option<ListDescriptor>

}

/// The amortization cashflow extension implementation.

impl ExtensionTrait for ElemAmortization {

  /// Get the element type value.
  /// 
  /// # Return
  ///
  /// * See description.

  fn elem_type(self: &Self) -> crate::ExtensionType {

    return self.elem_extension.extension_type();
  }

  /// Get the extension.
  /// 
  /// # Return
  ///
  /// * See description.

  fn elem_extension(self: &Self) -> &ElemExtension {

    return &self.elem_extension;
  }

  /// Get the mut extension.
  /// 
  /// # Return
  ///
  /// * See description.

  fn elem_extension_mut(self: &mut Self) -> &mut ElemExtension {

    return &mut self.elem_extension;
  }

  /// Set the extension.
  /// 
  /// # Arguments
  ///
  /// * `elem_extension_param` - See description.

  fn set_elem_extension(self: &mut Self, elem_extension_param: ElemExtension) -> () {

    self.elem_extension = elem_extension_param;    
  }

}

/// The amortization cashflow implementation.

impl ElemAmortization {

  /// Create a new amortization cashflow element.
  /// 
  /// # Arguments
  ///
  /// * `event_type_param` - Event type.
  /// * `orig_date_param` - Original date.
  /// * `event_date_param` - Event date.
  /// * `sort_order_param` - Sort order.
  /// * `orig_value_param` - Original value paremeter.
  /// * `value_param` - Value parameter.
  /// * `value_expr_param` - Value expression parameter.
  /// * `periods_param` - Periods parameter.
  /// * `intervals_param` - Intervals parameter.
  /// * `frequency_param` - Frequency parameter.
  /// * `principal_decrease_param` - Principal decrease.
  /// * `principal_increase_param` - Principal increase.
  /// * `interest_param` - Interest parameter.
  /// * `sl_interest_param` - Straight line interest.
  /// * `value_to_interest_param` - Value to interest.
  /// * `value_to_principal_param` - Value to principal.
  /// * `acc_balance_param` - Accrued balance.
  /// * `balance_param` - Balance.
  /// * `list_event_index_param` - List event index.
  /// * `event_sequence_param` - Event sequence.
  /// * `stat_sequence_param` - Statistics sequence.
  /// * `extension_param` - Extension (current value, interest change, principal change, statistic value).
  /// * `list_parameter_param` - List parameter.
  /// * `list_descriptor_param` - List descriptor.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(event_type_param: &str, orig_date_param: usize, event_date_param: usize, sort_order_param: usize,
    orig_value_param: Decimal, value_param: Decimal, value_expr_param: &str, periods_param: usize, 
    intervals_param: usize, frequency_param: crate::FrequencyType, principal_decrease_param: Decimal, 
    principal_increase_param: Decimal, interest_param: Decimal, sl_interest_param: Decimal, 
    value_to_interest_param: Decimal, value_to_principal_param: Decimal, acc_balance_param: Decimal, 
    balance_param: Decimal, list_event_index_param: usize, event_sequence_param: usize, 
    stat_sequence_param: usize, extension_param: ElemExtension, list_parameter_param: Option<ListParameter>, 
    list_descriptor_param: Option<ListDescriptor>) -> ElemAmortization {

    return ElemAmortization {
      event_type: String::from(event_type_param),
      orig_date: orig_date_param,
      event_date: event_date_param,
      sort_order: sort_order_param,
      orig_value: orig_value_param,
      value: value_param,
      value_expr: String::from(value_expr_param),
      periods: periods_param,
      intervals: intervals_param,
      frequency: frequency_param,
      principal_decrease: principal_decrease_param,
      principal_increase: principal_increase_param,
      interest: interest_param,
      sl_interest: sl_interest_param,
      value_to_interest: value_to_interest_param,
      value_to_principal: value_to_principal_param,
      acc_balance: acc_balance_param,
      balance: balance_param,
      list_event_index: Cell::new(list_event_index_param),
      event_sequence: event_sequence_param,
      stat_sequence: stat_sequence_param,
      elem_extension: extension_param,
      list_parameter: list_parameter_param,
      list_descriptor: list_descriptor_param
    }

  }

  /// Get the event type value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn event_type(self: &Self) -> &str {

    return self.event_type.as_str();
  }

  /// Get the event original date value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn orig_date(self: &Self) -> usize {

    return self.orig_date;
  }

  /// Get the event date value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn event_date(self: &Self) -> usize {

    return self.event_date;
  }

  /// Get the sort order value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sort_order(self: &Self) -> usize {

    return self.sort_order;
  }

  /// Get the original element value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn orig_value(self: &Self) -> Decimal {

    return self.orig_value;
  }

  /// Get the element value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value(self: &Self) -> Decimal {

    return self.value;
  }

  /// Get the element value expression.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value_expr(self: &Self) -> &str {

    return self.value_expr.as_str();
  }

  /// Get the periods value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn periods(self: &Self) -> usize {

    return self.periods;
  }

  /// Get the intervals value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn intervals(self: &Self) -> usize {

    return self.intervals;
  }

  /// Get the frequency value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn frequency(self: &Self) -> crate::FrequencyType {

    return self.frequency;
  }

  /// Get the principal decrease value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn principal_decrease(self: &Self) -> Decimal {

    return self.principal_decrease;
  }

  /// Get the principal increase value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn principal_increase(self: &Self) -> Decimal {

    return self.principal_increase;
  }

  /// Get the interest value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn interest(self: &Self) -> Decimal {

    return self.interest;
  }

  /// Get the straight line interest value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sl_interest(self: &Self) -> Decimal {

    return self.sl_interest;
  }

  /// Get the value to interest.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value_to_interest(self: &Self) -> Decimal {

    return self.value_to_interest;
  }

  /// Get the value to principal.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value_to_principal(self: &Self) -> Decimal {

    return self.value_to_principal;
  }

  /// Get the accrued balance value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn acc_balance(self: &Self) -> Decimal {

    return self.acc_balance;
  }

  /// Get the balance value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn balance(self: &Self) -> Decimal {

    return self.balance;
  }

  /// Get the list event index.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_event_index(self: &Self) -> usize {

    return self.list_event_index.get();
  }

  /// Get the event sequence.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn event_sequence(self: &Self) -> usize {

    return self.event_sequence;
  }

  /// Get the statistics sequence.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn stat_sequence(self: &Self) -> usize {

    return self.stat_sequence;
  }

  /// Get the list parameter.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_parameter(self: &Self) -> Option<&ListParameter> {

    return Option::from(self.list_parameter.as_ref());
  }

  /// Get the list descriptor.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_descriptor(self: &Self) -> Option<&ListDescriptor> {

    return Option::from(self.list_descriptor.as_ref());
  }

  /// Set the event type.
  /// 
  /// # Arguments
  ///
  /// * `event_type_param` - See description.

  pub fn set_event_type(self: &mut Self, event_type_param: &str) -> () {

    self.event_type = String::from(event_type_param);    
  }

  /// Set the original date.
  /// 
  /// # Arguments
  ///
  /// * `original_date_param` - See description.

  pub fn set_original_date(self: &mut Self, original_date_param: usize) -> () {

    self.orig_date = original_date_param;    
  }

  /// Set the event date.
  /// 
  /// # Arguments
  ///
  /// * `event_date_param` - See description.

  pub fn set_event_date(self: &mut Self, event_date_param: usize) -> () {

    self.event_date = event_date_param;    
  }

  /// Set the sort order.
  /// 
  /// # Arguments
  ///
  /// * `sort_order_param` - See description.

  pub fn set_sort_order(self: &mut Self, sort_order_param: usize) -> () {

    self.sort_order = sort_order_param;    
  }

  /// Set the original value.
  /// 
  /// # Arguments
  ///
  /// * `orig_value_param` - See description.

  pub fn set_orig_value(self: &mut Self, orig_value_param: Decimal) -> () {

    self.orig_value = orig_value_param;    
  }

  /// Set the value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.

  pub fn set_value(self: &mut Self, value_param: Decimal) -> () {

    self.value = value_param;    
  }

  /// Set the value expression.
  /// 
  /// # Arguments
  ///
  /// * `value__expr_param` - See description.

  pub fn set_value_expr(self: &mut Self, value_expr_param: &str) -> () {

    self.value_expr = String::from(value_expr_param);    
  }

  /// Set the periods.
  /// 
  /// # Arguments
  ///
  /// * `periods_param` - See description.

  pub fn set_periods(self: &mut Self, periods_param: usize) -> () {

    self.periods = periods_param;    
  }

  /// Set the intervals.
  /// 
  /// # Arguments
  ///
  /// * `intervals_param` - See description.

  pub fn set_intervals(self: &mut Self, intervals_param: usize) -> () {

    self.intervals = intervals_param;    
  }

  /// Set the frequency.
  /// 
  /// # Arguments
  ///
  /// * `frequency_param` - See description.

  pub fn set_frequency(self: &mut Self, frequency_param: crate::FrequencyType) -> () {

    self.frequency = frequency_param;    
  }

  /// Set the principal decrease.
  /// 
  /// # Arguments
  ///
  /// * `principal_decrease_param` - See description.

  pub fn set_principal_decrease(self: &mut Self, principal_decrease_param: Decimal) -> () {

    self.principal_decrease = principal_decrease_param;    
  }

  /// Set the principal increase value.
  /// 
  /// # Arguments
  ///
  /// * `principal_increase_param` - See description.

  pub fn set_principal_increase(self: &mut Self, principal_increase_param: Decimal) -> () {

    self.principal_increase = principal_increase_param;    
  }

  /// Set the interest value.
  /// 
  /// # Arguments
  ///
  /// * `interest_param` - See description.

  pub fn set_interest(self: &mut Self, interest_param: Decimal) -> () {

    self.interest = interest_param;    
  }

  /// Set the straight line interest value.
  /// 
  /// # Arguments
  ///
  /// * `sl_interest_param` - See description.

  pub fn set_sl_interest(self: &mut Self, sl_interest_param: Decimal) -> () {

    self.sl_interest = sl_interest_param;    
  }

  /// Set the value to interest.
  /// 
  /// # Arguments
  ///
  /// * `value_to_interest_param` - See description.

  pub fn set_value_to_interest(self: &mut Self, value_to_interest_param: Decimal) -> () {

    self.value_to_interest = value_to_interest_param;    
  }

  /// Set the value to principal.
  /// 
  /// # Arguments
  ///
  /// * `value_to_principal_param` - See description.

  pub fn set_value_to_principal(self: &mut Self, value_to_principal_param: Decimal) -> () {

    self.value_to_principal = value_to_principal_param;    
  }

  /// Set the accrued balance value.
  /// 
  /// # Arguments
  ///
  /// * `acc_balance_param` - See description.

  pub fn set_acc_balance(self: &mut Self, acc_balance_param: Decimal) -> () {

    self.acc_balance = acc_balance_param;    
  }

  /// Set the balance value.
  /// 
  /// # Arguments
  ///
  /// * `balance_param` - See description.

  pub fn set_balance(self: &mut Self, balance_param: Decimal) -> () {

    self.balance = balance_param;    
  }

  /// Set the list event index.
  /// 
  /// # Arguments
  ///
  /// * `list_event_index_param` - See description.

  pub fn set_list_event_index(self: &Self, list_event_index_param: usize) -> () {

    self.list_event_index.set(list_event_index_param);    
  }

  /// Set the event sequence.
  /// 
  /// # Arguments
  ///
  /// * `event_sequence_param` - See description.

  pub fn set_event_sequence(self: &mut Self, event_sequence_param: usize) -> () {

    self.event_sequence = event_sequence_param;    
  }

  /// Set the statistics sequence.
  /// 
  /// # Arguments
  ///
  /// * `stat_sequence_param` - See description.

  pub fn set_stat_sequence(self: &mut Self, stat_sequence_param: usize) -> () {

    self.stat_sequence = stat_sequence_param;    
  }
   
  /// Set the list parameter.
  /// 
  /// # Arguments
  ///
  /// * `list_parameter_param` - See description.

  pub fn set_list_parameter(self: &mut Self, list_parameter_param: Option<ListParameter>) -> () {

    self.list_parameter = list_parameter_param;    
  }

  /// Set the list descriptor.
  /// 
  /// # Arguments
  ///
  /// * `list_descriptor_param` - See description.

  pub fn set_list_descriptor(self: &mut Self, list_descriptor_param: Option<ListDescriptor>) -> () {

    self.list_descriptor = list_descriptor_param;    
  }
  
}
