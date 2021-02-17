//! Result definition of the last call to balance the cashflow.
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

use rust_decimal::prelude::*;

pub struct ElemBalanceResult {

  /// Total number of TYPE_PRINCIPAL_CHANGEs with statistics set. 
  prin_total: usize,
  /// Number of TYPE_PRINCIPAL_CHANGEs following first current value present with statistics set. 
  prin_present: usize,
  /// Total compounded interest charged not including any remaining interest. 
  interest_total: Decimal,
  /// Compounded interest following first current value present. 
  interest_present: Decimal,   
  /// Total straight-line interest charged not including any remaining interest. 
  sl_interest_total: Decimal,
  /// Straight-line interest following first current value present. 
  sl_interest_present: Decimal,   
  /// Final accrued interest balance. 
  acc_balance: Decimal,
  /// Final balance. 
  balance: Decimal,
  /// Final balance date. 
  balance_date: usize,
  /// Total value of TYPE_PRINCIPAL_CHANGE decreases without auxiliary set. 
  prin_decrease: Decimal,
  /// Total value of TYPE_PRINCIPAL_CHANGE increases without auxiliary set. 
  prin_increase: Decimal,
  /// Value of active TYPE_PRINCIPAL_CHANGE decreases with auxiliary set. 
  aux_active_decrease: Decimal,
  /// Value of active TYPE_PRINCIPAL_CHANGE increases with auxiliary set. 
  aux_active_increase: Decimal,
  /// Value of passive TYPE_PRINCIPAL_CHANGE decreases with auxiliary set. 
  aux_passive_decrease: Decimal,
  /// Value of passive TYPE_PRINCIPAL_CHANGE increases with auxiliary set. 
  aux_passive_increase: Decimal,
  /// -1 = Negitive cashflow, 1 = Positive cashflow. 
  polarity: i32,
  /// Accrued interest balance seen. 
  acc_balance_seen: bool,
  /// Rule of 78 seen. 
  rule_of_78_seen: bool,
  
  /// First principal change. 
  prin_first_index: usize,
  /// First principal change with statistics set. 
  prin_first_stat_index: usize,
  /// First principal change after PV. 
  prin_first_pv_index: usize,
  /// First principal change with statistics set after PV. 
  prin_first_stat_pv_index: usize,
  /// Last principal change. 
  prin_last_index: usize,
  /// Last principal change with statistics set. 
  prin_last_stat_index: usize,
  /// First current value with PV set. 
  cur_first_pv_index: usize,
  /// First interest change. 
  int_first_index: usize,
  /// Last interest change. 
  int_last_index: usize,

  /// Last yield calculated. 
  last_yield: Decimal
  
}

/// Result definition implementation.

impl ElemBalanceResult {

  /// Create a new result definition.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new() -> ElemBalanceResult {

    return ElemBalanceResult {
      prin_total: 0,
      prin_present: 0,
      interest_total: dec!(0.0),
      interest_present: dec!(0.0),
      sl_interest_total: dec!(0.0),
      sl_interest_present: dec!(0.0),
      acc_balance: dec!(0.0),
      balance: dec!(0.0),
      balance_date: 0,
      prin_decrease: dec!(0.0),
      prin_increase: dec!(0.0),
      aux_active_decrease: dec!(0.0),
      aux_active_increase: dec!(0.0),
      aux_passive_decrease: dec!(0.0),
      aux_passive_increase: dec!(0.0),
      polarity: 1,
      acc_balance_seen: false,
      rule_of_78_seen: false,      
      prin_first_index: usize::MAX,
      prin_first_stat_index: usize::MAX,
      prin_first_pv_index: usize::MAX,
      prin_first_stat_pv_index: usize::MAX,
      prin_last_index: usize::MAX,
      prin_last_stat_index: usize::MAX,
      cur_first_pv_index: usize::MAX,
      int_first_index: usize::MAX,
      int_last_index: usize::MAX,
      last_yield: dec!(0.0)
    };
  }

  /// Copy this result definition as a new result definition.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ElemBalanceResult {

    return ElemBalanceResult {
      prin_total: self.prin_total,
      prin_present: self.prin_present,
      interest_total: self.interest_total,
      interest_present: self.interest_present,
      sl_interest_total: self.sl_interest_total,
      sl_interest_present: self.sl_interest_present,
      acc_balance: self.acc_balance,
      balance: self.balance,
      balance_date: self.balance_date,
      prin_decrease: self.prin_decrease,
      prin_increase: self.prin_increase,
      aux_active_decrease: self.aux_active_decrease,
      aux_active_increase: self.aux_active_increase,
      aux_passive_decrease: self.aux_passive_decrease,
      aux_passive_increase: self.aux_passive_increase,
      polarity: self.polarity,
      acc_balance_seen: self.acc_balance_seen,
      rule_of_78_seen: self.rule_of_78_seen,      
      prin_first_index: self.prin_first_index,
      prin_first_stat_index: self.prin_first_stat_index,
      prin_first_pv_index: self.prin_first_pv_index,
      prin_first_stat_pv_index: self.prin_first_stat_pv_index,
      prin_last_index: self.prin_last_index,
      prin_last_stat_index: self.prin_last_stat_index,
      cur_first_pv_index: self.cur_first_pv_index,
      int_first_index: self.int_first_index,
      int_last_index: self.int_last_index,
      last_yield: self.last_yield
    };
  }

  /// Clear all values.

  pub fn clear(self: &mut Self) -> () {

    self.prin_total = 0;
    self.prin_present = 0;
    self.interest_total = dec!(0.0);
    self.interest_present = dec!(0.0);
    self.sl_interest_total = dec!(0.0);
    self.sl_interest_present = dec!(0.0);
    self.acc_balance = dec!(0.0);
    self.balance = dec!(0.0);
    self.balance_date = 0;
    self.prin_decrease = dec!(0.0);
    self.prin_increase = dec!(0.0);
    self.aux_active_decrease = dec!(0.0);
    self.aux_active_increase = dec!(0.0);
    self.aux_passive_decrease = dec!(0.0);
    self.aux_passive_increase = dec!(0.0);
    self.polarity = 1; // Positive CF
    self.acc_balance_seen = false;
    self.rule_of_78_seen = false;    
    self.prin_first_index = usize::MAX;
    self.prin_first_stat_index = usize::MAX;
    self.prin_first_pv_index = usize::MAX;
    self.prin_first_stat_pv_index = usize::MAX;
    self.prin_last_index = usize::MAX;
    self.prin_last_stat_index = usize::MAX;
    self.cur_first_pv_index = usize::MAX;
    self.int_first_index = usize::MAX;
    self.int_last_index = usize::MAX;
  }

  /// Get the total number of TYPE_PRINCIPAL_CHANGEs with statistics set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_total(self: &Self) -> usize {

    return self.prin_total;
  }

  /// Get the number of TYPE_PRINCIPAL_CHANGEs following first current value 
  /// present with statistics set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_present(self: &Self) -> usize {

    return self.prin_present;
  }

  /// Get the total compounded interest charged not including any remaining interest.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn interest_total(self: &Self) -> Decimal {

    return self.interest_total;
  }

  /// Get the compounded interest following first current value present.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn interest_present(self: &Self) -> Decimal {

    return self.interest_present;
  }

  /// Get the total straight-line interest charged not including any remaining interest.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sl_interest_total(self: &Self) -> Decimal {

    return self.sl_interest_total;
  }

  /// Get the straight-line interest following first current value present.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sl_interest_present(self: &Self) -> Decimal {

    return self.sl_interest_present;
  }

  /// Get the final accrued interest balance.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn acc_balance(self: &Self) -> Decimal {

    return self.acc_balance;
  }

  /// Get the final balance.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn balance(self: &Self) -> Decimal {

    return self.balance;
  }

  /// Get the final balance date.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn balance_date(self: &Self) -> usize {

    return self.balance_date;
  }

  /// Get the total value of TYPE_PRINCIPAL_CHANGE decreases without auxiliary set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_decrease(self: &Self) -> Decimal {

    return self.prin_decrease;
  }

  /// Get the total value of TYPE_PRINCIPAL_CHANGE increases without auxiliary set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_increase(self: &Self) -> Decimal {

    return self.prin_increase;
  }

  /// Get the value of active TYPE_PRINCIPAL_CHANGE decreases with auxiliary set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn aux_active_decrease(self: &Self) -> Decimal {

    return self.aux_active_decrease;
  }

  /// Get the value of active TYPE_PRINCIPAL_CHANGE increases with auxiliary set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn aux_active_increase(self: &Self) -> Decimal {

    return self.aux_active_increase;
  }

  /// Get the value of passive TYPE_PRINCIPAL_CHANGE decreases with auxiliary set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn aux_passive_decrease(self: &Self) -> Decimal {

    return self.aux_passive_decrease;
  }

  /// Get the value of passive TYPE_PRINCIPAL_CHANGE increases with auxiliary set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn aux_passive_increase(self: &Self) -> Decimal {

    return self.aux_passive_increase;
  }

  /// Get the polarity of the cashflow.
  /// 
  /// # Return
  ///
  /// * -1=Negitive cashflow, 1=Positive cashflow.

  pub fn polarity(self: &Self) -> i32 {

    return self.polarity;
  }

  /// Get the accrued interest balance seen.
  /// 
  /// # Return
  ///
  /// * True if accrued interest balance seen, otherwise false.

  pub fn acc_balance_seen(self: &Self) -> bool {

    return self.acc_balance_seen;
  }

  /// Get the Rule of 78 seen.
  /// 
  /// # Return
  ///
  /// * True if rule of 78 seen, otherwise false.

  pub fn rule_of_78_seen(self: &Self) -> bool {

    return self.rule_of_78_seen;
  }
  
  /// Get the first principal change.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_first_index(self: &Self) -> usize {

    return self.prin_first_index;
  }

  /// Get the first principal change with statistics set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_first_stat_index(self: &Self) -> usize {

    return self.prin_first_stat_index;
  }

  /// Get the first principal change after PV.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_first_pv_index(self: &Self) -> usize {

    return self.prin_first_pv_index;
  }

  /// Get the first principal change with statistics set after PV.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_first_stat_pv_index(self: &Self) -> usize {

    return self.prin_first_stat_pv_index;
  }

  /// Get the last principal change.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_last_index(self: &Self) -> usize {

    return self.prin_last_index;
  }

  /// Get the last principal change with statistics set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn prin_last_stat_index(self: &Self) -> usize {

    return self.prin_last_stat_index;
  }
  
  /// Get the first current value with PV set.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn cur_first_pv_index(self: &Self) -> usize {

    return self.cur_first_pv_index;
  }
  
  /// Get the first interest change.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn int_first_index(self: &Self) -> usize {

    return self.int_first_index;
  }
  
  /// Get the last interest change.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn int_last_index(self: &Self) -> usize {

    return self.int_last_index;
  }
  
  /// Get the last yield calculated.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn last_yield(self: &Self) -> Decimal {

    return self.last_yield;
  }

  /// Increment the total number of TYPE_PRINCIPAL_CHANGEs with statistics set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_prin_total(self: &mut Self, param: usize) -> () {

    self.prin_total += param;
  }

  /// Increment the number of TYPE_PRINCIPAL_CHANGEs following first current 
  /// value present with statistics set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_prin_present(self: &mut Self, param: usize) -> () {

    self.prin_present += param;
  }

  /// Increment the total compounded interest charged not including any 
  /// remaining interest.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_interest_total(self: &mut Self, param: Decimal) -> () {

    self.interest_total += param;
  }

  /// Increment the compounded interest following first current value 
  /// present not including any remaining interest.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_interest_present(self: &mut Self, param: Decimal) -> () {

    self.interest_present += param;
  }

  /// Increment the total straight-line interest charged not including 
  /// any remaining interest.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_sl_interest_total(self: &mut Self, param: Decimal) -> () {

    self.sl_interest_total += param;
  }

  /// Increment the straight-line interest following first current 
  /// value present not including any remaining interest.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_sl_interest_present(self: &mut Self, param: Decimal) -> () {

    self.sl_interest_present += param;
  }

  /// Increment the total value of TYPE_PRINCIPAL_CHANGE decreases 
  /// without auxiliary set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_prin_decrease(self: &mut Self, param: Decimal) -> () {

    self.prin_decrease += param;
  }

  /// Increment the total value of TYPE_PRINCIPAL_CHANGE increases 
  /// without auxiliary set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_prin_increase(self: &mut Self, param: Decimal) -> () {

    self.prin_increase += param;
  }

  /// Increment the value of active TYPE_PRINCIPAL_CHANGE decreases 
  /// with auxiliary set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_aux_active_decrease(self: &mut Self, param: Decimal) -> () {

    self.aux_active_decrease += param;
  }

  /// Increment the value of active TYPE_PRINCIPAL_CHANGE increases 
  /// with auxiliary set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_aux_active_increase(self: &mut Self, param: Decimal) -> () {

    self.aux_active_increase += param;
  }

  /// Increment the value of passive TYPE_PRINCIPAL_CHANGE decreases 
  /// with auxiliary set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_aux_passive_decrease(self: &mut Self, param: Decimal) -> () {

    self.aux_passive_decrease += param;
  }

  /// Increment the value of passive TYPE_PRINCIPAL_CHANGE increases 
  /// with auxiliary set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn incr_aux_passive_increase(self: &mut Self, param: Decimal) -> () {

    self.aux_passive_increase += param;
  }

  /// Set the final accrued interest balance.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_acc_balance(self: &mut Self, param: Decimal) -> () {

    self.acc_balance = param;
  }

  /// Set the final balance.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_balance(self: &mut Self, param: Decimal) -> () {

    self.balance = param;
  }

  /// Set the final balance date.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_balance_date(self: &mut Self, param: usize) -> () {

    self.balance_date = param;
  }

  /// Set the polarity of the cashflow.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_polarity(self: &mut Self, param: i32) -> () {

    self.polarity = param;
  }

  /// Set the accrued interest balance seen.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_acc_balance_seen(self: &mut Self, param: bool) -> () {

    self.acc_balance_seen = param;
  }

  /// Set the rule of 78 seen.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_rule_of_78_seen(self: &mut Self, param: bool) -> () {

    self.rule_of_78_seen = param;
  }
  
  /// Set the first principal change.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_prin_first_index(self: &mut Self, param: usize) -> () {

    self.prin_first_index = param;
  }

  /// Set the first principal change with statistics set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_prin_first_stat_index(self: &mut Self, param: usize) -> () {

    self.prin_first_stat_index = param;
  }

  /// Set the first principal change after PV.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_prin_first_pv_index(self: &mut Self, param: usize) -> () {

    self.prin_first_pv_index = param;
  }

  /// Set the first principal change with statistics set after PV.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_prin_first_stat_pv_index(self: &mut Self, param: usize) -> () {

    self.prin_first_stat_pv_index = param;
  }

  /// Set the last principal change.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_prin_last_index(self: &mut Self, param: usize) -> () {

    self.prin_last_index = param;
  }

  /// Set the last principal change with statistics set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_prin_last_stat_index(self: &mut Self, param: usize) -> () {

    self.prin_last_stat_index = param;
  }
  
  /// Set the first current value with PV set.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_cur_first_pv_index(self: &mut Self, param: usize) -> () {

    self.cur_first_pv_index = param;
  }
  
  /// Set the first interest change.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_int_first_index(self: &mut Self, param: usize) -> () {

    self.int_first_index = param;
  }
  
  /// Set the last interest change.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_int_last_index(self: &mut Self, param: usize) -> () {

    self.int_last_index = param;
  }
  
  /// Set the last yield calculated.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_last_yield(self: &mut Self, param: Decimal) -> () {

    self.last_yield = param;
  }

}