//! The interest change definition of an event.
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

pub struct ElemInterestChange {

  /// Interest method used. 
  method: crate::MethodType,
  /// Day count basis. 
  day_count_basis: crate::DayCountType,
  /// Number of days in the year. 
  days_in_year: usize,
  /// Optional effective frequency. 
  effective_frequency: crate::FrequencyType,
  /// Optional interest amortization frequency. 
  interest_frequency: crate::FrequencyType,
  /// Round intermediate balance results. 
  round_balance: crate::RoundType,
  /// Less than zero: Use preferences 
  /// Greater than zero and less than one: Round to fraction
  /// Zero or greater than and equal to one: Decimal digits to round. 
  round_decimal_digits: Decimal

}

/// The interest change implementation.

impl ElemInterestChange {

  /// Create a new interest change element.
  /// 
  /// # Arguments
  ///
  /// * `method_param` - Interest method.
  /// * `day_count_basis_param` - Day count basis.
  /// * `days_in_year_param` - Days in year.
  /// * `effective_frequency_param` - Optional effective frequency.
  /// * `interest_frequency_param` - Optional interest amortization frequency.
  /// * `round_balance_param` - Round balance.
  /// * `round_decimal_digits_param` - Round decimal digits.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(method_param: crate::MethodType, day_count_basis_param: crate::DayCountType, 
    days_in_year_param: usize, effective_frequency_param: crate::FrequencyType,
    interest_frequency_param: crate::FrequencyType, round_balance_param: crate::RoundType, 
    round_decimal_digits_param: Decimal) -> ElemInterestChange {

    return ElemInterestChange {
      method: method_param,
      day_count_basis: day_count_basis_param,
      days_in_year: days_in_year_param,
      effective_frequency: effective_frequency_param,
      interest_frequency: interest_frequency_param,
      round_balance: round_balance_param,
      round_decimal_digits: round_decimal_digits_param
    }
  }

  /// Copy this interest change element as a new interest change element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ElemInterestChange {

    return ElemInterestChange::new(
      self.method, self.day_count_basis, self.days_in_year, self.effective_frequency,
      self.interest_frequency, self.round_balance, self.round_decimal_digits);
  }

  /// Tests if this interest change object and another are equal.
  /// 
  /// # Arguments
  ///
  /// * `elem_interest_change` - Object to compare.
  /// # Return
  ///
  /// * True if equals, otherwise false.
  
  pub fn equal(self: &Self, elem_interest_change: &ElemInterestChange) -> bool {
    
    return self.method == elem_interest_change.method &&
      self.day_count_basis == elem_interest_change.day_count_basis &&
      self.days_in_year == elem_interest_change.days_in_year &&
      self.effective_frequency == elem_interest_change.effective_frequency &&
      self.days_in_year == elem_interest_change.days_in_year &&
      self.days_in_year == elem_interest_change.days_in_year &&
      self.days_in_year == elem_interest_change.days_in_year;
  }

  /// Get the interest method used.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn method(self: &Self) -> crate::MethodType {

    return self.method;
  }

  /// Get the day count basis.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn day_count_basis(self: &Self) -> crate::DayCountType {

    return self.day_count_basis;
  }

  /// Get the number of days in the year.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn days_in_year(self: &Self) -> usize {

    return self.days_in_year;
  }

  /// Get the optional effective frequency.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn effective_frequency(self: &Self) -> crate::FrequencyType {

    return self.effective_frequency;
  }

  /// Get the optional interest amortization frequency.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn interest_frequency(self: &Self) -> crate::FrequencyType {

    return self.interest_frequency;
  }

  /// Get the round intermediate balance results.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn round_balance(self: &Self) -> crate::RoundType {

    return self.round_balance;
  }

  /// Get the round decimal digits.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn round_decimal_digits(self: &Self) -> Decimal {

    return self.round_decimal_digits;
  }

  /// Set the interest method used.
  /// 
  /// # Arguments
  ///
  /// * `method_param` - See description.
  ///     

  pub fn set_method(self: &mut Self, method_param: crate::MethodType) -> () {

    self.method = method_param;
  }

  /// Set the day count basis.
  /// 
  /// # Arguments
  ///
  /// * `day_count_basis_param` - See description.
  ///     

  pub fn set_day_count_basis(self: &mut Self, day_count_basis_param: crate::DayCountType) -> () {

    self.day_count_basis = day_count_basis_param;
  }

  /// Set the number of days in the year.
  /// 
  /// # Arguments
  ///
  /// * `days_in_year_param` - See description.

  pub fn set_days_in_year(self: &mut Self, days_in_year_param: usize) -> () {

    self.days_in_year = days_in_year_param;
  }

  /// Set the optional effective frequency.
  /// 
  /// # Arguments
  ///
  /// * `effective_frequency_param` - See description.
  ///     

  pub fn set_effective_frequency(self: &mut Self, effective_frequency_param: crate::FrequencyType) -> () {

    self.effective_frequency = effective_frequency_param;
  }

  /// Set the optional interest amortization frequency.
  /// 
  /// # Arguments
  ///
  /// * `interest_frequency_param` - See description.
  ///     

  pub fn set_interest_frequency(self: &mut Self, interest_frequency_param: crate::FrequencyType) -> () {

    self.interest_frequency = interest_frequency_param;
  }

  /// Set the round intermediate balance results.
  /// 
  /// # Arguments
  ///
  /// * `round_balance_param` - See description.
  ///     

  pub fn set_round_balance(self: &mut Self, round_balance_param: crate::RoundType) -> () {

    self.round_balance = round_balance_param;
  }

  /// Set the round decimal digits.
  /// 
  /// # Arguments
  ///
  /// * `round_decimal_digits_param` - See description.
  ///     

  pub fn set_round_decimal_digits(self: &mut Self, round_decimal_digits_param: Decimal) -> () {

    self.round_decimal_digits = round_decimal_digits_param;
  }

}