//! The AmFn utility methods.
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
use rust_decimal::prelude::*;

use crate::{ListTrait,ElemLevelType};
use crate::core::{CoreUtility, ElemSymbol, ElemBalanceResult, ElemExtension, 
  ListDescriptor, ListParameter, ListEvent, ListAmortization, ListSummary, ListColumn};
use super::{CalcManager, CalcExpression};

pub struct CalcUtility {
}

/// The AmFn utility methods implementation.

impl CalcUtility {
  
  /// Create the event type parameter list.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `elem_type` - Type of the event.
  /// * `elem_extension` - A ElemPrincipalChange, ElemCurrentValue, 
  ///     ElemInterestChange, or ElemStatisticValue element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn create_event_type_list_parameter(calc_manager_param: &Rc<RefCell<CalcManager>>,
      elem_type: crate::ExtensionType, elem_extension: &ElemExtension) -> ListParameter {

    let calc_manager = Rc::clone(calc_manager_param);
    let mut list_parameter = ListParameter::new(calc_manager.borrow().core_manager(), ElemLevelType::Event);
    let updating_json = calc_manager.borrow().updating_json();
    
    match elem_type {
      crate::ExtensionType::CurrentValue => {
        list_parameter.add_parameter("intEOM", updating_json);
        list_parameter.set_integer(if elem_extension.cv_eom() { 1 } else { 0 });
        list_parameter.add_parameter("intPassive", updating_json);
        list_parameter.set_integer(if elem_extension.cv_passive() { 1 } else { 0 });
        list_parameter.add_parameter("intPresent", updating_json);
        list_parameter.set_integer(if elem_extension.cv_present() { 1 } else { 0 });
      }
      crate::ExtensionType::InterestChange => {
        list_parameter.add_parameter("strMethod", updating_json);
        list_parameter.set_string(CoreUtility::get_interest_method_mnemonic_short(
          elem_extension.ic_method()).as_str());
        list_parameter.add_parameter("strDayCount", updating_json);
        list_parameter.set_string(CoreUtility::get_day_count_basis_mnemonic_short(
          elem_extension.ic_day_count_basis()).as_str());
        list_parameter.add_parameter("intDaysInYear", updating_json);
        list_parameter.set_integer(elem_extension.ic_days_in_year());
        list_parameter.add_parameter("strEffFreq", updating_json);
        list_parameter.set_string(CoreUtility::get_frequency_mnemonic(
          elem_extension.ic_effective_frequency()).as_str());
        list_parameter.add_parameter("strExpFreq", updating_json);
        list_parameter.set_string(CoreUtility::get_frequency_mnemonic(
          elem_extension.ic_interest_frequency()).as_str());
        list_parameter.add_parameter("strRound", updating_json);
        list_parameter.set_string(CoreUtility::get_round_balance(
          elem_extension.ic_round_balance()).as_str());
        list_parameter.add_parameter("decRoundDD", updating_json);
        list_parameter.set_decimal(elem_extension.ic_round_decimal_digits());
      }
      crate::ExtensionType::StatisticValue => {
        list_parameter.add_parameter("strName", updating_json);
        list_parameter.set_string(elem_extension.sv_name());
        list_parameter.add_parameter("intEOM", updating_json);
        list_parameter.set_integer(if elem_extension.sv_eom() { 1 } else { 0 });
        list_parameter.add_parameter("intFinal", updating_json);
        list_parameter.set_integer(if elem_extension.sv_is_final() { 1 } else { 0 });
      }
      _ => {        
        list_parameter.add_parameter("strPrinType", updating_json);
        list_parameter.set_string(CoreUtility::get_principal_type_mnemonic_short(
          elem_extension.pc_type()).as_str());
        list_parameter.add_parameter("intEOM", updating_json);
        list_parameter.set_integer(if elem_extension.pc_eom() { 1 } else { 0 });
        list_parameter.add_parameter("intPrinFirst", updating_json);
        list_parameter.set_integer(if elem_extension.pc_principal_first() { 1 } else { 0 });
        list_parameter.add_parameter("intBalStats", updating_json);
        list_parameter.set_integer(if elem_extension.pc_balance_statistics() { 1 } else { 0 });
        list_parameter.add_parameter("intAuxiliary", updating_json);
        list_parameter.set_integer(if elem_extension.pc_auxiliary() { 1 } else { 0 });
        list_parameter.add_parameter("intPassive", updating_json);
        list_parameter.set_integer(if elem_extension.pc_aux_passive() { 1 } else { 0 });
      }
    }
    
    return list_parameter;
  }

  /// Evaluate all of the descriptors in the list.
  /// For each descriptor that specifies an expression,
  /// execute the expression using the list of parameters.
  ///
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `expression` - CalcExpression element.
  /// * `list_parameter` - List of parameters used with evaluation.
  /// * `list_descriptor` - List of descriptors to evaluate.

  pub fn evaluate_descriptors(calc_manager_param: &Rc<RefCell<CalcManager>>, 
      expression: &RefCell<CalcExpression>, list_parameter: &ListParameter, 
      list_descriptor: &ListDescriptor) -> () {

    let calc_manager = Rc::clone(calc_manager_param);
    let reg = calc_manager.borrow();

    let orig_index = list_descriptor.index();
    let mut index: usize = 0;

    loop {

      if !list_descriptor.get_element(index) { break; }
      
      if list_descriptor.desc_type() == crate::TYPE_LOCALE {
        reg.reg().list_locale_mut().select_event_locale(list_descriptor.code());
      }

      if list_descriptor.value_expr().len() == 0 { 
        index += 1;
        continue;
      }

      let mut list_descriptor_cashflow: Option<&ListDescriptor> = None;
      match reg.list_cashflow().preferences().as_ref() {
        None => { }
        Some(o) => { list_descriptor_cashflow = Option::from(o.list_descriptor()); }
      }
  
      expression.borrow_mut().init_expression(
        list_descriptor_cashflow, None, Option::from(
          list_parameter), list_descriptor.value_expr().as_str());

      let elem_result_symbol: ElemSymbol;
      let result = expression.borrow().evaluate(None, None);
      match result {
        Err(e) => {
          let error_string = reg.get_error_string(e);
          list_descriptor.set_value(format!("{}{}", crate::ERROR_PREFIX, error_string).as_str());
          reg.reg().list_locale_mut().select_event_locale("");
          index += 1;
          continue;
        }
        Ok(o) => { 
          elem_result_symbol = o;
        }
      }

      let value: String;
      match elem_result_symbol.sym_type() {
        crate::TokenType::Integer => { value = format!("{}", elem_result_symbol.sym_integer()); }
        crate::TokenType::Decimal => { value = format!("{}", elem_result_symbol.sym_decimal()); }
        crate::TokenType::String => { value = String::from(elem_result_symbol.sym_string()); }
        _ => { value = String::from(""); }
      }
      
      list_descriptor.set_value(value.as_str());
      
      reg.reg().list_locale_mut().select_event_locale("");

      index += 1;
    }

    list_descriptor.set_index(orig_index);
  }

  /// Evaluate the expression.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `list_parameter` - List of parameters used with evaluation.
  /// * `expression_str` - The expression to evaluate.
  /// * `cashflow` - Search the cashflow preferences.
  /// 
  /// # Return
  ///
  /// * Resulting symbol if successful, otherwise an error
  ///     message in the symbol.
  
  pub fn evaluate_expression(calc_manager_param: &Rc<RefCell<CalcManager>>,
      list_parameter: &ListParameter, expression_str: &str, cashflow: bool) -> ElemSymbol {

    let calc_manager = Rc::clone(calc_manager_param);
    let reg = calc_manager.borrow();
    let mut list_descriptor_cashflow: Option<&ListDescriptor> = None;

    match reg.list_cashflow().preferences().as_ref() {
      None => { }
      Some(o) => { list_descriptor_cashflow = Option::from(o.list_descriptor()); }
    }

    let mut expression = CalcExpression::new(
      &calc_manager, reg.fiscal_year_start(cashflow), reg.decimal_digits(cashflow));

    expression.init_expression(
      list_descriptor_cashflow, None, Option::from(list_parameter), expression_str);

    let mut elem_result_symbol: ElemSymbol;
    let result = expression.evaluate(None, None);
    match result {
      Err(e) => {
        elem_result_symbol = ElemSymbol::new();
        let error_string = reg.get_error_string(e);
        elem_result_symbol.set_string(format!("{}{}", crate::ERROR_PREFIX, error_string).as_str());
      }
      Ok(o) => { 
        elem_result_symbol = o;
      }
    }

    return elem_result_symbol;
  }

  /// Get the appropriate event list value as a string.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `list_column` - List of columns object.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn get_event_value(
    calc_manager_param: &Rc<RefCell<CalcManager>>, list_column: &ListColumn) -> String {

    let calc_manager = Rc::clone(calc_manager_param);
    let calc_reg = calc_manager.borrow();
    let decimal_digits = calc_reg.decimal_digits(true);
    let reg = calc_reg.reg();
    let mut list_locale = reg.list_locale_mut();
    let list_cashflow = calc_reg.list_cashflow();
    let list_event_opt = list_cashflow.list_event();    
    let mut result = String::from("");

    let list_event: &ListEvent;
    match list_event_opt.as_ref() {
      None => { return result; }
      Some(o) => { list_event = o; }
    }      

    let orig_list_index = list_event.index();
    
    list_locale.select_event_locale("");
    
    if list_column.col_type() == crate::TYPE_LOCALE && list_column.code().len() > 0 {
      list_locale.select_event_locale(list_column.code());
    }
    
    match CoreUtility::get_col_name(list_column.col_name_index()) {
      crate::ColumnType::None => {
        let list_descriptor_opt = list_event.list_descriptor();
        match list_descriptor_opt.as_ref() {
          None => { return result; }
          Some(o) => {
            if o.get_element_by_name(
                list_column.group(), list_column.name(), list_column.col_type(), 
                list_column.code(), true) {
              match list_column.format() {
                crate::FormatType::Date => {
                  match o.value().parse::<usize>() {
                    Err(_e) => { }
                    Ok(o2) => { result = String::from(list_locale.format_date(o2)); }
                  }              
                }
                crate::FormatType::Integer => {
                  match o.value().parse::<i32>() {
                    Err(_e) => { }
                    Ok(o2) => { result = String::from(list_locale.format_integeri(o2)); }
                  }              
                }
                crate::FormatType::Decimal => {
                  match o.value().parse::<Decimal>() {
                    Err(_e) => { }
                    Ok(o2) => { result = String::from(list_locale.format_decimal(o2)); }
                  }              
                }
                crate::FormatType::Currency => {
                  match o.value().parse::<Decimal>() {
                    Err(_e) => { }
                    Ok(o2) => { result = String::from(list_locale.format_currency(
                      calc_manager.borrow().util_convert_currency_event(o2), decimal_digits)); }
                  }              
                }
                _ => {
                  result = String::from(o.value());
                }
              }
            }
          }
        }
      }
      crate::ColumnType::EventType => {
        result = String::from(list_event.event_type());
      }
      crate::ColumnType::Date => {
        result = String::from(list_locale.format_date(list_event.event_date()));
      }
      crate::ColumnType::DateExpr => {
        if list_event.date_expr().len() > 0 {
          result = String::from(list_event.date_expr());
        }
      }
      crate::ColumnType::Sort => {
        result = list_event.sort_order().to_string();
      }
      crate::ColumnType::Value => {
        match list_event.elem_type() {
          crate::ExtensionType::PrincipalChange => {
            result = String::from(list_locale.format_currency(
              calc_manager.borrow().util_convert_currency_event(list_event.value()), decimal_digits));
          }
          crate::ExtensionType::InterestChange => {
            result = String::from(list_locale.format_decimal(list_event.value()));
          }
          _ => {
          }
        }
      }
      crate::ColumnType::ValueExpr => {
        if list_event.value_expr().len() > 0 {
          result = String::from(list_event.value_expr());
        }
      }
      crate::ColumnType::Periods => {
        if list_event.elem_type() != crate::ExtensionType::StatisticValue {
          result = format!("{}", list_event.periods());
        }
      }
      crate::ColumnType::PeriodsExpr => {
        if list_event.periods_expr().len() > 0 {
          result = String::from(list_event.periods_expr());
        }
      }
      crate::ColumnType::SkipPeriods => {
        if list_event.skip_mask_len() > 0 {
          result = format!("{}/{}", CoreUtility::skip_mask_true_bits(
            list_event.skip_mask_len(), list_event.skip_mask()), list_event.skip_mask_len());
        }
      }
      crate::ColumnType::Intervals => {
        result = format!("{}", list_event.intervals());
      }
      crate::ColumnType::Frequency => {
        calc_manager.borrow().reg().map_frequency().get_element_by_value(list_event.frequency() as usize);
        result = String::from(list_locale.get_resource(calc_manager.borrow().reg().map_frequency().key()));
      }
      crate::ColumnType::EndDate => {
        if list_event.periods() > 1 {
          let val = CalcManager::util_date_new(
            list_event.event_date(), list_event.periods() - 1,
            list_event.frequency(), list_event.intervals(), list_event.elem_extension().extension_eom());        
          result = String::from(list_locale.format_date(val));
        }
      }
      crate::ColumnType::ParameterList => {
        match list_event.list_parameter().as_ref() {
          None => { result = String::from(""); }
          Some(o) => {
            if o.count() > 0 {
              result = format!("{}", o.count());
            }
          }
        }
      }
      crate::ColumnType::DescriptorList => {
        match list_event.list_descriptor().as_ref() {
          None => { result = String::from(""); }
          Some(o) => {
            if o.count() > 0 {
              result = format!("{}", o.count());
            }
          }
        }
      }
      crate::ColumnType::EventName => {
        if list_event.event_name().len() > 0 {
          result = String::from(list_event.event_name());
        }
      }
      crate::ColumnType::NextName => {
        if list_event.next_name().len() > 0 {
          result = String::from(list_event.next_name());
        }
      }
      _  => {
      }
    }
    
    list_event.get_element(orig_list_index);
    list_locale.select_event_locale("");
    
    return result;
  }

  /// Get the appropriate amortization list value as a string.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `list_column` - List of columns object.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn get_am_value(
    calc_manager_param: &Rc<RefCell<CalcManager>>, list_column: &ListColumn) -> String {

    let calc_manager = Rc::clone(calc_manager_param);
    let calc_reg = calc_manager.borrow();
    let decimal_digits = calc_reg.decimal_digits(true);
    let reg = calc_reg.reg();
    let mut list_locale = reg.list_locale_mut();
    let list_cashflow = calc_reg.list_cashflow();
    let list_am_opt = list_cashflow.list_amortization();      
    let elem_balance_result_opt = list_cashflow.elem_balance_result();

    let list_am: &ListAmortization;
    match list_am_opt.as_ref() {
      None => { return String::from(""); }
      Some(o) => { list_am = o; }
    }      

    let elem_balance_result: &ElemBalanceResult;
    match elem_balance_result_opt.as_ref() {
      None => { return String::from(""); }
      Some(o) => { elem_balance_result = o; }
    }

    let orig_list_index = list_am.index();
    if orig_list_index == usize::MAX {
      return String::from("");
    }

    list_locale.select_event_locale("");
    
    if list_column.col_type() == crate::TYPE_LOCALE && list_column.code().len() > 0 {
      list_locale.select_event_locale(list_column.code());
    }

    let mut result = String::from("");    
    
    match CoreUtility::get_col_name(list_column.col_name_index()) {
      crate::ColumnType::None => {
        let list_descriptor_opt = list_am.list_descriptor();
        match list_descriptor_opt.as_ref() {
          None => { }
          Some(o) => {
            if o.get_element_by_name(list_column.group(), list_column.name(), 
                list_column.col_type(), list_column.code(), true) {
              match list_column.format() {
                crate::FormatType::Date => {
                  match o.value().parse::<usize>() {
                    Err(_e) => { }
                    Ok(o) => { result = String::from(list_locale.format_date(o)); }
                  }              
                }
                crate::FormatType::Integer => {
                  match o.value().parse::<i32>() {
                    Err(_e) => { }
                    Ok(o) => { result = String::from(list_locale.format_integeri(o)); }
                  }              
                }
                crate::FormatType::Decimal => {
                  match o.value().parse::<Decimal>() {
                    Err(_e) => { }
                    Ok(o) => { result = String::from(list_locale.format_decimal(o)); }
                  }              
                }
                crate::FormatType::Currency => {
                  match o.value().parse::<Decimal>() {
                    Err(_e) => { }
                    Ok(o) => { 
                      result = String::from(list_locale.format_currency(
                        calc_manager.borrow().util_convert_currency_event(o), decimal_digits)); 
                    }
                  }              
                }
                _ => {
                  result =  String::from(o.value());
                }
              }
            }
          }
        }
      }
      crate::ColumnType::Sequence => {
        let val = list_am.stat_sequence();
        if val > 0 {
          result = format!("{}", val);
        }
      }
      crate::ColumnType::EventType => {
        result = String::from(list_am.event_type());
      }
      crate::ColumnType::Date => {
        result = String::from(list_locale.format_date(list_am.event_date()));
      }
      crate::ColumnType::Sort => {
        result = format!("{}", list_am.sort_order());
      }
      crate::ColumnType::Value => {
        match list_am.elem_type() {
          crate::ExtensionType::PrincipalChange => {
            result = String::from(list_locale.format_currency(
              calc_manager.borrow().util_convert_currency_event(list_am.value()), decimal_digits));
          }
          crate::ExtensionType::InterestChange => {
            result = String::from(list_locale.format_decimal(list_am.value()));
          }
          _ => {            
          }
        }
      }
      crate::ColumnType::Decrease => {
        if list_am.principal_decrease() > dec!(0.0) {
          result = String::from(list_locale.format_currency(
            calc_manager.borrow().util_convert_currency_event(list_am.principal_decrease()), decimal_digits));
        }
      }
      crate::ColumnType::Increase => {
        match list_am.elem_type() {
          crate::ExtensionType::InterestChange => {
            result = String::from(list_locale.format_decimal(list_am.value()));
          }
          _ => {
            if list_am.principal_increase() > dec!(0.0) {
              result = String::from(list_locale.format_currency(
                calc_manager.borrow().util_convert_currency_event(list_am.principal_increase()), decimal_digits));
            }
          }
        }
      }
      crate::ColumnType::Intervals => {
        result = format!("{}", list_am.intervals());
      }
      crate::ColumnType::Frequency => {
        calc_manager.borrow().reg().map_frequency().get_element_by_value(list_am.frequency() as usize);
        result = String::from(list_locale.get_resource(calc_manager.borrow().reg().map_frequency().key()));
      }
      crate::ColumnType::ParameterList => {
        match list_am.list_parameter().as_ref() {
          None => { result = String::from(""); }
          Some(o) => {
            if o.count() > 0 {
              result = format!("{}", o.count());
            }
          }
        }
      }
      crate::ColumnType::DescriptorList => {
        match list_am.list_descriptor().as_ref() {
          None => { result = String::from(""); }
          Some(o) => {
            if o.count() > 0 {
              result = format!("{}", o.count());
            }
          }
        }
      }
      crate::ColumnType::Interest => {
        if !(list_am.interest() == dec!(0.0) && 
            list_am.elem_type() == crate::ExtensionType::StatisticValue) {
          result = String::from(list_locale.format_currency(
            calc_manager.borrow().util_convert_currency_event(list_am.interest()), decimal_digits));
        }
      }
      crate::ColumnType::SlInterest => {
        if !(list_am.sl_interest() == dec!(0.0) && 
            list_am.elem_type() == crate::ExtensionType::StatisticValue) {
          result = String::from(list_locale.format_currency(
            calc_manager.borrow().util_convert_currency_event(list_am.sl_interest()), decimal_digits));
        }
      }
      crate::ColumnType::IntOnInterest => {
        if !(list_am.interest() - list_am.sl_interest() == dec!(0.0) && 
            list_am.elem_type() == crate::ExtensionType::StatisticValue) {
          result = String::from(list_locale.format_currency(
            calc_manager.borrow().util_convert_currency_event(
              list_am.interest() - list_am.sl_interest()), decimal_digits));
        }
      }
      crate::ColumnType::ValueToInterest => {
        if !(list_am.value_to_interest() == dec!(0.0) && 
            list_am.elem_type() == crate::ExtensionType::StatisticValue) {
          result = String::from(list_locale.format_currency(
            calc_manager.borrow().util_convert_currency_event(list_am.value_to_interest()), decimal_digits));
        }
      }
      crate::ColumnType::ValueToPrincipal => {
        if !(list_am.value_to_principal() == dec!(0.0) && 
            list_am.elem_type() == crate::ExtensionType::StatisticValue) {
          result = String::from(list_locale.format_currency(
            calc_manager.borrow().util_convert_currency_event(list_am.value_to_principal()), decimal_digits));
        }
      }
      crate::ColumnType::AccruedBalance => {
        if !(list_am.acc_balance() == dec!(0.0) && 
            list_am.elem_type() == crate::ExtensionType::StatisticValue) {
          result = String::from(list_locale.format_currency(
            calc_manager.borrow().util_convert_currency_event(list_am.acc_balance()), decimal_digits));
        }
      }
      crate::ColumnType::Balance => {
        if !(list_am.balance() == dec!(0.0) && 
            list_am.elem_type() == crate::ExtensionType::StatisticValue) {
          let balance = CoreUtility::round(
            list_am.balance(), list_column.decimal_digits(), crate::RoundType::Bankers);
          if elem_balance_result.polarity() < 0 {
            if balance > dec!(0.0) {
              result = format!("+{}", String::from(list_locale.format_currency(
                calc_manager.borrow().util_convert_currency_event(balance), decimal_digits)));
            } else {
              result = String::from(list_locale.format_currency(
                calc_manager.borrow().util_convert_currency_event(balance.abs()), decimal_digits));
            }
          } else {
            result = String::from(list_locale.format_currency(
              calc_manager.borrow().util_convert_currency_event(balance), decimal_digits));
          }
        }
      }
      _ => {        
      }
    }
    
    list_locale.select_event_locale("");
    list_am.get_element(orig_list_index);
    
    return result;
  }

  /// Determine if the column is empty.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `list_column` - List of columns object.
  /// * `elem_type` - The type of table.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn is_column_empty(calc_manager_param: &Rc<RefCell<CalcManager>>, 
    list_column: &ListColumn, elem_type: crate::TableType) -> bool {

    let calc_manager = Rc::clone(calc_manager_param);
    let reg = calc_manager.borrow();
    let list_cashflow = reg.list_cashflow();
    let list_am_opt = list_cashflow.list_amortization();      
    let list_event_opt = list_cashflow.list_event();          
    let count;

    match elem_type {
      crate::TableType::Amortization => {
        match list_am_opt.as_ref() {
          None => { return false; }
          Some(o) => { count = o.count(); }
        }
      }
      _ => {
        match list_event_opt.as_ref() {
          None => { return false; }
          Some(o) => { count = o.count(); }
        }
      }
    }

    let mut result: String;
    let mut index: usize = 0;

    while index < count {

      match elem_type {
        crate::TableType::Amortization => {
          match list_am_opt.as_ref() {
            None => { return false; }
            Some(o) => { 
              if !o.get_element(index) { break; }
              result = String::from(CalcUtility::get_am_value(&calc_manager, list_column));
            }
          }
        }
        _ => {
          match list_event_opt.as_ref() {
            None => { return false; }
            Some(o) => { 
              if !o.get_element(index) { break; }
              result = String::from(CalcUtility::get_event_value(&calc_manager, list_column));
            }
          }
        }
      }

      index += 1;

      if result == crate::EMPTY_DISPLAY {
        continue;
      }

      let dval: Decimal = CoreUtility::parse_decimal(result.as_str());
      if dval <= list_column.column_empty_value() {
        continue;
      }
        
      break;
    }
        
    return index >= count;    
  }

  /// Normalize the expression.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `expression` - The expression to normalize.
  /// * `new_line` - If true, format with a newline character between expressions.
  /// 
  /// # Return
  ///
  /// * Normalized expression.
  
  pub fn normalize_expression(calc_manager_param: &Rc<RefCell<CalcManager>>, 
    expression: &str, new_line: bool) -> String {

    let calc_manager = Rc::clone(calc_manager_param);

    let mut calc_expression = CalcExpression::new(
      &calc_manager, calc_manager.borrow().fiscal_year_start(false), 
      calc_manager.borrow().decimal_digits(false));
        
    calc_expression.init_expression(None, None, None, expression);
    
    return calc_expression.normalize_expression(new_line);
  }

  /// Create and return a column list object.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// * `elem_type` - The type of table.
  /// * `cashflow` - Search the cashflow preferences.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn parse_columns(calc_manager_param: &Rc<RefCell<CalcManager>>, 
    elem_type: crate::TableType, cashflow: bool) -> ListColumn {

    let calc_manager = Rc::clone(calc_manager_param);
    let calc_reg = calc_manager.borrow();
    let reg = calc_reg.reg();
    let list_locale = reg.list_locale();
        
    let mut group = String::from(if !cashflow { crate::GROUP_TEMPLATE } else
      if elem_type == crate::TableType::Amortization { crate::GROUP_AM } else { crate::GROUP_EVENT });
    
    let locale_str: &str;
    if cashflow {
      locale_str = list_locale.cashflow_locale().locale_str();
    } else { 
      locale_str = list_locale.user_locale().locale_str();
    }

    let mut columns = calc_manager.borrow().descriptor_value(
      group.as_str(), crate::NAME_COLUMNS, crate::TYPE_LOCALE, locale_str, true, false);

    if columns.len() == 0 {
      columns = calc_manager.borrow().descriptor_value(
        group.as_str(), crate::NAME_COLUMNS, "", "", true, false);
      if columns.len() == 0 {
        columns = String::from(if !cashflow { crate::DEFAULT_TEMPLATE_COLUMNS } else
          if elem_type == crate::TableType::Amortization { crate::DEFAULT_AM_COLUMNS } else 
            { crate::DEFAULT_EVENT_COLUMNS });
      }
    }
    
    let mut list_column = ListColumn::new();
    
    for column in columns.split("|") {
      let text = column.trim();
      let mut col_name = CoreUtility::parse_token1(text);
      let mut width = CoreUtility::parse_integer(CoreUtility::parse_token2(text));

      if width < crate::MINIMUM_COLUMN_WIDTH {
        width = crate::MINIMUM_COLUMN_WIDTH;
      }

      group = String::from("");

      let mut col_header: String;
      let mut col_description = String::from("");
      let mut name = String::from("");
      let mut elem_type = String::from("");
      let mut code = String::from("");
      let mut col_empty_value: Decimal = dec!(-1.0);
      let mut format = crate::FormatType::String;
      let mut col_name_index: usize = 0;
      let mut decimal_digits = calc_manager.borrow().decimal_digits(cashflow);
      let mut column_exclude: bool = false;
      let mut desc_text: String;

      if calc_manager.borrow().reg().map_col_names().get_element_by_key(col_name) {
        col_name_index = calc_manager.borrow().reg().map_col_names().value();

        col_header = String::from(calc_manager.borrow().descriptor_value(
          crate::GROUP_COLHEADER, col_name, crate::TYPE_LOCALE, locale_str, true, false));
        if col_header.len() == 0 {
          col_header = String::from(calc_manager.borrow().descriptor_value(
            crate::GROUP_COLHEADER, col_name, "", "", true, false));
          if col_header.len() == 0 {
            col_header = String::from(list_locale.get_resource(
              CalcManager::col_name_resource_key(CoreUtility::get_col_name(col_name_index)).as_str()));
            if (calc_manager.borrow().reg().map_col_names().value_ext() & crate::MAPCOLNAMES_EMPTY) != 0 {
              col_empty_value = dec!(0.0);
            }
            column_exclude = (calc_manager.borrow().reg().map_col_names().value_ext() & 
              crate::MAPCOLNAMES_EXCLUDE) != 0;
          }
        }
      } else {
        desc_text = String::from(calc_manager.borrow().descriptor_value(
          crate::GROUP_COLVALUE, col_name, crate::TYPE_LOCALE, locale_str, true, false));
        if desc_text.len() == 0 {
          desc_text = String::from(calc_manager.borrow().descriptor_value(
            crate::GROUP_COLVALUE, col_name, "", "", true, false));
          if desc_text.len() == 0 {
            continue;
          }
        }

        let tokens: Vec<_> = desc_text.split("~").collect();
        if tokens.len() != 3 && tokens.len() != 6 {
          continue;
        }

        if tokens.len() == 3 {
          col_name = tokens[0].trim();
          elem_type = crate::TYPE_LOCALE.to_string();
          code = String::from(tokens[1].trim());
          decimal_digits = CoreUtility::parse_integer(tokens[2].trim());

          if !calc_manager.borrow().reg().map_col_names().get_element_by_key(col_name) {
            continue;
          }

          col_name_index = calc_manager.borrow().reg().map_col_names().value();

          col_header = String::from(calc_manager.borrow().descriptor_value(
            crate::GROUP_COLHEADER, col_name, elem_type.as_str(), code.as_str(), true, false));
          if col_header.len() == 0 {
            col_header = String::from(calc_manager.borrow().descriptor_value(
              crate::GROUP_COLHEADER, col_name, "", "", true, false));
            if col_header.len() == 0 {
              reg.list_locale_mut().select_event_locale(code.as_str());
              col_header = String::from(list_locale.get_resource(
                CalcManager::col_name_resource_key(CoreUtility::get_col_name(col_name_index)).as_str()));
              reg.list_locale_mut().select_event_locale("");
            }
          }
        } else {
          group = String::from(tokens[0].trim());
          name = String::from(tokens[1].trim());
          elem_type = String::from(tokens[2].trim());
          code = String::from(tokens[3].trim());
          format = CoreUtility::get_format(CoreUtility::parse_integer(tokens[4].trim()));
          decimal_digits = CoreUtility::parse_integer(tokens[5].trim());

          col_header = String::from(calc_manager.borrow().descriptor_value(
            crate::GROUP_COLHEADER, col_name, elem_type.as_str(), code.as_str(), true, false));
          if col_header.len() > 0 {
            col_header = String::from(calc_manager.borrow().descriptor_value(
              crate::GROUP_COLHEADER, col_name, "", "", true, false));
          }
        }
      }

      let col_header_str = col_header;

      let tokens: Vec<_> = col_header_str.split("~").collect();
      if tokens.len() < 1 {
        continue;
      }
      
      col_header = String::from(tokens[0].trim());        
      if tokens.len() > 1 {
        col_description = String::from(tokens[1].trim());
        if tokens.len() > 3 {
          col_empty_value = CoreUtility::parse_decimal(tokens[2].trim());
          column_exclude = CoreUtility::parse_integer(tokens[3].trim()) != 0;
        }
      }
      
      if col_header.len() == 0 {
        col_header = String::from(col_name);
      }
      
      let col = CoreUtility::get_col_name(col_name_index);
      match col {
        crate::ColumnType::Balance => { format = crate::FormatType::Currency; }
        crate::ColumnType::EndDate =>  { format = crate::FormatType::Date; }
        _ => { }
      }
      
      list_column.add_column(col_name, col_name_index, col_header.as_str(), 
        col_description.as_str(), group.as_str(), name.as_str(), elem_type.as_str(), 
        code.as_str(), col_empty_value, format, decimal_digits, width, column_exclude);
    }
    
    return list_column;
  }

  /// Create and return a summary list object.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn parse_summary(calc_manager_param: &Rc<RefCell<CalcManager>>) -> ListSummary {
    
    let calc_manager = Rc::clone(calc_manager_param);
    let calc_reg = calc_manager.borrow();
    let reg = calc_reg.reg();
    let list_locale = reg.list_locale();
    let list_cashflow = calc_reg.list_cashflow();     
    let preferences = list_cashflow.preferences();

    let mut list_summary = ListSummary::new();
    let locale_str = list_locale.cashflow_locale().locale_str();

    let list_parameter: &ListParameter;
    match preferences.as_ref() {
      None => { return list_summary; }
      Some(o) => { list_parameter = o.list_parameter(); }
    }

    let list_descriptor: &ListDescriptor;
    match preferences.as_ref() {
      None => { return list_summary; }
      Some(o) => { list_descriptor = o.list_descriptor(); }
    }
    
    let mut calc_expression = CalcExpression::new(
      &calc_manager, calc_reg.fiscal_year_start(true), calc_reg.decimal_digits(true));
    
    let mut summary = calc_reg.descriptor_value(
      crate::GROUP_GENERAL, crate::NAME_SUMMARY, crate::TYPE_LOCALE, locale_str, true, false);
    if summary.len() == 0 {
      summary = calc_reg.descriptor_value(crate::GROUP_GENERAL, crate::NAME_SUMMARY, "", "", true, false);
      if summary.len() == 0 {
        return list_summary;
      }
    }
    
    for summary in summary.split("|") {
      let name = summary.trim();

      let mut text = calc_reg.descriptor_value(
        crate::GROUP_SUMMARY, name, crate::TYPE_LOCALE, locale_str, true, false);

      if text.len() == 0 {
        text = calc_reg.descriptor_value(crate::GROUP_SUMMARY, name , "", "", true, false);
        if text.len() == 0 {
          continue;
        }
      }

      let tokens: Vec<_> = text.split("~").collect();
      let label_expr: String;
      let result_expr: String;

      if tokens.len() < 2 {
        label_expr = String::from(name);
        result_expr = String::from(text);
      } else {
        label_expr = String::from(tokens[0].trim());
        result_expr = String::from(tokens[1].trim());
      }
        
      calc_expression.init_expression(
        Option::from(list_descriptor), None, Option::from(list_parameter), label_expr.as_str());

      let mut label_str = String::from("");

      let mut elem_result_symbol: ElemSymbol;
      let result = calc_expression.evaluate(
        list_cashflow.list_amortization(), list_cashflow.elem_balance_result());

      match result {
        Err(e) => {         
          let error_string = calc_reg.get_error_string(e);
          label_str = format!("{}{}", crate::ERROR_PREFIX, error_string);
        }
        Ok(o) => {
          elem_result_symbol = o;

          match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
              label_str = elem_result_symbol.sym_integer().to_string();
            }
            crate::TokenType::Decimal => {
              label_str = String::from(list_locale.format_decimal(elem_result_symbol.sym_decimal()));
            }
            crate::TokenType::String => {
              label_str = String::from(elem_result_symbol.sym_string());
            }
            _ => {            
            }
          }
        }
      }
      
      if label_str.len() == 0 {
        continue; // Omit summary item
      }

      calc_expression.init_expression(
        Option::from(list_descriptor), None, Option::from(list_parameter), result_expr.as_str());

      let mut result_str = String::from("");
      let result = calc_expression.evaluate(
        list_cashflow.list_amortization(), list_cashflow.elem_balance_result());
      match result {
        Err(e) => {
          let error_string = calc_reg.get_error_string(e);
          result_str = format!("{}{}", crate::ERROR_PREFIX, error_string);
        }
        Ok(o) => {
          elem_result_symbol = o;

          match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
              result_str = elem_result_symbol.sym_integer().to_string();
            }
            crate::TokenType::Decimal => {
              result_str = String::from(list_locale.format_decimal(elem_result_symbol.sym_decimal()));
            }
            crate::TokenType::String => {
              result_str = String::from(elem_result_symbol.sym_string());
            }
            _ => {
            }
          }
        }
      }
      
      list_summary.add_summary(
        name, label_str.as_str(), label_expr.as_str(), result_str.as_str(), result_expr.as_str());
    }
    
    return list_summary;
  }

}