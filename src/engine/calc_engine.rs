//! The AmFn engine component.
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
use std::cell::{Ref, RefMut};
use std::collections::HashMap;
use rust_decimal::prelude::*;

use crate::{ExtensionTrait, ListTrait, ElemLevelType};
use crate::core::{CoreManager, UpdateListener, CoreUtility, ElemSymbol, ElemBalanceResult, 
  ListParameter, ListDescriptor, ListEvent, ListAmortization, ListStatisticHelper};
use super::{CalcManager, CalcExpression, CalcUtility, ElemPreferences, ElemCashflowStats};

pub struct CalcEngine {

  /// Calculator manager element. 
  calc_manager: Rc<RefCell<CalcManager>>

}

/// The main implementation of the AmFn engine component.

impl CalcEngine {

  /// Create and return a new AmFn engine.
  /// 
  /// # Arguments
  ///
  /// * `update_listener_param` - Update listener.
  /// 
  /// # Return
  ///
  /// * See description.
   
  pub fn new(update_listener_param: UpdateListener) -> CalcEngine {

    let calc_engine = CalcEngine { // The AmFn manager instance
      calc_manager: Rc::new(RefCell::new(CalcManager::new(CoreManager::new(update_listener_param))))
    };

    // Inject the wrapped calculation manager into itself
    calc_engine.calc_reg_mut().init_calc_manager(&calc_engine.calc_manager());

    calc_engine.calc_reg_mut().list_cashflow_mut().set_calc_reg(&calc_engine.calc_manager());
    calc_engine.calc_reg_mut().list_template_group_mut().set_calc_reg(&calc_engine.calc_manager());

    return calc_engine;
  }

  /// Return the calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn calc_manager(self: &Self) -> &Rc<RefCell<CalcManager>> {

    return &self.calc_manager;
  }

  /// Returns the calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn calc_reg(self: &Self) -> Ref<CalcManager> {

    return self.calc_manager.borrow();
  }

  /// Returns the mutable calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn calc_reg_mut(self: &Self) -> RefMut<CalcManager> {

    return self.calc_manager.borrow_mut();
  }

  /// Initialize the engine.
  /// 
  /// # Arguments
  ///
  /// * `locale_str_param` - Initial locale string to select.

  pub fn init_engine(self: &Self, locale_str_param: &str) -> () {

    if locale_str_param.len() > 0 {
      self.calc_reg().reg().list_locale_mut().select_user_locale(locale_str_param);
      
      let decimal_digits = self.calc_reg().reg().list_locale().decimal_digits(false);
      self.calc_reg().preferences().set_decimal_digits(decimal_digits);
    }

    let orig_index = self.calc_reg().list_cashflow().index();

    self.evaluate_user_descriptors();
    self.calc_reg().list_template_group().evaluate_descriptors();

    let mut index = 0;

    loop {
      if !self.calc_reg().list_cashflow().get_element(index) { break; }

      if !self.calc_reg().list_cashflow().cashflow_valid() {    

        let locale = String::from(self.calc_reg().locale(true));
        self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());
    
        self.evaluate_cashflow_descriptors();
        self.evaluate_cashflow_event_type_all();

        match self.balance_cashflow() {
          Err(_e) => { }
          Ok(_o) => { }
        }
      }

      index += 1;
    }
    
    self.calc_reg().list_cashflow().get_element(orig_index);

  }

  /// Copy the current preferences locales, exchange rates, template groups, 
  /// and optionally cashflows and return a new AmFn engine.
  /// 
  /// # Arguments
  ///
  /// * `update_listener_param` - Update listener.
  /// * `cashflow` - Copy the list of cashflows.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self, update_listener_param: UpdateListener, cashflow: bool) -> CalcEngine {
    let calc_engine = CalcEngine::new(update_listener_param);

    let preferences = self.calc_manager().borrow().preferences().copy_with_calc_manager(
      calc_engine.calc_manager(), ElemLevelType::Engine, true);
    calc_engine.calc_reg_mut().set_preferences(preferences);

    let list_locale = self.calc_manager().borrow().reg().list_locale().copy();
    calc_engine.calc_reg_mut().reg_mut().set_list_locale(list_locale);

    let list_exchange_rate = self.calc_manager().borrow().list_exchange_rate().copy_with_calc_manager(
      calc_engine.calc_manager().borrow().core_manager());
    calc_engine.calc_reg_mut().set_list_exchange_rate(list_exchange_rate);

    let list_template_group = self.calc_manager().borrow().list_template_group().copy_with_calc_manager(
      calc_engine.calc_manager());
    calc_engine.calc_reg_mut().set_list_template_group(list_template_group);

    if cashflow {
      let list_cashflow = self.calc_manager().borrow().list_cashflow().copy_with_calc_manager(
        calc_engine.calc_manager());
      calc_engine.calc_reg_mut().set_cashflow(list_cashflow);

      let mut index: usize = 0;

      loop {
        if !self.calc_reg().list_cashflow().get_element(index) { break; }

        let locale = String::from(self.calc_reg().locale(true));
        calc_engine.calc_manager().borrow().reg().list_locale_mut().select_cashflow_locale(
          locale.as_str());
    
        self.evaluate_cashflow_descriptors();
        self.evaluate_cashflow_event_type_all();

        if !self.calc_reg().list_cashflow().cashflow_valid() {
          match self.balance_cashflow() {
            Err(_e) => { }
            Ok(_o) => { }
          }
        }

        index += 1;
      }
    }

    return calc_engine;
  }

  /// Copies the event list from the currently selected template event into
  /// the event list of the currently selected cashflow.
  /// 
  /// # Arguments
  ///
  /// * `date_param` - Base starting date for the new event(s).
  /// * `end_date_param` - Base ending date for the new event(s).
  /// * `new_date_param` - Next date for the new event(s) (i.e.,
  ///     normally end_date_param plus one period).
  /// * `frequency_param` - Next frequency for the new event(s).
  /// * `is_sort_on_add` - If true sets sort on add.
  ///
  /// # Return
  ///
  /// * ERROR_NONE if successful, otherwise an error code.

  fn copy_template_events(self: &Self, date_param: usize, end_date_param: usize, 
    new_date_param: usize, frequency_param: crate::FrequencyType,
    is_set_sort_on_add: bool) -> Result<(), crate::ErrorType> {

    let new_list_event: ListEvent;
    match self.calc_reg().copy_template_events(
        date_param, end_date_param, new_date_param, frequency_param) {
      Err(e) => { return Err(e); }
      Ok(o) => { new_list_event = o; }
    }

    let mut reg = self.calc_reg_mut();
    let list_event: &mut ListEvent;
    match reg.list_cashflow_mut().list_event_mut() {
      None => { return Err(crate::ErrorType::Index); }
      Some(o) => { list_event = o; }
    }

    list_event.set_sort_on_add(false);

    for event in new_list_event.list() {

      let new_extension = event.elem_extension().copy();

      let mut list_parameter_opt: Option<ListParameter> = None;
      match event.list_parameter().as_ref() {
        None => { }
        Some(o2) => { list_parameter_opt = Option::from(o2.copy(ElemLevelType::Event, true)); }
      }

      let mut list_descriptor_opt: Option<ListDescriptor> = None;
      match event.list_descriptor().as_ref() {
        None => { }
        Some(o2) => { list_descriptor_opt = Option::from(o2.copy(false, ElemLevelType::Event, true)); }
      }
  
      list_event.add_event_ex(
        event.event_date(), event.date_expr(), event.sort_order(),
        event.value(), event.value_expr(), event.value_expr_balance(),
        event.periods(), event.periods_expr(), event.skip_mask_len(), event.skip_mask(),
        event.intervals(), event.frequency(), new_extension, list_parameter_opt, list_descriptor_opt,
        event.event_name(), event.next_name());
    }


    if is_set_sort_on_add { 
      list_event.set_sort_on_add(true); // Sorts list
    }

    return Ok(());
  }

  /// Performs primary calculations on a cashflow.
  /// Passes through an entire cashflow calculating the
  /// interest, accrued interest balance and balance for each
  /// event within the cashflow. This method directly handles
  /// the rule of 78 and subsequently compresses the cashflow.
  /// 
  /// # Return
  ///
  /// * The results from this method or an error code.

  pub fn balance_cashflow(self: &Self) -> Result<ElemBalanceResult, crate::ErrorType> {    

    let combine_principal = self.calc_reg().combine_principal(true);

    let mut list_am: ListAmortization;
    let mut statistic_helper: ListStatisticHelper;
    let mut elem_balance_result = ElemBalanceResult::new();

    let cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        
        let list_event = o.list_event();        
        statistic_helper = o.list_statistic_helper().copy();

        let result = o.calculate().expand_cashflow(list_event, false);
        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { list_am = o; }
        }

        let result = o.calculate().normalize_cashflow(&list_am, &mut statistic_helper, combine_principal);

        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { list_am = o; }
        }
        
        let mut result = o.calculate().balance_cashflow(
          &mut list_am, &mut statistic_helper, &elem_balance_result, false, false, false);
          match result {
            Err(e) => { return Err(e); }
            Ok(o) => { elem_balance_result = o; }
          }
              
        if elem_balance_result.rule_of_78_seen() { // Perform rule of 78 interest allocation
          result = o.calculate().balance_cashflow(
            &mut list_am, &mut statistic_helper, &elem_balance_result, false, true, false);
          match result {
            Err(e) => { return Err(e); }
            Ok(o) => { elem_balance_result = o; }
          }
        }
      }
    }
    
    let balance_result = elem_balance_result.copy();    

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        o.set_list_amortization(list_am);
        o.set_list_statistic_helper(statistic_helper);
        o.set_elem_balance_result(elem_balance_result);
        
        if !o.list_amortization().get_element(o.last_amortization_index()) && 
            o.last_amortization_index() != usize::MAX {
          o.list_amortization().get_element(o.list_amortization().count() - 1);
        }
        
        o.set_last_amortization_index(o.list_amortization().index());
            
        o.set_cashflow_valid(true);
      }
    }

    return Ok(balance_result);
  }
  
  /// Calculates the value for an overall yield (i.e., APR).
  /// Calculates an overall yield value that will satisfy the
  /// condition that the remaining balance of the cashflow
  /// is the smallest amount greater than or equal to the given
  /// parameter value.
  /// 
  /// # Arguments
  ///
  /// * `balance` - Desired balance or 0.
  /// 
  /// # Return
  ///
  /// * The results from this method or an error code.

  pub fn calculate_yield(self: &Self, balance: Decimal) -> Result<ElemBalanceResult, crate::ErrorType> {

    let updating_json = self.calc_reg().updating_json();

    let mut list_am: ListAmortization;
    let mut list_statistic_helper: ListStatisticHelper;

    let cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    let elem_balance_result: ElemBalanceResult;
    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
    
        list_am = o.list_amortization().copy(updating_json);
        list_statistic_helper = o.list_statistic_helper().copy();
    
        let result = o.calculate().calculate_yield(
          o.list_event(), &mut list_am, &mut list_statistic_helper, balance);
        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { elem_balance_result = o; }
        }
      }
    }

    let balance_result = elem_balance_result.copy();    

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        o.set_list_amortization(list_am);
        o.set_list_statistic_helper(list_statistic_helper);
        o.set_elem_balance_result(elem_balance_result);
      }
    }
    
    return Ok(balance_result);
  }

  /// Calculates the value for an event.
  /// Calculates either an interest amount or a principal amount
  /// (depending upon the selected event type) that will satisfy
  /// the condition that the remaining balance of the cashflow
  /// is the smallest amount greater than or equal to the given
  /// parameter value.
  /// 
  /// # Arguments
  ///
  /// * `target_value` - See description.
  /// 
  /// # Return
  ///
  /// * The results from this method or an error code.

  pub fn calculate_value(self: &Self, target_value: Decimal) -> Result<ElemBalanceResult, crate::ErrorType> {

    let updating_json = self.calc_reg().updating_json();

    let mut list_am: ListAmortization;
    let mut list_statistic_helper: ListStatisticHelper;
    let elem_balance_result: ElemBalanceResult;

    let cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        list_am = o.list_amortization().copy(updating_json);
        list_statistic_helper = o.list_statistic_helper().copy();

        match o.list_event().elem_type() {
          crate::ExtensionType::PrincipalChange => {
            let result = o.calculate().calculate_principal(
              o.list_event(), &mut list_am,
              &mut list_statistic_helper, target_value);

            match result {
              Err(e) => { return Err(e); }
              Ok(o) => { elem_balance_result = o; }
            }
          }
          crate::ExtensionType::InterestChange => {
            let result = o.calculate().calculate_interest(
              o.list_event(), &mut list_am,
              &mut list_statistic_helper, target_value);

            match result {
              Err(e) => { return Err(e); }
              Ok(o) => { elem_balance_result = o; }
            }
          }
          _ => { 
            elem_balance_result = ElemBalanceResult::new();       
          }
        }
      }
    }

    let balance_result = elem_balance_result.copy();   

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        o.set_cashflow_valid(false);
        o.set_list_amortization(list_am);
        o.set_list_statistic_helper(list_statistic_helper);
        o.set_elem_balance_result(elem_balance_result);
      }
    }
    
    return Ok(balance_result);
  }

  /// Calculates the periods for an event.
  /// Calculates the number of periods that will satisfy the
  /// condition that the remaining balance of the cashflow
  /// is the smallest amount greater than or equal to the given
  /// parameter value.
  /// 
  /// # Arguments
  ///
  /// * `target_value` - See description.
  /// 
  /// # Return
  ///
  /// * The results from this method or an error code.

  pub fn calculate_periods(self: &Self, target_value: Decimal) -> Result<ElemBalanceResult, crate::ErrorType> {

    let updating_json = self.calc_reg().updating_json();

    let mut list_am: ListAmortization;
    let mut list_statistic_helper: ListStatisticHelper;
    let elem_balance_result: ElemBalanceResult;    

    let cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        list_am = o.list_amortization().copy(updating_json);
        list_statistic_helper = o.list_statistic_helper().copy();
        
        let result = o.calculate().calculate_periods(
          o.list_event(), &mut list_am,
          &mut list_statistic_helper, target_value);

        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { elem_balance_result = o; }
        }
      }
    }

    let balance_result = elem_balance_result.copy();    

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        o.set_cashflow_valid(false);
        o.set_list_amortization(list_am);
        o.set_list_statistic_helper(list_statistic_helper);
        o.set_elem_balance_result(elem_balance_result);
      }
    }

    return Ok(balance_result);
  }

  /// Combines the principal change events from two amortization element lists 
  /// into a new amortization element list, compresses the list and transforms 
  /// the compressed list into a new event list. All events except principal 
  /// change and interest change events are discarded.
  /// 
  /// # Arguments
  ///
  /// * `name2_param` - The name of the second cashflow to combine.
  /// * `new_name_param` - The name of the new cashflow.
  /// * `new_group_param` - The optional name of the new template group or empty.
  /// 
  /// # Return
  ///
  /// * The results from this method or an error code.
  
  pub fn combine_cashflow(self: &Self, name2_param: &str, new_name_param: &str,
    new_group_param: &str) -> Result<ElemBalanceResult, crate::ErrorType> {

    let mut cf_index = self.calc_reg().list_cashflow().index();
        
    if !self.calc_reg().list_cashflow().get_element_by_name(name2_param, true) {
      return Err(crate::ErrorType::CfName);
    }

    let cf_index2 = self.calc_reg().list_cashflow().index();

    self.calc_reg().list_cashflow().get_element(cf_index);
    
    let mut new_name = String::from(new_name_param);
    if new_name.len() == 0 {
      new_name = String::from(self.calc_reg().reg().list_locale().get_resource(crate::USER_NEW));
    }
        
    let mut new_group = String::from(new_group_param);        
    
    let new_list_event: ListEvent;

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.calc_reg().set_updating_json(false);

    let mut elem_preferences_opt: Option<ElemPreferences> = None;
    if new_group.len() > 0 {
      if self.calc_reg().list_template_group().get_element_by_group(new_group.as_str(), true) {
        elem_preferences_opt = Option::from(self.calc_reg().list_template_group().preferences().copy(
          ElemLevelType::Cashflow, true));
      }
    }

    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {

        let new_list_am: ListAmortization;
        match self.calc_reg().list_cashflow().list().iter().nth(cf_index2) {
          None => { return Err(crate::ErrorType::Cashflow); }
          Some(o2) => {

            let result = o.calculate().combine_cashflow(o.list_amortization(), o2.list_amortization());
            match result {
              Err(e) => { return Err(e); }
              Ok(o) => { new_list_am = o; }
            }
          }
        }

        let new_list_am_output: ListAmortization;
        let result = o.calculate().create_cashflow_output(&new_list_am, true, false, true, true, true);
        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { new_list_am_output = o; }
        }

        let result = o.calculate().transform_cashflow(&new_list_am_output, false, false, true, true);
        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { new_list_event = o; }
        }
        
        if new_group.len() == 0 && o.preferences().group().len() > 0 {
          new_group = String::from(o.preferences().group());
        }

        match elem_preferences_opt.as_mut() {
          None => { }
          Some(o2) => {
            let list_parameter = o.preferences().list_parameter();
            if list_parameter.get_element_by_name(crate::PARAM_DESCRIPTION, true) {
              if !o2.list_parameter().get_element_by_name(crate::PARAM_DESCRIPTION, true) {
                o2.list_parameter_mut().add_parameter(crate::PARAM_DESCRIPTION, false);
              }
              o2.list_parameter_mut().set_string(list_parameter.param_string());
            }
          }
        }
        
        if elem_preferences_opt.is_none() {
          elem_preferences_opt = Option::from(o.preferences().copy(ElemLevelType::Cashflow, true));
        }
      }
    }

    let result = self.calc_reg().list_cashflow().add_cashflow(
      new_name.as_str(), Option::from(new_list_event), 
      elem_preferences_opt, new_group.as_str());
    match result {
      Err(_e) => { return Err(crate::ErrorType::Cashflow); }
      Ok(o) => {
        let mut reg = self.calc_reg_mut();
        reg.list_cashflow_mut().list_mut().push(o);
        reg.list_cashflow_mut().sort();
        
        match reg.list_cashflow().list().iter().position(|e| e.name() == new_name) {
          None => { }
          Some(o) => {
            reg.list_cashflow().set_index(o);      
          }
        }    
      }
    }

    cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.evaluate_cashflow_descriptors();
    self.evaluate_cashflow_event_type_all();

    let elem_balance_result: ElemBalanceResult;
    match self.balance_cashflow() {
      Err(e) => { return Err(e); }
      Ok(o) => { elem_balance_result = o; }
    }

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {        
        if new_group.len() > 0 {
          o.preferences_mut().set_group_result(new_group.as_str());
        }

        o.list_event().set_index(0);
      }
    }

    self.calc_reg().set_updating_json(false);
    
    return Ok(elem_balance_result);
  }

  /// Merges the events of two cashflows into a new cashflow according
  /// to order defined by intDate and intSort with a bias towards the
  /// first cashflow given equal sort keys. All events except principal
  /// change and interest change events are discarded.
  /// 
  /// # Arguments
  ///
  /// * `name2_param` - The name of the second cashflow to merge.
  /// * `new_name_param` - The name of the new cashflow.
  /// * `new_group_param` - The optional name of the new template group or empty.
  /// * `interest_event_action` - The action to be performed when interest events 
  ///     are encountered.
  ///
  /// # Return
  ///
  /// * The results from this method or an error code.

  pub fn merge_cashflow(self: &Self, name2_param: &str, new_name_param: &str, 
      new_group_param: &str, interest_event_action: crate::MergeType) -> 
      Result<ElemBalanceResult, crate::ErrorType> {

    let mut cf_index = self.calc_reg().list_cashflow().index();
        
    if !self.calc_reg().list_cashflow().get_element_by_name(name2_param, true) {
      return Err(crate::ErrorType::CfName);
    }

    let cf_index2 = self.calc_reg().list_cashflow().index();

    self.calc_reg().list_cashflow().get_element(cf_index);

    let mut new_name: String = String::from(new_name_param);
    if new_name.len() == 0 {
      new_name = String::from(self.calc_reg().reg().list_locale().get_resource(crate::USER_NEW));
    }

    let mut new_group = String::from(new_group_param);
    let new_list_event: ListEvent;

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.calc_reg().set_updating_json(true);

    let mut elem_preferences_opt: Option<ElemPreferences> = None;
    if new_group.len() > 0 {
      if self.calc_reg().list_template_group().get_element_by_group(new_group.as_str(), true) {
        elem_preferences_opt = Option::from(self.calc_reg().list_template_group().preferences().copy(
          ElemLevelType::Cashflow, true));
      }
    }

    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => { 

        match self.calc_reg().list_cashflow().list().iter().nth(cf_index2) {
          None => { return Err(crate::ErrorType::Cashflow); }
          Some(o2) => {

            let result = o.calculate().merge_cashflow(
              o.list_event(), o2.list_event(), interest_event_action);
            match result {
              Err(e) => { return Err(e); }
              Ok(o) => { new_list_event = o; }
            }
          }
        }
          
        if new_group.len() == 0 && o.preferences().group().len() > 0 {
          new_group = String::from(o.preferences().group());
        }

        match elem_preferences_opt.as_mut() {
          None => { }
          Some(o2) => {
            let list_parameter = o.preferences().list_parameter();
            if list_parameter.get_element_by_name(crate::PARAM_DESCRIPTION, true) {
              if !o2.list_parameter().get_element_by_name(crate::PARAM_DESCRIPTION, true) {
                o2.list_parameter_mut().add_parameter(crate::PARAM_DESCRIPTION, false);
              }
              o2.list_parameter_mut().set_string(list_parameter.param_string());
            }
          }
        }
        
        if elem_preferences_opt.is_none() {
          elem_preferences_opt = Option::from(o.preferences().copy(ElemLevelType::Cashflow, true));
        }
      }
    }

    let result = self.calc_reg().list_cashflow().add_cashflow(
      new_name.as_str(), Option::from(new_list_event), 
      elem_preferences_opt, new_group.as_str());
    match result {
      Err(_e) => { return Err(crate::ErrorType::Cashflow); }
      Ok(o) => { 
        let mut reg = self.calc_reg_mut();
        reg.list_cashflow_mut().list_mut().push(o);
        reg.list_cashflow_mut().sort();
        
        match reg.list_cashflow().list().iter().position(|e| e.name() == new_name) {
          None => { }
          Some(o) => {
            reg.list_cashflow().set_index(o);      
          }
        }    
      }
    }

    cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.evaluate_cashflow_descriptors();
    self.evaluate_cashflow_event_type_all();

    let elem_balance_result: ElemBalanceResult;
    match self.balance_cashflow() {
      Err(e) => { return Err(e); }
      Ok(o) => { elem_balance_result = o; }
    }

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {        
        if new_group.len() > 0 {
          o.preferences_mut().set_group_result(new_group.as_str());
        }

        o.list_event().set_index(0);
      }
    }

    self.calc_reg().set_updating_json(false);

    return Ok(elem_balance_result);
  }

  /// Splits the currently selected principal change event into
  /// multiple principal change events whenever an active event
  /// is seen in the event list that is between the start and end
  /// dates of the principal change event.
  /// 
  /// # Arguments
  ///
  /// * `all_events` - If true split the principal change event when any
  ///     active event is seen (otherwise only split the principal change event
  ///     when an interest change event is seen).
  ///
  /// # Return
  ///
  /// * The results from this method or an error code.
  
  pub fn split_cashflow(self: &Self, all_events: bool) -> Result<ElemBalanceResult, crate::ErrorType> {

    let mut list_event: ListEvent;

    let cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.calc_reg().set_updating_json(true);

    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        list_event = o.list_event().copy(true);        
        
        let result = o.calculate().split_cashflow(&mut list_event, all_events);

        match result {
          Err(e) => { return Err(e); }
          Ok(_o) => { }
        }
      }
    }

    let elem_balance_result: ElemBalanceResult;
    match self.balance_cashflow() {
      Err(e) => { return Err(e); }
      Ok(o) => { elem_balance_result = o; }
    }

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
        o.set_list_event(list_event);
      }
    }

    self.calc_reg().set_updating_json(false);
    
    return Ok(elem_balance_result);
  }

  /// Transform the amortization list by creating an event list
  /// composed of events that are either before the current value
  /// present event (if after_pv is false) or after the current
  /// value present event (if after_pv is true). The current
  /// value present event itself is transformed into a principal
  /// change event.
  /// 
  /// # Arguments
  ///
  /// * `new_name_param` - The name of the new cashflow.
  /// * `new_group_param` - The optional name of the new template group or empty.
  /// * `after_pv` - If true transform the events after the
  ///     current value present event. Otherwise, transform the events
  ///     before the current value present event.
  /// * `omit_interest_events` - If true the interest events are
  ///     eliminated from the resulting event list.
  ///
  /// # Return
  ///
  /// * The results from this method or an error code.
  
  pub fn transform_cashflow(self: &Self, new_name_param: &str, new_group_param: &str, 
      after_pv: bool, omit_interest_events: bool) -> Result<ElemBalanceResult, crate::ErrorType> {

    let mut new_name: String = String::from(new_name_param);
    if new_name.len() == 0 {
      new_name = String::from(self.calc_reg().reg().list_locale().get_resource(crate::USER_NEW));
    }

    let mut new_group: String = String::from(new_group_param);
    let new_list_event: ListEvent;

    let mut cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.calc_reg().set_updating_json(true);

    let mut elem_preferences_opt: Option<ElemPreferences> = None;
    if new_group.len() > 0 {
      if self.calc_reg().list_template_group().get_element_by_group(new_group.as_str(), true) {
        elem_preferences_opt = Option::from(self.calc_reg().list_template_group().preferences().copy(
          ElemLevelType::Cashflow, true));
      }
    }

    match self.calc_reg().list_cashflow().list().iter().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {
      
        if !o.cashflow_valid() {
          return Err(crate::ErrorType::Cashflow);
        }

        let new_list_am_output: ListAmortization;
        let result = o.calculate().create_cashflow_output(
          o.list_amortization(), true, false, true, true, true);
        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { new_list_am_output = o; }
        }

        let result = o.calculate().transform_cashflow(
          &new_list_am_output, after_pv, omit_interest_events, false, true);
        match result {
          Err(e) => { return Err(e); }
          Ok(o) => { new_list_event = o; }
        }
          
        if new_group.len() == 0 && o.preferences().group().len() > 0 {
          new_group = String::from(o.preferences().group());
        }

        if new_group.len() > 0 {
          let calc_reg = self.calc_reg();
          let list_template_group = calc_reg.list_template_group();
          if list_template_group.get_element_by_group(new_group.as_str(), true) {
            let list_parameter = o.preferences().list_parameter();
            let mut elem_preferences = list_template_group.preferences().copy(ElemLevelType::Cashflow, true);
            if list_parameter.get_element_by_name(crate::PARAM_DESCRIPTION, true) {
              if !elem_preferences.list_parameter().get_element_by_name(crate::PARAM_DESCRIPTION, true) {
                elem_preferences.list_parameter_mut().add_parameter(crate::PARAM_DESCRIPTION, false);
              }
              elem_preferences.list_parameter_mut().set_string(list_parameter.param_string());
            }
            elem_preferences_opt = Option::from(elem_preferences);
          }
        }

        match elem_preferences_opt.as_mut() {
          None => { }
          Some(o2) => {
            let list_parameter = o.preferences().list_parameter();
            if list_parameter.get_element_by_name(crate::PARAM_DESCRIPTION, true) {
              if !o2.list_parameter().get_element_by_name(crate::PARAM_DESCRIPTION, true) {
                o2.list_parameter_mut().add_parameter(crate::PARAM_DESCRIPTION, false);
              }
              o2.list_parameter_mut().set_string(list_parameter.param_string());
            }
          }
        }
        
        if elem_preferences_opt.is_none() {
          elem_preferences_opt = Option::from(o.preferences().copy(ElemLevelType::Cashflow, true));
        }        
      }
    }

    let result = self.calc_reg().list_cashflow().add_cashflow(
      new_name.as_str(), Option::from(new_list_event), 
      elem_preferences_opt, new_group.as_str());
    match result {
      Err(_e) => { return Err(crate::ErrorType::Cashflow); }
      Ok(o) => { 
        let mut reg = self.calc_reg_mut();
        reg.list_cashflow_mut().list_mut().push(o);
        reg.list_cashflow_mut().sort();
        
        match reg.list_cashflow().list().iter().position(|e| e.name() == new_name) {
          None => { }
          Some(o) => {
            reg.list_cashflow().set_index(o);      
          }
        }    
      }
    }

    cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.evaluate_cashflow_descriptors();
    self.evaluate_cashflow_event_type_all();

    let elem_balance_result: ElemBalanceResult;
    match self.balance_cashflow() {
      Err(e) => { return Err(e); }
      Ok(o) => { elem_balance_result = o; }
    }

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {        
        if new_group.len() > 0 {
          o.preferences_mut().set_group_result(new_group.as_str());
        }

        o.list_event().set_index(0);
      }
    }

    self.calc_reg().set_updating_json(false);
   
    return Ok(elem_balance_result);
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
  /// 
  /// # Return
  ///
  /// * The resulting amortization list or an error code.

  pub fn create_cashflow_output(self: &Self, include_rollups: bool, include_details: bool, 
      compress_descriptor: bool, omit_statistic_events: bool) -> Result<ListAmortization, crate::ErrorType> {
  
    return self.calc_reg().list_cashflow().create_cashflow_output(
      include_rollups, include_details, 
      compress_descriptor, omit_statistic_events, true);
  }

  /// Creates the events from currently selected template event list into
  /// the currently selected cashflow event list.
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
  /// * ERROR_NONE if successful, otherwise an error code.

  pub fn create_template_events(self: &Self, date_param: usize, end_date_param: usize, 
    new_date_param: usize, frequency_param: crate::FrequencyType) -> Result<(), crate::ErrorType> {

    return self.copy_template_events(date_param, end_date_param, new_date_param, frequency_param, true);
  }

  /// Creates a new cashflow and copies events from all template event lists
  /// under a named template group.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - The name of the template group.
  /// * `new_name_param` - The name of the new cashflow.
  ///
  /// # Return
  ///
  /// * A balance result if successful, otherwise an error code.

  pub fn create_cashflow_from_template_group(self: &Self, group_param: &str, 
    new_name_param: &str, new_group_param: &str) -> Result<ElemBalanceResult, crate::ErrorType> {

    let group = String::from(self.calc_reg().list_template_group().group());

    if !self.calc_reg().list_template_group().get_element_by_group(group_param, true) {
      return Err(crate::ErrorType::Index);
    }
        
    let elem_preferences_opt = Option::from(
      self.calc_reg().list_template_group().preferences().copy(ElemLevelType::Cashflow, true));

    let result = self.calc_reg().list_cashflow().add_cashflow(
      new_name_param, None, elem_preferences_opt, group.as_str());
    match result {
      Err(_e) => { return Err(crate::ErrorType::Cashflow); }
      Ok(o) => { 
        let mut reg = self.calc_reg_mut();
        reg.list_cashflow_mut().list_mut().push(o);
        reg.list_cashflow_mut().sort();
        
        match reg.list_cashflow().list().iter().position(|e| e.name() == new_name_param) {
          None => { }
          Some(o) => {
            reg.list_cashflow().set_index(o);      
          }
        }    
      }
    }

    let mut index = 0;
    loop {

      if !self.calc_reg().list_template_group().list_template_event().get_element(index) { break; }

      let mut event_date: usize = CoreUtility::date_now();
      let mut end_date: usize = event_date;
      let mut new_date: usize = event_date;
      let mut frequency = crate::FrequencyType::OneMonth;

      match self.calc_reg().list_cashflow().list_event() {
        None => { }
        Some(o) => {
          if o.count() > 0 {
            event_date = o.event_date();
            end_date = event_date;
    
            if o.periods() > 1 {
              end_date = CalcManager::util_date_new(end_date, o.periods() - 1,
                o.frequency(), o.intervals(), o.eom());
            }
    
            new_date = CalcManager::util_date_new(end_date, 1,
              o.frequency(), o.intervals(), o.eom());
    
            frequency = o.frequency();
          }
        }
      }

      let last_template_event = index >= self.calc_reg().list_template_group().list_template_event().count();
  
      match self.copy_template_events(event_date, end_date, new_date, frequency, last_template_event) {
        Err(e) => { return Err(e); }
        Ok(_o) => { }
      }

      index += 1;
    }

    let cf_index = self.calc_reg().list_cashflow().index();

    let locale = String::from(self.calc_reg().locale(true));
    self.calc_reg().reg().list_locale_mut().select_cashflow_locale(locale.as_str());

    self.evaluate_cashflow_descriptors();
    self.evaluate_cashflow_event_type_all();

    let elem_balance_result: ElemBalanceResult;
    match self.balance_cashflow() {
      Err(e) => { return Err(e); }
      Ok(o) => { elem_balance_result = o; }
    }

    match self.calc_reg_mut().list_cashflow_mut().list_mut().iter_mut().nth(cf_index) {
      None => { return Err(crate::ErrorType::Cashflow); }
      Some(o) => {        
        if new_group_param.len() > 0 {
          o.preferences_mut().set_group_result(new_group_param);
        }

        o.list_event().set_index(0);
      }
    }

    return Ok(elem_balance_result);
  }

  /// Creates and returns the statistics for the
  /// currently selected cashflow.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn create_cashflow_stats(self: &Self) -> ElemCashflowStats {

    return self.calc_reg().list_cashflow().create_cashflow_stats();
  }

  /// Evaluate all of the descriptors in the user preferences.
  /// For each descriptor that specifies an expression,
  /// execute the expression using the list of parameters.
  
  pub fn evaluate_user_descriptors(self: &Self) -> () {

    let calc_reg = self.calc_reg();

    let calc_expression = CalcExpression::new(
      self.calc_manager(), calc_reg.fiscal_year_start(false), calc_reg.decimal_digits(false));
    
    let expression = RefCell::new(calc_expression);

    let list_parameter = calc_reg.preferences().list_parameter();
    let list_descriptor = calc_reg.preferences().list_descriptor();

    CalcUtility::evaluate_descriptors(self.calc_manager(), &expression, list_parameter, list_descriptor);
  }

  /// Evaluate all of the descriptors in the selected cashflow.
  /// For each descriptor that specifies an expression,
  /// execute the expression using the list of parameters.

  pub fn evaluate_cashflow_descriptors(self: &Self) -> () {

    let calc_reg = self.calc_reg();

    let calc_expression = CalcExpression::new(
      self.calc_manager(), calc_reg.fiscal_year_start(false), calc_reg.decimal_digits(false));

    let preferences: &ElemPreferences; 
    let index = calc_reg.list_cashflow().index();
    match calc_reg.list_cashflow().list().iter().nth(index) {
      None => { panic!("Cashflow list index not set"); }
      Some(o) => { preferences = o.preferences(); }
    }

    let expression = RefCell::new(calc_expression);

    CalcUtility::evaluate_descriptors(
      self.calc_manager(), &expression, preferences.list_parameter(), preferences.list_descriptor());
  }

  /// Evaluate the event type expression for all events in the selected cashflow.

  pub fn evaluate_cashflow_event_type_all(self: &Self) -> () {    

    let mut list_result_symbol: Vec<Result<ElemSymbol, crate::ErrorType>> = Vec::new();
    let list_event: &ListEvent;
    let cfindex = self.calc_reg().list_cashflow().index();

    {
      let calc_reg = self.calc_reg();
      match calc_reg.list_cashflow().list().iter().nth(cfindex) {    
        None => { panic!("Cashflow list index not set"); }
        Some(o) => { list_event = o.list_event(); }
      }

      for elem in list_event.list().iter() {
        let group: String;
        match elem.elem_type() {
          crate::ExtensionType::CurrentValue => { group = String::from(crate::GROUP_CURRENT_VALUE); }
          crate::ExtensionType::StatisticValue => { group = String::from(crate::GROUP_STATISTIC_VALUE); }
          crate::ExtensionType::InterestChange => { group = String::from(crate::GROUP_INTEREST_CHANGE); }
          _ => { group = String::from(crate::GROUP_PRINCIPAL_CHANGE); }
        }

        let locale_str: String;
        if list_event.cashflow() {
          locale_str = String::from(calc_reg.reg().list_locale().cashflow_locale().locale_str());
        } else {
          locale_str = String::from(calc_reg.reg().list_locale().user_locale().locale_str());
        }
        
        let mut event_type_expr = String::from(calc_reg.descriptor_value(
          group.as_str(), crate::NAME_EVENT_TYPE, crate::TYPE_LOCALE, locale_str.as_str(), true, true));

        if event_type_expr.len() == 0 {
          event_type_expr = String::from(calc_reg.descriptor_value(
            group.as_str(), crate::NAME_EVENT_TYPE, "", "", true, true));
          if event_type_expr.len() == 0 {
            match elem.elem_type() {
              crate::ExtensionType::CurrentValue => { 
                event_type_expr =  String::from(list_event.get_resource(crate::USER_EVENT_TYPE_CURRENT_VALUE)); 
              }
              crate::ExtensionType::StatisticValue => { 
                event_type_expr =  String::from(list_event.get_resource(crate::USER_EVENT_TYPE_STATISTIC_VALUE)); 
              }
              crate::ExtensionType::InterestChange => { 
                event_type_expr =  String::from(list_event.get_resource(crate::USER_EVENT_TYPE_INTEREST_CHANGE)); 
              }
              _ => { 
                event_type_expr =  String::from(list_event.get_resource(crate::USER_EVENT_TYPE_PRINCIPAL_CHANGE)); 
              }
            }
          }
        }

        let mut core_expression = CalcExpression::new(
          self.calc_manager(), calc_reg.fiscal_year_start(true), calc_reg.decimal_digits(true));

        let preferences_cashflow = calc_reg.list_cashflow().preferences();

        let list_parameter = CalcUtility::create_event_type_list_parameter(
          self.calc_manager(), elem.elem_type(), elem.elem_extension());
      
        match preferences_cashflow.as_ref() {
          None => { panic!("Missing cashflow preferences"); }
          Some(o) => {
            core_expression.init_expression(
              Option::from(o.list_descriptor()), None,
              Option::from(&list_parameter), event_type_expr.as_str());
          }
        }

        let result = core_expression.evaluate(None, None);

        list_result_symbol.push(result);
      }
    }

    let mut errs: HashMap<usize, crate::ErrorType> = HashMap::new();

    {
      let mut reg = self.calc_reg_mut();
      match reg.list_cashflow_mut().list_mut().iter_mut().nth(cfindex) {
        None => { panic!("Cashflow list index not set"); }
        Some(o) => { 
          let list_event = o.list_event_mut();

          let mut index: usize = 0;
          for elem in list_event.list_mut().iter_mut() {
            let elem_result_symbol = list_result_symbol.get(index);
      
            match elem_result_symbol {
              None => { }
              Some(o) => {
                match o {
                  Err(e) => { 
                    errs.insert(index, *e);
                  }
                  Ok(o2) => { 
                    match o2.sym_type() {
                      crate::TokenType::Integer => { elem.set_event_type(format!("{}", o2.sym_integer()).as_str()); }
                      crate::TokenType::Decimal => { elem.set_event_type(format!("{}", o2.sym_decimal()).as_str()); }
                      crate::TokenType::String => { elem.set_event_type(o2.sym_string()); }
                      _ => { }
                    }
                  }
                }
              }
            }
      
            index += 1;
          }
        }
      }
    }    

    let mut errors: HashMap<usize, String> = HashMap::new();

    for (index, err) in errs.iter_mut() {
      let error_string = self.calc_reg_mut().get_error_string(*err);
      errors.insert(*index, error_string);
    }

    {
      let mut reg = self.calc_reg_mut();
      match reg.list_cashflow_mut().list_mut().iter_mut().nth(cfindex) {
        None => { panic!("Cashflow list index not set"); }
        Some(o) => { 
          let list_event = o.list_event_mut();

          let mut index: usize = 0;
          for elem in list_event.list_mut().iter_mut() {
            match errors.get(&index) { 
              None => { }
              Some(o) => { 
                elem.set_event_type(format!("{}{}", crate::ERROR_PREFIX, o).as_str());
              }
            }
      
            index += 1;
          }
        }
      }
    }    
  }

  /// Format and return a date string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The usize date value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_date(self: &Self, val: usize) -> String {

    let calc_reg = self.calc_reg();

    return calc_reg.reg().list_locale().format_date(val);
  }

  /// Format and return a integer string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The integer value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_integeri(self: &Self, val: i32) -> String {

    let calc_reg = self.calc_reg();

    return calc_reg.reg().list_locale().format_integeri(val);
  }

  /// Format and return an integer string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The usize value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_integer(self: &Self, val: usize) -> String {

    return self.format_integeri(val as i32);
  }

  /// Format and return a decimal string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The decimal value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_decimal(self: &Self, val: Decimal) -> String {

    let calc_reg = self.calc_reg();

    return calc_reg.reg().list_locale().format_decimal(val);
  }

  /// Format and return a currency string.
  /// 
  /// # Arguments
  ///
  /// * `val` - The decimal value to format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format_currency(self: &Self, val: Decimal) -> String {

    let calc_reg = self.calc_reg();
    let decimal_digits = calc_reg.decimal_digits(false);

    return calc_reg.reg().list_locale().format_currency(val, decimal_digits);
  }

  /// Return a rounded decimal.
  /// 
  /// # Arguments
  ///
  /// * `val` - The decimal value to round.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn round_decimal(self: &Self, val: Decimal) -> Decimal {

    return CoreUtility::util_round(val, crate::MAXIMUM_DISPLAY_DECIMAL_DIGITS);
  }

  /// Return a rounded currency.
  /// 
  /// # Arguments
  ///
  /// * `val` - The decimal value to round.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn round_currency(self: &Self, val: Decimal) -> Decimal {

    let calc_reg = self.calc_reg();
    let decimal_digits = calc_reg.decimal_digits(false);

    return CoreUtility::util_round(val, decimal_digits);
  }

}