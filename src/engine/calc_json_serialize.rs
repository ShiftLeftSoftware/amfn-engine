//! The serilaize json element of the AmFn engine.
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
use std::cell::{Cell, Ref, RefCell};
use rust_decimal::prelude::*;

use crate::ListTrait;
use crate::core::{CoreUtility, ExtensionValue, ElemBalanceResult, ElemCurrentValue, ElemInterestChange, 
  ElemPrincipalChange, ElemStatisticValue, ListDescriptor, ListParameter, ListEvent, ListAmortization};
use super::{CalcManager, ElemPreferences, ListCashflow, ListExchangeRate, ListTemplateEvent, ListTemplateGroup};

pub struct CalcJsonSerialize {

  /// Calculator manager element. 
  calc_manager: Rc<RefCell<CalcManager>>,

  /// Active Json Element depth 
  depth: Cell<usize>

}

/// The serilaize json implementation of the AmFn engine.

impl CalcJsonSerialize {

  /// Create and return a new serialization element.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(calc_manager_param: &Rc<RefCell<CalcManager>>) -> CalcJsonSerialize {

    return CalcJsonSerialize {
      calc_manager: Rc::clone(calc_manager_param),
      depth: Cell::new(0)
    }
  }

  /// Returns the calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.

  fn calc_reg(self: &Self) -> Ref<CalcManager> {

    return self.calc_manager.borrow();
  }

  /// Serialize elements into Json.
  /// 
  /// # Arguments
  ///
  /// * `options` - Determines the elements that are serialized.
  /// 
  /// # Return
  ///
  /// * ERROR_NONE if successful, otherwise error code.

  pub fn serialize(self: &Self, options: usize) -> String {

    let mut buf = String::from("");

    self.depth.set(0);

    buf.push_str("{");
    buf.push_str(crate::LINE_ENDING);    
    self.increment_depth(); 

    let mut add_comma = options & crate::JSON_SERIALIZE_EXCHANGE_RATES != 0 || 
      options & crate::JSON_SERIALIZE_PREFERENCES != 0 ||
      options & crate::JSON_SERIALIZE_TEMPLATES != 0;

    let is_am_list = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST != 0 ||
      options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS != 0 ||
      options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

    if (options & crate::JSON_SERIALIZE_CASHFLOW_PREFERENCES != 0) ||
        (options & crate::JSON_SERIALIZE_EVENT_LIST != 0) || is_am_list {
      self.serialize_cashflows(self.calc_reg().list_cashflow(), &mut buf, options, add_comma);
    }

    add_comma = options & crate::JSON_SERIALIZE_PREFERENCES != 0 ||
      options & crate::JSON_SERIALIZE_TEMPLATES != 0;

    if options & crate::JSON_SERIALIZE_EXCHANGE_RATES != 0 {
      self.serialize_exchange_rates(self.calc_reg().list_exchange_rate(), &mut buf, add_comma);
    }

    add_comma = options & crate::JSON_SERIALIZE_TEMPLATES != 0;

    if options & crate::JSON_SERIALIZE_PREFERENCES != 0 {
      self.serialize_preferences(self.calc_reg().preferences(), &mut buf, add_comma);
    }

    if options & crate::JSON_SERIALIZE_TEMPLATES != 0 {      
      self.serialize_template_groups(self.calc_reg().list_template_group(), &mut buf, false);
    }

    self.decrement_depth();     
    buf.push_str("}");
    buf.push_str(crate::LINE_ENDING);

    return buf;
  }

  /// Serialize list of amortization elements from the 
  /// currently selected cashflow.
  /// 
  /// # Arguments
  ///
  /// * `include_rollups` - Include rollup elements.
  /// * `include_details` - Include detail elements (if include_rollups is true).
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_am_list(self: &Self, include_rollups: bool, 
    include_details: bool, buf: &mut String, add_comma: bool) -> () {

    let calc_reg = self.calc_reg();
    let decimal_digits = calc_reg.decimal_digits(true);
    let reg = calc_reg.reg();
    let list_locale = reg.list_locale();

    let compress_descriptor = calc_reg.compress_descriptor(true);
    let omit_statistic_events = calc_reg.statistic_events(true);

    let list_am: ListAmortization;
    match calc_reg.list_cashflow().create_cashflow_output(
        include_rollups, include_details, 
        compress_descriptor, omit_statistic_events, true) {
      Err(_e) => { panic!("Cannot create amortization list for output"); }
      Ok(o) => { list_am = o; }
    }

    buf.push_str(self.indent().as_str());
    buf.push_str("\"am-list\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if list_am.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {
    
        let ext = list_am.elem_extension().extension_value();

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        if list_am.event_type().len() > 0 {
          buf.push_str(self.indent().as_str());
          buf.push_str("\"event-type\": \"");
          buf.push_str(list_am.event_type());
          buf.push_str("\",");
          buf.push_str(crate::LINE_ENDING);
        }
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"date\": \"");
        buf.push_str(list_locale.format_date(list_am.event_date()).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"frequency\": \"");
        buf.push_str(CoreUtility::get_frequency_mnemonic(list_am.frequency()).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"intervals\": ");
        buf.push_str(list_locale.format_integer(list_am.intervals()).as_str());
        buf.push_str(",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"periods\": ");
        buf.push_str(list_locale.format_integer(list_am.periods()).as_str());
        buf.push_str(",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"value\": \"");

        match ext {
          ExtensionValue::InterestChange(_o) => { 
            buf.push_str(list_locale.format_decimal(list_am.value()).as_str());
          }
          _ => { 
            buf.push_str(list_locale.format_currency(list_am.value(), decimal_digits).as_str());
          }
        }                

        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"value-to-interest\": \"");
        buf.push_str(list_locale.format_currency(list_am.value_to_interest(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"value-to-principal\": \"");
        buf.push_str(list_locale.format_currency(list_am.value_to_principal(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-decrease\": \"");
        buf.push_str(list_locale.format_currency(list_am.principal_decrease(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"principal-increase\": \"");
        buf.push_str(list_locale.format_currency(list_am.principal_increase(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"interest\": \"");
        buf.push_str(list_locale.format_currency(list_am.interest(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"sl-interest\": \"");
        buf.push_str(list_locale.format_currency(list_am.sl_interest(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"accrued-balance\": \"");
        buf.push_str(list_locale.format_currency(list_am.acc_balance(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"balance\": \"");
        buf.push_str(list_locale.format_currency(list_am.balance(), decimal_digits).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"sort-order\": ");
        buf.push_str(list_locale.format_integer(list_am.sort_order()).as_str());
        buf.push_str(",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"extension\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        match ext {
          ExtensionValue::CurrentValue(o) => { 
            self.serialize_current_value(o, buf); 
          }
          ExtensionValue::InterestChange(o) => { 
            self.serialize_interest_change(o, buf, list_am.value(), list_am.frequency()); 
          }
          ExtensionValue::PrincipalChange(o) => { 
            self.serialize_principal_change(o, buf); 
          }
          ExtensionValue::StatisticValue(o) => { 
            self.serialize_statistic_value(o, buf); 
          }
        }                

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("},");
        buf.push_str(crate::LINE_ENDING);

        match list_am.list_descriptor() {
          None => { }
          Some(o) => {
            self.serialize_descriptor_list(o, buf, false);
          }
        }
    
        index += 1;
        deserialize_list = list_am.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize balance result element.
  /// 
  /// # Arguments
  ///
  /// * `balance_result` - Balance result to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_balance_result(self: &Self,
    balance_result: &ElemBalanceResult, buf: &mut String, add_comma: bool) -> () {

    let calc_reg = self.calc_reg();
    let decimal_digits = calc_reg.decimal_digits(true);
    let reg = calc_reg.reg();
    let list_locale = reg.list_locale();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"balance-result\": {");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"accrued-balance-seen\": ");
    buf.push_str(self.get_bool_str(balance_result.acc_balance_seen()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"polarity\": \"");
    buf.push_str(self.get_polarity(balance_result.polarity()));
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"rule-of-78-seen\": ");
    buf.push_str(self.get_bool_str(balance_result.rule_of_78_seen()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"auxiliary-active-decrease\": \"");
    buf.push_str(list_locale.format_currency(balance_result.aux_active_decrease(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"auxiliary-active-increase\": \"");
    buf.push_str(list_locale.format_currency(balance_result.aux_active_increase(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"auxiliary-passive-decrease\": \"");
    buf.push_str(list_locale.format_currency(balance_result.aux_passive_decrease(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"auxiliary-passive-increase\": \"");
    buf.push_str(list_locale.format_currency(balance_result.aux_passive_increase(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"final-accrued-balance\": \"");
    buf.push_str(list_locale.format_currency(balance_result.acc_balance(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"final-balance\": \"");
    buf.push_str(list_locale.format_currency(balance_result.balance(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"final-balance-date\": \"");
    buf.push_str(list_locale.format_date(balance_result.balance_date()).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"interest-present\": \"");
    buf.push_str(list_locale.format_currency(balance_result.interest_present(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"interest-total\": \"");
    buf.push_str(list_locale.format_currency(balance_result.interest_total(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"sl-interest-present\": \"");
    buf.push_str(list_locale.format_currency(balance_result.sl_interest_present(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"sl-interest-total\": \"");
    buf.push_str(list_locale.format_currency(balance_result.sl_interest_total(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"principal-changes-present\": ");
    buf.push_str(list_locale.format_integer(balance_result.prin_present()).as_str());
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"principal-changes-total\": ");
    buf.push_str(list_locale.format_integer(balance_result.prin_total()).as_str());
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"principal-total-decrease\": \"");
    buf.push_str(list_locale.format_currency(balance_result.prin_decrease(), decimal_digits).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"principal-total-increase\": \"");
    buf.push_str(list_locale.format_currency(balance_result.prin_increase(), decimal_digits).as_str());
    buf.push_str("\"");
    buf.push_str(crate::LINE_ENDING);

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("}");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);    
  }

  /// Serialize list of cashflows.
  /// 
  /// # Arguments
  ///
  /// * `cashflows` - List of cashflows to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `options` - Serialization options.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_cashflows(self: &Self, cashflows: &ListCashflow, 
    buf: &mut String, options: usize, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"cashflows\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if cashflows.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"name\": \"");
        buf.push_str(cashflows.name());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        if options & crate::JSON_SERIALIZE_CASHFLOW_PREFERENCES != 0 {
          match cashflows.preferences() {
            None => { }
            Some(o) => { self.serialize_preferences(o, buf, true); }
          }
        }

        let is_am_list = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST != 0 ||
          options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS != 0 ||
          options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

        let is_am_rollups = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS != 0 ||
          options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

        let is_am_details = options & crate::JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS != 0;

        if options & crate::JSON_SERIALIZE_EVENT_LIST != 0 {
          match cashflows.list_event() {
            None => { }
            Some(o) => { self.serialize_event_list(o, buf, is_am_list); }
          }
        }

        if is_am_list {

          self.serialize_am_list(is_am_rollups, is_am_details, buf, true);

          match cashflows.elem_balance_result() {
            None => { }
            Some(o) => { self.serialize_balance_result(o, buf, false); }
          }
        }
    
        index += 1;

        deserialize_list = cashflows.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize current value element.
  /// 
  /// # Arguments
  ///
  /// * `current_value` - Current value element.
  /// * `buf` - Buffer to append serialization.

  fn serialize_current_value(self: &Self, current_value: &ElemCurrentValue, buf: &mut String) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"current-value\": {");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"passive\": ");
    buf.push_str(self.get_bool_str(current_value.passive()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"present\": ");
    buf.push_str(self.get_bool_str(current_value.present()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"eom\": ");
    buf.push_str(self.get_bool_str(current_value.eom()));
    buf.push_str(crate::LINE_ENDING);

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("}");
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize list of descriptors.
  /// 
  /// # Arguments
  ///
  /// * `list_descriptor` - List of descriptors to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_descriptor_list(
    self: &Self, list_descriptor: &ListDescriptor, buf: &mut String, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"descriptor-list\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if list_descriptor.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"group\": \"");
        buf.push_str(list_descriptor.group());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"name\": \"");
        buf.push_str(list_descriptor.name());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"descriptor-type\": \"");
        buf.push_str(list_descriptor.desc_type());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"descriptor-code\": \"");
        buf.push_str(list_descriptor.code());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"value\": \"");
        buf.push_str(self.escape_string(list_descriptor.value().as_str()).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        if list_descriptor.value_expr().len() > 0 {
          buf.push_str(self.indent().as_str());
          buf.push_str("\"expression\": \"");
          buf.push_str(self.escape_string(list_descriptor.value_expr().as_str()).as_str());
          buf.push_str("\",");
          buf.push_str(crate::LINE_ENDING);
        }
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"event-index\": ");
        buf.push_str(list_descriptor.index().to_string().as_str());
        buf.push_str(",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"propagate\": ");
        buf.push_str(self.get_bool_str(list_descriptor.propagate()));
        buf.push_str(crate::LINE_ENDING);
    
        index += 1;
        deserialize_list = list_descriptor.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize list of event elements.
  /// 
  /// # Arguments
  ///
  /// * `list_event` - List of event elements to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_event_list(
    self: &Self, list_event: &ListEvent, buf: &mut String, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"event-list\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if list_event.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
        
        let event_date = list_event.event_date();
        let mut end_date = event_date;
        let periods = list_event.periods();
        if periods > 1 {
          end_date = CoreUtility::date_new(
            event_date, event_date,
            list_event.frequency(), list_event.intervals() * (periods - 1), list_event.eom());        
        }
    
        if list_event.event_name().len() > 0 {
          buf.push_str(self.indent().as_str());
          buf.push_str("\"event-name\": \"");
          buf.push_str(list_event.event_name());
          buf.push_str("\",");
          buf.push_str(crate::LINE_ENDING);
        }
    
        if list_event.event_type().len() > 0 {
          buf.push_str(self.indent().as_str());
          buf.push_str("\"event-type\": \"");
          buf.push_str(list_event.event_type());
          buf.push_str("\",");
          buf.push_str(crate::LINE_ENDING);
        }
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"event-date\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"date\": \"");
        buf.push_str(self.get_date_str(event_date).as_str());
        if list_event.date_expr().len() > 0 { buf.push_str("\","); } else { buf.push_str("\""); }
        buf.push_str(crate::LINE_ENDING);
  
        if list_event.date_expr().len() > 0 {
          buf.push_str(self.indent().as_str());
          buf.push_str("\"expression\": \"");
          buf.push_str(self.escape_string(list_event.date_expr()).as_str());
          buf.push_str("\"");
          buf.push_str(crate::LINE_ENDING);
          }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("},");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"end-date\": \"");
        buf.push_str(self.get_date_str(end_date).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"frequency\": \"");
        buf.push_str(CoreUtility::get_frequency_mnemonic(list_event.frequency()).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"intervals\": ");
        buf.push_str(list_event.intervals().to_string().as_str());
        buf.push_str(",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"event-periods\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"periods\": ");
        buf.push_str(list_event.periods().to_string().as_str());
        if list_event.periods_expr().len() > 0 { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
    
        if list_event.periods_expr().len() > 0 {
          buf.push_str(self.indent().as_str());
          buf.push_str("\"expression\": \"");
          buf.push_str(self.escape_string(list_event.periods_expr()).as_str());
          buf.push_str("\"");
          buf.push_str(crate::LINE_ENDING);
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("},");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"event-value\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"value\": \"");
        buf.push_str(list_event.value().to_string().as_str());
        if list_event.value_expr().len() > 0 { buf.push_str("\","); } else { buf.push_str("\""); }
        buf.push_str(crate::LINE_ENDING);
    
        if list_event.value_expr().len() > 0 {
          buf.push_str(self.indent().as_str());
          buf.push_str("\"expression\": \"");
          buf.push_str(self.escape_string(list_event.value_expr()).as_str());
          buf.push_str("\",");
          buf.push_str(crate::LINE_ENDING);

          buf.push_str(self.indent().as_str());
          buf.push_str("\"expr-balance\": ");
          buf.push_str(self.get_bool_str(list_event.value_expr_balance()));
          buf.push_str(crate::LINE_ENDING);
        }

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("},");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"skip-mask\": \"");
        buf.push_str(CoreUtility::skip_mask_to_string(
          list_event.skip_mask_len(), list_event.skip_mask()).as_str());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"sort-order\": ");
        buf.push_str(list_event.sort_order().to_string().as_str());
        buf.push_str(",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"extension\": {");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        let ext = list_event.elem_extension().extension_value();

        match ext {
          ExtensionValue::CurrentValue(o) => { 
            self.serialize_current_value(o, buf); 
          }
          ExtensionValue::InterestChange(o) => { 
            self.serialize_interest_change(o, buf, dec!(0.0), crate::FrequencyType::OneMonth); 
          }
          ExtensionValue::PrincipalChange(o) => { 
            self.serialize_principal_change(o, buf); 
          }
          ExtensionValue::StatisticValue(o) => { 
            self.serialize_statistic_value(o, buf); 
          }
        }                

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("},");
        buf.push_str(crate::LINE_ENDING);

        match list_event.list_parameter() {
          None => { }
          Some(o) => {
            self.serialize_parameter_list(o, buf, true);
          }
        }

        match list_event.list_descriptor() {
          None => { }
          Some(o) => {
            self.serialize_descriptor_list(o, buf, true);
          }
        }
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"event-next-name\": \"");
        buf.push_str(list_event.next_name());
        buf.push_str("\"");
        buf.push_str(crate::LINE_ENDING);
    
        index += 1;
        deserialize_list = list_event.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize list of exchange rates.
  /// 
  /// # Arguments
  ///
  /// * `exchange_rates` - List of exchange rates to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_exchange_rates(
    self: &Self, exchange_rates: &ListExchangeRate, buf: &mut String, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"exchange-rates\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if exchange_rates.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"from\": \"");
        buf.push_str(exchange_rates.from_code());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"to\": \"");
        buf.push_str(exchange_rates.to_code());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        buf.push_str(self.indent().as_str());
        buf.push_str("\"value\": \"");
        buf.push_str(exchange_rates.exchange_rate().to_string().as_str());
        buf.push_str("\"");
        buf.push_str(crate::LINE_ENDING);
    
        index += 1;
        deserialize_list = exchange_rates.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize interest change element.
  /// 
  /// # Arguments
  ///
  /// * `interest_change` - Interest change element.
  /// * `buf` - Buffer to append serialization.
  /// * `nar` - Nominal annual rate.
  /// * `frequency` - Frequency value.

  fn serialize_interest_change(
    self: &Self, interest_change: &ElemInterestChange, buf: &mut String, nar: Decimal, 
    frequency: crate::FrequencyType) -> () {
    
    let calc_reg = self.calc_reg();
    let decimal_digits = calc_reg.decimal_digits(false);
    let reg = calc_reg.reg();
    let list_locale = reg.list_locale();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"interest-change\": {");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"interest-method\": \"");
    buf.push_str(CoreUtility::get_interest_method_mnemonic(interest_change.method()).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"day-count-basis\": \"");
    buf.push_str(CoreUtility::get_day_count_basis_mnemonic(interest_change.day_count_basis()).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    if interest_change.effective_frequency() != crate::FrequencyType::None {
      buf.push_str(self.indent().as_str());
      buf.push_str("\"effective-frequency\": \"");
      buf.push_str(CoreUtility::get_frequency_mnemonic(interest_change.effective_frequency()).as_str());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
    }

    if interest_change.interest_frequency() != crate::FrequencyType::None {
      buf.push_str(self.indent().as_str());
      buf.push_str("\"interest-frequency\": \"");
      buf.push_str(CoreUtility::get_frequency_mnemonic(interest_change.interest_frequency()).as_str());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
    }

    if (interest_change.day_count_basis() == crate::DayCountType::Periodic ||
        interest_change.day_count_basis() == crate::DayCountType::RuleOf78) &&
        interest_change.days_in_year() > 0 && nar > dec!(0.0) {
    
      buf.push_str(self.indent().as_str());
      buf.push_str("\"interest-statistics\": {");
      buf.push_str(crate::LINE_ENDING);
      self.increment_depth();
  
      buf.push_str(self.indent().as_str());
      buf.push_str("\"interest-statistics-dr\": \"");
      buf.push_str(list_locale.format_decimal(CoreUtility::rate_nar_to_dr(
        nar / dec!(100.0), interest_change.days_in_year()) * dec!(100.0)).as_str());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
  
      buf.push_str(self.indent().as_str());
      buf.push_str("\"interest-statistics-ear\": \"");
      buf.push_str(list_locale.format_decimal(CoreUtility::rate_nar_to_ear(
        nar / dec!(100.0), frequency, interest_change.days_in_year()) * dec!(100.0)).as_str());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
  
      buf.push_str(self.indent().as_str());
      buf.push_str("\"interest-statistics-pr\": \"");
      buf.push_str(list_locale.format_decimal(CoreUtility::rate_nar_to_pr(
        nar / dec!(100.0), frequency, interest_change.days_in_year()) * dec!(100.0)).as_str());
      buf.push_str("\"");
      buf.push_str(crate::LINE_ENDING);

      self.decrement_depth();
      buf.push_str(self.indent().as_str());
      buf.push_str("},");
      buf.push_str(crate::LINE_ENDING);

    }

    if interest_change.round_balance() != crate::RoundType::None {
      buf.push_str(self.indent().as_str());
      buf.push_str("\"round-balance\": \"");
      buf.push_str(CoreUtility::get_round_balance(interest_change.round_balance()).as_str());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
    }

    let dd: usize;
    match interest_change.round_decimal_digits().to_usize() {
      None => { dd = 0; }
      Some(o) => { dd = o; }
    }

    if dd != decimal_digits {
      buf.push_str(self.indent().as_str());
      buf.push_str("\"round-decimal-digits\": \"");
      buf.push_str(interest_change.round_decimal_digits().to_string().as_str());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
    }

    buf.push_str(self.indent().as_str());
    buf.push_str("\"days-in-year\": ");
    buf.push_str(interest_change.days_in_year().to_string().as_str());
    buf.push_str(crate::LINE_ENDING);

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("}");
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize list of parameters.
  /// 
  /// # Arguments
  ///
  /// * `list_parameter` - List of parameters to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_parameter_list(
    self: &Self, list_parameter: &ListParameter, buf: &mut String, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"parameter-list\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if list_parameter.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"name\": \"");
        buf.push_str(list_parameter.name());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"parameter-type\": \"");
        buf.push_str(self.get_param_type(list_parameter.param_type()));
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"value\": \"");

        match list_parameter.param_type() {
          crate::TokenType::Integer => { 
            buf.push_str(list_parameter.param_integer().to_string().as_str());
          }
          crate::TokenType::Decimal => { 
            buf.push_str(list_parameter.param_float().to_string().as_str());
          }
          _ => { 
            buf.push_str(self.escape_string(list_parameter.param_string()).as_str());
          }
        }

        buf.push_str("\"");
        buf.push_str(crate::LINE_ENDING);
    
        index += 1;
        deserialize_list = list_parameter.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize preferences element.
  /// 
  /// # Arguments
  ///
  /// * `preferences` - Preferences element.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_preferences(
    self: &Self, preferences: &ElemPreferences, buf: &mut String, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"preferences\": {");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"group\": \"");
    buf.push_str(preferences.group());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"locale\": \"");
    buf.push_str(preferences.locale_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    if preferences.cross_rate_code().len() > 0 {
      buf.push_str(self.indent().as_str());
      buf.push_str("\"cross-rate-code\": \"");
      buf.push_str(preferences.cross_rate_code());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
    }

    if preferences.default_encoding().len() > 0 {
      buf.push_str(self.indent().as_str());
      buf.push_str("\"default-encoding\": \"");
      buf.push_str(preferences.default_encoding());
      buf.push_str("\",");
      buf.push_str(crate::LINE_ENDING);
    }

    buf.push_str(self.indent().as_str());
    buf.push_str("\"decimal-digits\": ");
    buf.push_str(preferences.decimal_digits().to_string().as_str());
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"fiscal-year-start\": ");
    buf.push_str(preferences.fiscal_year_start().to_string().as_str());
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"combine-principal\": ");
    buf.push_str(preferences.combine_principal().to_string().as_str());
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"compress-descriptor\": ");
    buf.push_str(preferences.compress_descriptor().to_string().as_str());
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"omit-statistic-events\": ");
    buf.push_str(preferences.statistic_events().to_string().as_str());
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    self.serialize_parameter_list(preferences.list_parameter(), buf, true);

    self.serialize_descriptor_list(preferences.list_descriptor(), buf, false);

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("}");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize principal change element.
  /// 
  /// # Arguments
  ///
  /// * `prin_change` - Principal change element.
  /// * `buf` - Buffer to append serialization.

  fn serialize_principal_change(
    self: &Self, prin_change: &ElemPrincipalChange, buf: &mut String) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"principal-change\": {");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"principal-type\": \"");
    buf.push_str(CoreUtility::get_principal_type_mnemonic(prin_change.pc_type()).as_str());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"auxiliary\": ");
    buf.push_str(self.get_bool_str(prin_change.auxiliary()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"passive\": ");
    buf.push_str(self.get_bool_str(prin_change.aux_passive()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"principal-first\": ");
    buf.push_str(self.get_bool_str(prin_change.principal_first()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"statistics\": ");
    buf.push_str(self.get_bool_str(prin_change.balance_statistics()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"eom\": ");
    buf.push_str(self.get_bool_str(prin_change.eom()));
    buf.push_str(crate::LINE_ENDING);

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("}");
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize statistic value element.
  /// 
  /// # Arguments
  ///
  /// * `statistic_value` - Statistic value element.
  /// * `buf` - Buffer to append serialization.

  fn serialize_statistic_value(
    self: &Self, statistic_value: &ElemStatisticValue, buf: &mut String) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"statistic-value\": {");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    buf.push_str(self.indent().as_str());
    buf.push_str("\"name\": \"");
    buf.push_str(statistic_value.name());
    buf.push_str("\",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"final\": ");
    buf.push_str(self.get_bool_str(statistic_value.is_final()));
    buf.push_str(",");
    buf.push_str(crate::LINE_ENDING);

    buf.push_str(self.indent().as_str());
    buf.push_str("\"eom\": ");
    buf.push_str(self.get_bool_str(statistic_value.eom()));
    buf.push_str(crate::LINE_ENDING);

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("}");
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize list of template events.
  /// 
  /// # Arguments
  ///
  /// * `template_events` - List of template events to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_template_events(
    self: &Self, template_events: &ListTemplateEvent, buf: &mut String, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"template-events\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if template_events.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();

        self.serialize_event_list(template_events.list_event(), buf, true);
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"name\": \"");
        buf.push_str(template_events.name());
        buf.push_str("\"");
        buf.push_str(crate::LINE_ENDING);
    
        index += 1;
        deserialize_list = template_events.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Serialize list of template groups.
  /// 
  /// # Arguments
  ///
  /// * `template_groups` - List of template groups to serialize.
  /// * `buf` - Buffer to append serialization.
  /// * `add_comma` - Append comma on last line of output.

  fn serialize_template_groups(
    self: &Self, template_groups: &ListTemplateGroup, buf: &mut String, add_comma: bool) -> () {

    buf.push_str(self.indent().as_str());
    buf.push_str("\"template-groups\": [");
    buf.push_str(crate::LINE_ENDING);
    self.increment_depth();

    let mut index: usize = 0;
    if template_groups.get_element(index) {
      let mut deserialize_list = true;

      while deserialize_list {

        buf.push_str(self.indent().as_str());
        buf.push_str("{");
        buf.push_str(crate::LINE_ENDING);
        self.increment_depth();
    
        buf.push_str(self.indent().as_str());
        buf.push_str("\"group\": \"");
        buf.push_str(template_groups.group());
        buf.push_str("\",");
        buf.push_str(crate::LINE_ENDING);

        self.serialize_preferences(template_groups.preferences(), buf, true);

        self.serialize_template_events(template_groups.list_template_event(), buf, false);
    
        index += 1;
        deserialize_list = template_groups.get_element(index);

        self.decrement_depth();
        buf.push_str(self.indent().as_str());
        buf.push_str("}");
        if deserialize_list { buf.push_str(","); }
        buf.push_str(crate::LINE_ENDING);
      }
    }

    self.decrement_depth();
    buf.push_str(self.indent().as_str());
    buf.push_str("]");
    if add_comma { buf.push_str(","); }
    buf.push_str(crate::LINE_ENDING);
  }

  /// Escape the input string and return json output.
  ///
  /// # Arguments
  ///
  /// * `input_str` - Input string.
  /// 
  /// # Return
  ///
  /// * See description.

  fn escape_string(self: &Self, input_str: &str) -> String {

    return input_str.replace('"', "\\\"");
  }

  /// Decrement the indentation depth.

  fn decrement_depth(self: &Self) -> () {

    if self.depth.get() == 0 { return; }

    self.depth.set(self.depth.get() - 1);
  }

  /// Serialize a boolean value.
  /// 
  /// # Arguments
  ///
  /// * `opt` - Boolean option.
  /// 
  /// # Return
  ///
  /// * See description.

  fn get_bool_str(self: &Self, opt: bool) -> &str {

    return if opt { "true" } else { "false" };
  }

  /// Serialize a date value.
  /// 
  /// # Arguments
  ///
  /// * `event_date` - Date value.
  /// 
  /// # Return
  ///
  /// * See description.

  fn get_date_str(self: &Self, event_date: usize) -> String {

    return format!("{:04}-{:02}-{:02}", 
      event_date / 10000, event_date / 100 % 100, event_date % 100);
  }

  /// Serialize a parameter type.
  /// 
  /// # Arguments
  ///
  /// * `param_type` - Parameter type value.
  /// 
  /// # Return
  ///
  /// * See description.

  fn get_param_type(self: &Self, param_type: crate::TokenType) -> &str {

    match param_type {
      crate::TokenType::Integer => { return "integer"; }
      crate::TokenType::Decimal => { return "decimal"; }
      _ => { return "string";}
    }
  }

  /// Serialize a polarity value.
  /// 
  /// # Arguments
  ///
  /// * `polarity` - Polarity value.
  /// 
  /// # Return
  ///
  /// * See description.

  fn get_polarity(self: &Self, polarity: i32) -> &str {

    if polarity < 0 { return "negative"; }    
    return "positive";
  }

  /// Increment the indentation depth.

  fn increment_depth(self: &Self) -> () {

    self.depth.set(self.depth.get() + 1);
  }

  /// Return a string containing spaces relative to the indentation depth.
  /// 
  /// # Return
  ///
  /// * See description.

  fn indent(self: &Self) -> String {

    return " ".repeat(self.depth.get() * crate::TAB_SPACES);
  }

}