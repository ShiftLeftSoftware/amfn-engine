//! The statistic helper definition.
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

pub struct ElemStatisticHelper {

  /// Name of the statistic event. 
  name: String,
  /// Accumulated principal decrease for statistics period. 
  principal_decrease: Decimal,
  /// Accumulated principal increase for statistics period. 
  principal_increase: Decimal,
  /// Accumulated compounded interest for period. 
  interest: Decimal,
  /// Accumulated straight-line interest for period. 
  sl_interest: Decimal,
  /// Accumulated value to interest for period. 
  value_to_interest: Decimal,
  /// Accumulated value to principal for period. 
  value_to_principal: Decimal,
  /// The last statistic event date. 
  last_date: usize,
  /// Index of the ElemAmortization object. 
  elem_am_index: usize

}

/// The statistic helper implementation.

impl ElemStatisticHelper {

  /// Create a new statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - Statistic helper name.
  /// * `principal_decrease_param` - Principal decrease.
  /// * `principal_increase_param` - Principal increase.
  /// * `interest_param` - Interest parameter.
  /// * `sl_interest_param` - Straight line interest.
  /// * `value_to_interest_param` - Value to interest.
  /// * `value_to_principal_param` - Value to principal.
  /// * `last_date_param` - Last date.
  /// * `elem_am_index_param` - Amortization element index.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(name_param: &str, principal_decrease_param: Decimal, principal_increase_param: Decimal,
    interest_param: Decimal, sl_interest_param: Decimal, value_to_interest_param: Decimal, 
    value_to_principal_param: Decimal, last_date_param: usize, elem_am_index_param: usize) -> ElemStatisticHelper {

    return ElemStatisticHelper {
      name: String::from(name_param),
      principal_decrease: principal_decrease_param,
      principal_increase: principal_increase_param,
      interest: interest_param,
      sl_interest: sl_interest_param,
      value_to_interest: value_to_interest_param,
      value_to_principal: value_to_principal_param,
      last_date: last_date_param,
      elem_am_index: elem_am_index_param
    }
  }

  /// Get the name of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the principal decrease of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn principal_decrease(self: &Self) -> Decimal {

    return self.principal_decrease;
  }

  /// Get the principal increase of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn principal_increase(self: &Self) -> Decimal {

    return self.principal_increase;
  }

  /// Get the interest of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn interest(self: &Self) -> Decimal {

    return self.interest;
  }

  /// Get the straight line interest of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sl_interest(self: &Self) -> Decimal {

    return self.sl_interest;
  }

  /// Get the value to interest of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value_to_interest(self: &Self) -> Decimal {

    return self.value_to_interest;
  }

  /// Get the value to principal of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn value_to_principal(self: &Self) -> Decimal {

    return self.value_to_principal;
  }

  /// Get the last date of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn last_date(self: &Self) -> usize {

    return self.last_date;
  }

  /// Get the element amortization index of the statistic helper.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn elem_am_index(self: &Self) -> usize {

    return self.elem_am_index;
  }

  /// Set the name of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - See description.

  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the principal decrease of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `principal_decrease_param` - See description.

  pub fn set_principal_decrease(self: &mut Self, principal_decrease_param: Decimal) -> () {

    self.principal_decrease = principal_decrease_param;
  }

  /// Set the principal increase of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `principal_increase_param` - See description.

  pub fn set_principal_increase(self: &mut Self, principal_increase_param: Decimal) -> () {

    self.principal_increase = principal_increase_param;
  }

  /// Set the interest of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `interest_param` - See description.

  pub fn set_interest(self: &mut Self, interest_param: Decimal) -> () {

    self.interest = interest_param;
  }

  /// Set the straight line interest of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `sl_interest_param` - See description.

  pub fn set_sl_interest(self: &mut Self, sl_interest_param: Decimal) -> () {

    self.sl_interest = sl_interest_param;
  }

  /// Set the value to interest of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `value_to_interest_param` - See description.

  pub fn set_value_to_interest(self: &mut Self, value_to_interest_param: Decimal) -> () {

    self.value_to_interest = value_to_interest_param;
  }

  /// Set the value to principal of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `value_to_principal_param` - See description.

  pub fn set_value_to_principal(self: &mut Self, value_to_principal_param: Decimal) -> () {

    self.value_to_principal = value_to_principal_param;
  }

  /// Set the last date of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `last_date_param` - See description.

  pub fn set_last_date(self: &mut Self, last_date_param: usize) -> () {

    self.last_date = last_date_param;
  }

  /// Set the element amortization index of the statistic helper.
  /// 
  /// # Arguments
  ///
  /// * `elem_am_index_param` - See description.

  pub fn set_elem_am_index(self: &mut Self, elem_am_index_param: usize) -> () {

    self.elem_am_index = elem_am_index_param;
  }

}
