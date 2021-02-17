//! The event element definition.
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

pub struct ElemEvent {

  /// Event type information. 
  event_type: String,
  /// Date of the element in YYYYMMDD format (major sort key). 
  event_date: Cell<usize>,
  /// Optional date expression evaluated when created from a template. 
  date_expr: String,
  /// Sort order within the element date (minor sort key). 
  sort_order: usize,
  /// Value of the element. 
  value: Cell<Decimal>,
  /// Optional value expression evaluated when the amortization list is balanced. 
  value_expr: String,
  /// The expression is evalulated when balanced, otherwise when expanded. 
  value_expr_balance: bool,
  /// Number of periods. 
  periods: Cell<usize>,
  /// Optional periods expression evaluated when the event list is expanded. 
  periods_expr: String,
  /// Optional periods to skip length when the event list is expanded. 
  skip_mask_len: Cell<usize>,
  /// Optional periods to skip when the event list is expanded. 
  skip_mask: Cell<u128>,
  /// Intervals of frequency between periods (default 1). 
  intervals: usize,
  /// Frequency of the element. 
  frequency: crate::FrequencyType,
  /// An ElemExtension. 
  elem_extension: ElemExtension,
  /// Compressed cashflow parameter list. 
  list_parameter: Option<ListParameter>,
  /// Compressed cashflow descriptor list. 
  list_descriptor: Option<ListDescriptor>,
  /// Optional name of the event. 
  event_name: String,
  /// Optional next name of the template. 
  next_name: String

}

/// The event element extension implementation.

impl ExtensionTrait for ElemEvent {

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

/// The event element implementation.

impl ElemEvent {

  /// Create a new event cashflow element.
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
  /// * See description.

  pub fn new(event_type_param: &str, event_date_param: usize, date_expr_param: &str, 
    sort_order_param: usize, value_param: Decimal, value_expr_param: &str, 
    value_expr_balance_param: bool, periods_param: usize, periods_expr_param: &str, 
    skip_mask_len_param: usize, skip_mask_param: u128, intervals_param: usize, 
    frequency_param: crate::FrequencyType, extension_param: ElemExtension,
    list_parameter_param: Option<ListParameter>, list_descriptor_param: Option<ListDescriptor>, 
    event_name_param: &str, next_name_param: &str) -> ElemEvent {

    return ElemEvent {
      event_type: String::from(event_type_param),
      event_date: Cell::new(event_date_param),
      date_expr: String::from(date_expr_param),
      sort_order: sort_order_param,
      value: Cell::new(value_param),
      value_expr: String::from(value_expr_param),
      value_expr_balance: value_expr_balance_param,
      periods: Cell::new(periods_param),
      periods_expr: String::from(periods_expr_param),
      skip_mask_len: Cell::new(skip_mask_len_param),
      skip_mask: Cell::new(skip_mask_param),
      intervals: intervals_param,
      frequency: frequency_param,
      elem_extension: extension_param,
      list_parameter: list_parameter_param,
      list_descriptor: list_descriptor_param,
      event_name: String::from(event_name_param),
      next_name: String::from(next_name_param)
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

  /// Get the event date value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn event_date(self: &Self) -> usize {

    return self.event_date.get();
  }

  /// Get the date expression value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn date_expr(self: &Self) -> &str {

    return self.date_expr.as_str();
  }

  /// Get the sort order value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sort_order(self: &Self) -> usize {

    return self.sort_order;
  }

  /// Get the element value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value(self: &Self) -> Decimal {

    return self.value.get();
  }

  /// Get the element value expression.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value_expr(self: &Self) -> &str {

    return self.value_expr.as_str();
  }

  /// Get the element value expression balance.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value_expr_balance(self: &Self) -> bool {

    return self.value_expr_balance;
  }

  /// Get the periods value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn periods(self: &Self) -> usize {

    return self.periods.get();
  }

  /// Get the periods expression value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn periods_expr(self: &Self) -> &str {

    return self.periods_expr.as_str();
  }

  /// Get the skip mask length.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn skip_mask_len(self: &Self) -> usize {

    return self.skip_mask_len.get();
  }

  /// Get the skip mask value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn skip_mask(self: &Self) -> u128 {

    return self.skip_mask.get();
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

  /// Get the event name.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn event_name(self: &Self) -> &str {

    return self.event_name.as_str();
  }

  /// Get the next name.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn next_name(self: &Self) -> &str {

    return self.next_name.as_str();
  }

  /// Set the event type.
  /// 
  /// # Arguments
  ///
  /// * `event_type_param` - See description.

  pub fn set_event_type(self: &mut Self, event_type_param: &str) -> () {

    self.event_type = String::from(event_type_param);    
  }

  /// Set the event date.
  /// 
  /// # Arguments
  ///
  /// * `event_date_param` - See description.

  pub fn set_event_date(self: &Self, event_date_param: usize) -> () {

    self.event_date.set(event_date_param);    
  }

  /// Set the date expression.
  /// 
  /// # Arguments
  ///
  /// * `date_expr_param` - See description.

  pub fn set_date_expr(self: &mut Self, date_expr_param: &str) -> () {

    self.date_expr = String::from(date_expr_param);    
  }

  /// Set the sort order.
  /// 
  /// # Arguments
  ///
  /// * `sort_order_param` - See description.

  pub fn set_sort_order(self: &mut Self, sort_order_param: usize) -> () {

    self.sort_order = sort_order_param;    
  }

  /// Set the value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.

  pub fn set_value(self: &Self, value_param: Decimal) -> () {

    self.value.set(value_param);    
  }

  /// Set the value expression.
  /// 
  /// # Arguments
  ///
  /// * `value_expr_param` - See description.

  pub fn set_value_expr(self: &mut Self, value_expr_param: &str) -> () {

    self.value_expr = String::from(value_expr_param);    
  }

  /// Set the value expression balance.
  /// 
  /// # Arguments
  ///
  /// * `value_expr_balance_param` - See description.

  pub fn set_value_expr_balance(self: &mut Self, value_expr_balance_param: bool) -> () {

    self.value_expr_balance = value_expr_balance_param;    
  }

  /// Set the periods.
  /// 
  /// # Arguments
  ///
  /// * `periods_param` - See description.

  pub fn set_periods(self: &Self, periods_param: usize) -> () {

    self.periods.set(periods_param);    
  }

  /// Set the periods expression.
  /// 
  /// # Arguments
  ///
  /// * `periods_expr_param` - See description.

  pub fn set_periods_expr(self: &mut Self, periods_expr_param: &str) -> () {

    self.periods_expr = String::from(periods_expr_param);    
  }

  /// Set the skip mask.
  /// 
  /// # Arguments
  ///
  /// * `skip_mask_param` - See description.

  pub fn set_skip_mask(self: &Self, skip_mask_len_param: usize, skip_mask_param: u128) -> () {

    self.skip_mask_len.set(skip_mask_len_param);
    self.skip_mask.set(skip_mask_param);    
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

  /// Set the event name.
  /// 
  /// # Arguments
  ///
  /// * `event_name_param` - See description.

  pub fn set_event_name(self: &mut Self, event_name_param: &str) -> () {

    self.event_name = String::from(event_name_param);    
  }

  /// Set the next name.
  /// 
  /// # Arguments
  ///
  /// * `next_name_param` - See description.

  pub fn set_next_name(self: &mut Self, next_name_param: &str) -> () {

    self.next_name = String::from(next_name_param);    
  }
  
}