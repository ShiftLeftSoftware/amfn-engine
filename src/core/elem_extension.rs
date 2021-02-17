//! The extension enumeration value of an event.
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

use crate::{ElemUpdateType, ElemLevelType};
use super::{CoreUtility, ElemPrincipalChange, ElemCurrentValue, ElemInterestChange, ElemStatisticValue, CoreManager};

pub enum ExtensionValue {
  PrincipalChange(ElemPrincipalChange),
  CurrentValue(ElemCurrentValue),
  InterestChange(ElemInterestChange),
  StatisticValue(ElemStatisticValue)
}

/// The extension element of an event.
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

pub struct ElemExtension {

  /// CoreManager element. 
  core_manager: Rc<RefCell<CoreManager>>,

  /// Extension value. 
  extension_value: ExtensionValue
  
}

/// The extension implementation.

impl ElemExtension {

  /// Return a new current value event.
  /// 
  /// # Arguments
  ///
  /// * `current_value_param` - Principal change to add.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new_current_value(core_manager_param: &Rc<RefCell<CoreManager>>, 
    current_value_param: ElemCurrentValue) -> ElemExtension {

    return ElemExtension::new(core_manager_param, ExtensionValue::CurrentValue(current_value_param));
  }

  /// Return a new interest change event.
  /// 
  /// # Arguments
  ///
  /// * `interest_change_param` - Principal change to add.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new_interest_change(core_manager_param: &Rc<RefCell<CoreManager>>, 
    interest_change_param: ElemInterestChange) -> ElemExtension {

    return ElemExtension::new(core_manager_param, ExtensionValue::InterestChange(interest_change_param));
  }

  /// Return a new principal change event.
  /// 
  /// # Arguments
  ///
  /// * `principal_change_param` - Principal change to add.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new_principal_change(core_manager_param: &Rc<RefCell<CoreManager>>, 
    principal_change_param: ElemPrincipalChange) -> ElemExtension {

    return ElemExtension::new(core_manager_param, ExtensionValue::PrincipalChange(principal_change_param));
  }

  /// Return a new statistic value event.
  /// 
  /// # Arguments
  ///
  /// * `statistic_value_param` - Principal change to add.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new_statistic_value(core_manager_param: &Rc<RefCell<CoreManager>>, 
    statistic_value_param: ElemStatisticValue) -> ElemExtension {

    return ElemExtension::new(core_manager_param, ExtensionValue::StatisticValue(statistic_value_param));
  }

  /// Create a extension element.
  /// 
  /// # Arguments
  ///
  /// * `extension_value_param` - Extension value.
  /// 
  /// # Return
  ///
  /// * See description.

  fn new(core_manager_param: &Rc<RefCell<CoreManager>>, 
    extension_value_param: ExtensionValue) -> ElemExtension {

    return ElemExtension {
      core_manager: Rc::clone(core_manager_param),
      extension_value: extension_value_param
    }
  }

  /// Copy this extension element as a new extension element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ElemExtension {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { 
        return ElemExtension::new(&self.core_manager, ExtensionValue::PrincipalChange(o.copy())); 
      }
      ExtensionValue::CurrentValue(o) => { 
        return ElemExtension::new(&self.core_manager, ExtensionValue::CurrentValue(o.copy())); 
      }
      ExtensionValue::InterestChange(o) => { 
        return ElemExtension::new(&self.core_manager, ExtensionValue::InterestChange(o.copy())); 
      }
      ExtensionValue::StatisticValue(o) => { 
        return ElemExtension::new(&self.core_manager, ExtensionValue::StatisticValue(o.copy())); 
      }
    }
  }

  /// Compare this extension to the extension parameter.
  /// 
  /// # Arguments
  ///
  /// * `elem` - Extension to compare.
  /// 
  /// # Return
  ///
  /// * True if equal, otherwise false.

  pub fn equal(self: &Self, elem: &ElemExtension) -> bool {

    if self.extension_type() != elem.extension_type() { return false; }

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { 
        match elem.extension_value() {
          ExtensionValue::PrincipalChange(o2) => { return o.equal(o2); }
          ExtensionValue::CurrentValue(_o2) => { return false; }
          ExtensionValue::InterestChange(_o2) => { return false; }
          ExtensionValue::StatisticValue(_o2) => { return false; }
        }
      }
      ExtensionValue::CurrentValue(o) => {
        match elem.extension_value() {
          ExtensionValue::PrincipalChange(_o2) => { return false; }
          ExtensionValue::CurrentValue(o2) => { return o.equal(o2); }
          ExtensionValue::InterestChange(_o2) => { return false; }
          ExtensionValue::StatisticValue(_o2) => { return false; }
        }
      }
      ExtensionValue::InterestChange(o) => { 
        match elem.extension_value() {
          ExtensionValue::PrincipalChange(_o2) => { return false; }
          ExtensionValue::CurrentValue(_o2) => { return false; }
          ExtensionValue::InterestChange(o2) => { return o.equal(o2); }
          ExtensionValue::StatisticValue(_o2) => { return false; }
        }
      }
      ExtensionValue::StatisticValue(o) => { 
        match elem.extension_value() {
          ExtensionValue::PrincipalChange(_o2) => { return false; }
          ExtensionValue::CurrentValue(_o2) => { return false; }
          ExtensionValue::InterestChange(_o2) => { return false; }
          ExtensionValue::StatisticValue(o2) => { return o.equal(o2); }
        }
      }
    }
  }

  /// Get the extension type.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn extension_type(self: &Self) -> crate::ExtensionType {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(_o) => { return crate::ExtensionType::PrincipalChange; }
      ExtensionValue::CurrentValue(_o) => { return crate::ExtensionType::CurrentValue; }
      ExtensionValue::InterestChange(_o) => { return crate::ExtensionType::InterestChange; }
      ExtensionValue::StatisticValue(_o) => { return crate::ExtensionType::StatisticValue; }
    }
  }

  /// Get the principal change.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn extension_value(self: &Self) -> &ExtensionValue {

    return &self.extension_value;
  }

  /// Get the mut principal change.
  /// 
  /// # Return
  ///
  /// * See description.

  fn extension_value_mut(self: &mut Self) -> &mut ExtensionValue {

    return &mut self.extension_value;
  }

  /// Get the end-of-month.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn extension_eom(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { return o.eom(); }
      ExtensionValue::CurrentValue(o) => { return o.eom(); }
      ExtensionValue::InterestChange(_o) => { return false; }
      ExtensionValue::StatisticValue(o) => { return o.eom(); }
    }
  }

  /// Get the principal change type.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn pc_type(self: &Self) -> crate::PrincipalType {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { return o.pc_type(); }
      _ => { return crate::PrincipalType::Increase }
    }
  }

  /// Get the value of adjust successive dates to end of month.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn pc_eom(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { return o.eom(); }
      _ => { return false; }
    }
  }

  /// Get the value of apply change to principal balance first for simple interest.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn pc_principal_first(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { return o.principal_first(); }
      _ => { return false; }
    }
  }

  /// Get the value of include with balance result statistics.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn pc_balance_statistics(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { return o.balance_statistics(); }
      _ => { return false; }
    }
  }

  /// Get the auxiliary principal change event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn pc_auxiliary(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { return o.auxiliary(); }
      _ => { return false; }
    }
  }

  /// Get the auxiliary passive principal change event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn pc_aux_passive(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::PrincipalChange(o) => { return o.aux_passive(); }
      _ => { return false; }
    }
  }

  /// Get the value to adjust successive dates to end of month.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cv_eom(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::CurrentValue(o) => { return o.eom(); }
      _ => { return false; }
    }
  }

  /// Get the value to not affect the remaining cashflow.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cv_passive(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::CurrentValue(o) => { return o.passive(); }
      _ => { return false; }
    }
  }

  /// Get the value to designate as present value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cv_present(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::CurrentValue(o) => { return o.present(); }
      _ => { return false; }
    }
  }

  /// Get the interest method used.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn ic_method(self: &Self) -> crate::MethodType {

    match self.extension_value() {
      ExtensionValue::InterestChange(o) => { return o.method(); }
      _ => { return crate::MethodType::Actuarial; }
    }
  }

  /// Get the day count basis.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn ic_day_count_basis(self: &Self) -> crate::DayCountType {

    match self.extension_value() {
      ExtensionValue::InterestChange(o) => { return o.day_count_basis(); }
      _ => { return crate::DayCountType::Periodic; }
    }
  }

  /// Get the number of days in the year.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn ic_days_in_year(self: &Self) -> usize {

    match self.extension_value() {
      ExtensionValue::InterestChange(o) => { return o.days_in_year(); }
      _ => { return 0; }
    }
  }

  /// Get the optional effective frequency.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn ic_effective_frequency(self: &Self) -> crate::FrequencyType {

    match self.extension_value() {
      ExtensionValue::InterestChange(o) => { return o.effective_frequency(); }
      _ => { return crate::FrequencyType::OneMonth; }
    }
  }

  /// Get the optional interest amortization frequency.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn ic_interest_frequency(self: &Self) -> crate::FrequencyType {

    match self.extension_value() {
      ExtensionValue::InterestChange(o) => { return o.interest_frequency(); }
      _ => { return crate::FrequencyType::OneMonth; }
    }
  }

  /// Get the round intermediate balance results.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn ic_round_balance(self: &Self) -> crate::RoundType {

    match self.extension_value() {
      ExtensionValue::InterestChange(o) => { return o.round_balance(); }
      _ => { return crate::RoundType::None; }
    }
  }

  /// Get the round decimal digits.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn ic_round_decimal_digits(self: &Self) -> Decimal {

    match self.extension_value() {
      ExtensionValue::InterestChange(o) => { return o.round_decimal_digits(); }
      _ => { return dec!(0.0); }
    }
  }

  /// Get the name of the statistic event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sv_name(self: &Self) -> &str {

    match self.extension_value() {
      ExtensionValue::StatisticValue(o) => { return o.name(); }
      _ => { return ""; }
    }
  }

  /// Get the value of adjust successive dates to end of month.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sv_eom(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::StatisticValue(o) => { return o.eom(); }
      _ => { return false; }
    }
  }

  /// Get the value to final statistic event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sv_is_final(self: &Self) -> bool {

    match self.extension_value() {
      ExtensionValue::StatisticValue(o) => { return o.is_final(); }
      _ => { return false; }
    }
  }

  /// Set the principal change.
  /// 
  /// # Arguments
  ///
  /// * `principal_change_param` - See description.

  pub fn set_principal_change(self: &mut Self, principal_change_param: ElemPrincipalChange) -> () {

    self.extension_value = ExtensionValue::PrincipalChange(principal_change_param);    

    self.set_updated();
  }

  /// Set the principal change type.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.
  ///     

  pub fn set_pc_type(self: &mut Self, param: crate::PrincipalType) -> () {

    match self.extension_value_mut() {
      ExtensionValue::PrincipalChange(o) => { 
        o.set_type(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the value of adjust successive dates to end of month.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_pc_eom(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::PrincipalChange(o) => { 
        o.set_eom(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the value of apply change to principal balance first for simple interest.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_pc_principal_first(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::PrincipalChange(o) => { 
        o.set_principal_first(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the value of include with balance result statistics.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_pc_balance_statistics(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::PrincipalChange(o) => { 
        o.set_balance_statistics(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the auxiliary principal change event.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_pc_auxiliary(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::PrincipalChange(o) => { 
        o.set_auxiliary(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the auxiliary passive principal change event.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_pc_aux_passive(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::PrincipalChange(o) => { 
        o.set_aux_passive(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the current value.
  /// 
  /// # Arguments
  ///
  /// * `current_value_param` - See description.

  pub fn set_current_value(self: &mut Self, current_value_param: ElemCurrentValue) -> () {

    self.extension_value = ExtensionValue::CurrentValue(current_value_param);    

    self.set_updated();
  }

  /// Set the value to adjust successive dates to end of month.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_cv_eom(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::CurrentValue(o) => { 
        o.set_eom(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the value to not affect the remaining cashflow.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_cv_passive(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::CurrentValue(o) => { 
        o.set_passive(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the value to designate as present value.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_cv_present(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::CurrentValue(o) => { 
        o.set_present(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the interest change.
  /// 
  /// # Arguments
  ///
  /// * `interest_change_param` - See description.

  pub fn set_interest_change(self: &mut Self, interest_change_param: ElemInterestChange) -> () {

    self.extension_value = ExtensionValue::InterestChange(interest_change_param);    

    self.set_updated();
  }

  /// Set the interest method used.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.
  
  pub fn set_ic_method(self: &mut Self, param: crate::MethodType) -> () {

    match self.extension_value_mut() {
      ExtensionValue::InterestChange(o) => { 
        o.set_method(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the day count basis.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.
  
  pub fn set_ic_day_count_basis(self: &mut Self, param: crate::DayCountType) -> () {

    match self.extension_value_mut() {
      ExtensionValue::InterestChange(o) => { 
        o.set_day_count_basis(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the number of days in the year.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_ic_days_in_year(self: &mut Self, param: usize) -> () {

    match self.extension_value_mut() {
      ExtensionValue::InterestChange(o) => { 
        o.set_days_in_year(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the optional effective frequency.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_ic_effective_frequency(self: &mut Self, param: crate::FrequencyType) -> () {

    match self.extension_value_mut() {
      ExtensionValue::InterestChange(o) => { 
        o.set_effective_frequency(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the optional interest amortization frequency.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.
  ///     

  pub fn set_ic_interest_frequency(self: &mut Self, param: crate::FrequencyType) -> () {

    match self.extension_value_mut() {
      ExtensionValue::InterestChange(o) => { 
        o.set_interest_frequency(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the round intermediate balance results.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.
  ///     

  pub fn set_ic_round_balance(self: &mut Self, param: crate::RoundType) -> () {

    match self.extension_value_mut() {
      ExtensionValue::InterestChange(o) => { 
        o.set_round_balance(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the round decimal digits.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.
  ///     

  pub fn set_ic_round_decimal_digits(self: &mut Self, param: Decimal) -> () {

    match self.extension_value_mut() {
      ExtensionValue::InterestChange(o) => { 
        o.set_round_decimal_digits(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the statistic value.
  /// 
  /// # Arguments
  ///
  /// * `statistic_value_param` - See description.

  pub fn set_statistic_value(self: &mut Self, statistic_value_param: ElemStatisticValue) -> () {

    self.extension_value = ExtensionValue::StatisticValue(statistic_value_param);    

    self.set_updated();
  }

  /// Set the name of the statistic event.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_sv_name(self: &mut Self, param: &str) -> () {

    match self.extension_value_mut() {
      ExtensionValue::StatisticValue(o) => { 
        o.set_name(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the value of adjust successive dates to end of month.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_sv_eom(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::StatisticValue(o) => { 
        o.set_eom(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Set the value to final statistic event.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_sv_final(self: &mut Self, param: bool) -> () {

    match self.extension_value_mut() {
      ExtensionValue::StatisticValue(o) => { 
        o.set_final(param);    
        self.set_updated();
      }
      _ => { }
    }
  }

  /// Call the updated signal.
  
  fn set_updated(self: &Self) -> () {

    self.core_manager.borrow().notify(
      CoreUtility::format_update(ElemUpdateType::Event, ElemLevelType::Event));
  }
  
}