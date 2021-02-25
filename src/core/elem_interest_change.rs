//! The interest change definition of an event.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    round_decimal_digits: Decimal,
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

    pub fn new(
        method_param: crate::MethodType,
        day_count_basis_param: crate::DayCountType,
        days_in_year_param: usize,
        effective_frequency_param: crate::FrequencyType,
        interest_frequency_param: crate::FrequencyType,
        round_balance_param: crate::RoundType,
        round_decimal_digits_param: Decimal,
    ) -> ElemInterestChange {
        ElemInterestChange {
            method: method_param,
            day_count_basis: day_count_basis_param,
            days_in_year: days_in_year_param,
            effective_frequency: effective_frequency_param,
            interest_frequency: interest_frequency_param,
            round_balance: round_balance_param,
            round_decimal_digits: round_decimal_digits_param,
        }
    }

    /// Copy this interest change element as a new interest change element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ElemInterestChange {
        ElemInterestChange::new(
            self.method,
            self.day_count_basis,
            self.days_in_year,
            self.effective_frequency,
            self.interest_frequency,
            self.round_balance,
            self.round_decimal_digits,
        )
    }

    /// Tests if this interest change object and another are equal.
    ///
    /// # Arguments
    ///
    /// * `elem_interest_change` - Object to compare.
    /// # Return
    ///
    /// * True if equals, otherwise false.
    pub fn equal(&self, elem_interest_change: &ElemInterestChange) -> bool {
        self.method == elem_interest_change.method
            && self.day_count_basis == elem_interest_change.day_count_basis
            && self.days_in_year == elem_interest_change.days_in_year
            && self.effective_frequency == elem_interest_change.effective_frequency
            && self.interest_frequency == elem_interest_change.interest_frequency
            && self.round_balance == elem_interest_change.round_balance
            && self.round_decimal_digits == elem_interest_change.round_decimal_digits
    }

    /// Get the interest method used.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn method(&self) -> crate::MethodType {
        self.method
    }

    /// Get the day count basis.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn day_count_basis(&self) -> crate::DayCountType {
        self.day_count_basis
    }

    /// Get the number of days in the year.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn days_in_year(&self) -> usize {
        self.days_in_year
    }

    /// Get the optional effective frequency.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn effective_frequency(&self) -> crate::FrequencyType {
        self.effective_frequency
    }

    /// Get the optional interest amortization frequency.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn interest_frequency(&self) -> crate::FrequencyType {
        self.interest_frequency
    }

    /// Get the round intermediate balance results.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn round_balance(&self) -> crate::RoundType {
        self.round_balance
    }

    /// Get the round decimal digits.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn round_decimal_digits(&self) -> Decimal {
        self.round_decimal_digits
    }

    /// Set the interest method used.
    ///
    /// # Arguments
    ///
    /// * `method_param` - See description.
    ///     

    pub fn set_method(&mut self, method_param: crate::MethodType) {
        self.method = method_param;
    }

    /// Set the day count basis.
    ///
    /// # Arguments
    ///
    /// * `day_count_basis_param` - See description.
    ///     

    pub fn set_day_count_basis(&mut self, day_count_basis_param: crate::DayCountType) {
        self.day_count_basis = day_count_basis_param;
    }

    /// Set the number of days in the year.
    ///
    /// # Arguments
    ///
    /// * `days_in_year_param` - See description.

    pub fn set_days_in_year(&mut self, days_in_year_param: usize) {
        self.days_in_year = days_in_year_param;
    }

    /// Set the optional effective frequency.
    ///
    /// # Arguments
    ///
    /// * `effective_frequency_param` - See description.
    ///     

    pub fn set_effective_frequency(&mut self, effective_frequency_param: crate::FrequencyType) {
        self.effective_frequency = effective_frequency_param;
    }

    /// Set the optional interest amortization frequency.
    ///
    /// # Arguments
    ///
    /// * `interest_frequency_param` - See description.
    ///     

    pub fn set_interest_frequency(&mut self, interest_frequency_param: crate::FrequencyType) {
        self.interest_frequency = interest_frequency_param;
    }

    /// Set the round intermediate balance results.
    ///
    /// # Arguments
    ///
    /// * `round_balance_param` - See description.
    ///     

    pub fn set_round_balance(&mut self, round_balance_param: crate::RoundType) {
        self.round_balance = round_balance_param;
    }

    /// Set the round decimal digits.
    ///
    /// # Arguments
    ///
    /// * `round_decimal_digits_param` - See description.
    ///     

    pub fn set_round_decimal_digits(&mut self, round_decimal_digits_param: Decimal) {
        self.round_decimal_digits = round_decimal_digits_param;
    }
}
