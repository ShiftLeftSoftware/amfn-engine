//! The extension element and enumeration value of an event.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;

use super::{ElemCurrentValue, ElemInterestChange, ElemPrincipalChange, ElemStatisticValue};

pub enum ExtensionValue {
    PrincipalChange(ElemPrincipalChange),
    CurrentValue(ElemCurrentValue),
    InterestChange(ElemInterestChange),
    StatisticValue(ElemStatisticValue),
}

/// The extension element of an event.

pub struct ElemExtension {
    /// Extension value.
    extension_value: ExtensionValue,
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

    pub fn new_current_value(current_value_param: ElemCurrentValue) -> ElemExtension {
        ElemExtension::new(ExtensionValue::CurrentValue(current_value_param))
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

    pub fn new_interest_change(interest_change_param: ElemInterestChange) -> ElemExtension {
        ElemExtension::new(ExtensionValue::InterestChange(interest_change_param))
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

    pub fn new_principal_change(principal_change_param: ElemPrincipalChange) -> ElemExtension {
        ElemExtension::new(ExtensionValue::PrincipalChange(principal_change_param))
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

    pub fn new_statistic_value(statistic_value_param: ElemStatisticValue) -> ElemExtension {
        ElemExtension::new(ExtensionValue::StatisticValue(statistic_value_param))
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

    fn new(extension_value_param: ExtensionValue) -> ElemExtension {
        ElemExtension {
            extension_value: extension_value_param,
        }
    }

    /// Copy this extension element as a new extension element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ElemExtension {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => {
                ElemExtension::new(ExtensionValue::PrincipalChange(o.copy()))
            }
            ExtensionValue::CurrentValue(o) => {
                ElemExtension::new(ExtensionValue::CurrentValue(o.copy()))
            }
            ExtensionValue::InterestChange(o) => {
                ElemExtension::new(ExtensionValue::InterestChange(o.copy()))
            }
            ExtensionValue::StatisticValue(o) => {
                ElemExtension::new(ExtensionValue::StatisticValue(o.copy()))
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

    pub fn equal(&self, elem: &ElemExtension) -> bool {
        if self.extension_type() != elem.extension_type() {
            return false;
        }

        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => match elem.extension_value() {
                ExtensionValue::PrincipalChange(o2) => o.equal(o2),
                ExtensionValue::CurrentValue(_o2) => false,
                ExtensionValue::InterestChange(_o2) => false,
                ExtensionValue::StatisticValue(_o2) => false,
            },
            ExtensionValue::CurrentValue(o) => match elem.extension_value() {
                ExtensionValue::PrincipalChange(_o2) => false,
                ExtensionValue::CurrentValue(o2) => o.equal(o2),
                ExtensionValue::InterestChange(_o2) => false,
                ExtensionValue::StatisticValue(_o2) => false,
            },
            ExtensionValue::InterestChange(o) => match elem.extension_value() {
                ExtensionValue::PrincipalChange(_o2) => false,
                ExtensionValue::CurrentValue(_o2) => false,
                ExtensionValue::InterestChange(o2) => o.equal(o2),
                ExtensionValue::StatisticValue(_o2) => false,
            },
            ExtensionValue::StatisticValue(o) => match elem.extension_value() {
                ExtensionValue::PrincipalChange(_o2) => false,
                ExtensionValue::CurrentValue(_o2) => false,
                ExtensionValue::InterestChange(_o2) => false,
                ExtensionValue::StatisticValue(o2) => o.equal(o2),
            },
        }
    }

    /// Get the extension type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn extension_type(&self) -> crate::ExtensionType {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(_o) => crate::ExtensionType::PrincipalChange,
            ExtensionValue::CurrentValue(_o) => crate::ExtensionType::CurrentValue,
            ExtensionValue::InterestChange(_o) => crate::ExtensionType::InterestChange,
            ExtensionValue::StatisticValue(_o) => crate::ExtensionType::StatisticValue,
        }
    }

    /// Get the principal change.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn extension_value(&self) -> &ExtensionValue {
        &self.extension_value
    }

    /// Get the mut principal change.
    ///
    /// # Return
    ///
    /// * See description.

    fn extension_value_mut(&mut self) -> &mut ExtensionValue {
        &mut self.extension_value
    }

    /// Get the end-of-month.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn extension_eom(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => o.eom(),
            ExtensionValue::CurrentValue(o) => o.eom(),
            ExtensionValue::InterestChange(_o) => false,
            ExtensionValue::StatisticValue(o) => o.eom(),
        }
    }

    /// Get the principal change type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn pc_type(&self) -> crate::PrincipalType {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => o.pc_type(),
            _ => crate::PrincipalType::Increase,
        }
    }

    /// Get the value of adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn pc_eom(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => o.eom(),
            _ => false,
        }
    }

    /// Get the value of apply change to principal balance first for simple interest.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn pc_principal_first(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => o.principal_first(),
            _ => false,
        }
    }

    /// Get the value of include with balance result statistics.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn pc_balance_statistics(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => o.balance_statistics(),
            _ => false,
        }
    }

    /// Get the auxiliary principal change event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn pc_auxiliary(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => o.auxiliary(),
            _ => false,
        }
    }

    /// Get the auxiliary passive principal change event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn pc_aux_passive(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::PrincipalChange(o) => o.aux_passive(),
            _ => false,
        }
    }

    /// Get the value to adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cv_eom(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::CurrentValue(o) => o.eom(),
            _ => false,
        }
    }

    /// Get the value to not affect the remaining cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cv_passive(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::CurrentValue(o) => o.passive(),
            _ => false,
        }
    }

    /// Get the value to designate as present value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn cv_present(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::CurrentValue(o) => o.present(),
            _ => false,
        }
    }

    /// Get the interest method used.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn ic_method(&self) -> crate::MethodType {
        match self.extension_value() {
            ExtensionValue::InterestChange(o) => o.method(),
            _ => crate::MethodType::Actuarial,
        }
    }

    /// Get the day count basis.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn ic_day_count_basis(&self) -> crate::DayCountType {
        match self.extension_value() {
            ExtensionValue::InterestChange(o) => o.day_count_basis(),
            _ => crate::DayCountType::Periodic,
        }
    }

    /// Get the number of days in the year.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn ic_days_in_year(&self) -> usize {
        match self.extension_value() {
            ExtensionValue::InterestChange(o) => o.days_in_year(),
            _ => 0,
        }
    }

    /// Get the optional effective frequency.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn ic_effective_frequency(&self) -> crate::FrequencyType {
        match self.extension_value() {
            ExtensionValue::InterestChange(o) => o.effective_frequency(),
            _ => crate::FrequencyType::OneMonth,
        }
    }

    /// Get the optional interest amortization frequency.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn ic_interest_frequency(&self) -> crate::FrequencyType {
        match self.extension_value() {
            ExtensionValue::InterestChange(o) => o.interest_frequency(),
            _ => crate::FrequencyType::OneMonth,
        }
    }

    /// Get the round intermediate balance results.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn ic_round_balance(&self) -> crate::RoundType {
        match self.extension_value() {
            ExtensionValue::InterestChange(o) => o.round_balance(),
            _ => crate::RoundType::None,
        }
    }

    /// Get the round decimal digits.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn ic_round_decimal_digits(&self) -> Decimal {
        match self.extension_value() {
            ExtensionValue::InterestChange(o) => o.round_decimal_digits(),
            _ => {
                dec!(0.0)
            }
        }
    }

    /// Get the name of the statistic event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sv_name(&self) -> &str {
        match self.extension_value() {
            ExtensionValue::StatisticValue(o) => o.name(),
            _ => "",
        }
    }

    /// Get the value of adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sv_eom(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::StatisticValue(o) => o.eom(),
            _ => false,
        }
    }

    /// Get the value to final statistic event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sv_is_final(&self) -> bool {
        match self.extension_value() {
            ExtensionValue::StatisticValue(o) => o.is_final(),
            _ => false,
        }
    }

    /// Set the principal change.
    ///
    /// # Arguments
    ///
    /// * `principal_change_param` - See description.

    pub fn set_principal_change(&mut self, principal_change_param: ElemPrincipalChange) {
        self.extension_value = ExtensionValue::PrincipalChange(principal_change_param);
    }

    /// Set the principal change type.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.
    ///     

    pub fn set_pc_type(&mut self, param: crate::PrincipalType) {
        if let ExtensionValue::PrincipalChange(o) = self.extension_value_mut() {
            o.set_type(param);
        }
    }

    /// Set the value of adjust successive dates to end of month.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_pc_eom(&mut self, param: bool) {
        if let ExtensionValue::PrincipalChange(o) = self.extension_value_mut() {
            o.set_eom(param);
        }
    }

    /// Set the value of apply change to principal balance first for simple interest.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_pc_principal_first(&mut self, param: bool) {
        if let ExtensionValue::PrincipalChange(o) = self.extension_value_mut() {
            o.set_principal_first(param);
        }
    }

    /// Set the value of include with balance result statistics.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_pc_balance_statistics(&mut self, param: bool) {
        if let ExtensionValue::PrincipalChange(o) = self.extension_value_mut() {
            o.set_balance_statistics(param);
        }
    }

    /// Set the auxiliary principal change event.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_pc_auxiliary(&mut self, param: bool) {
        if let ExtensionValue::PrincipalChange(o) = self.extension_value_mut() {
            o.set_auxiliary(param);
        }
    }

    /// Set the auxiliary passive principal change event.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_pc_aux_passive(&mut self, param: bool) {
        if let ExtensionValue::PrincipalChange(o) = self.extension_value_mut() {
            o.set_aux_passive(param);
        }
    }

    /// Set the current value.
    ///
    /// # Arguments
    ///
    /// * `current_value_param` - See description.

    pub fn set_current_value(&mut self, current_value_param: ElemCurrentValue) {
        self.extension_value = ExtensionValue::CurrentValue(current_value_param);
    }

    /// Set the value to adjust successive dates to end of month.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_cv_eom(&mut self, param: bool) {
        if let ExtensionValue::CurrentValue(o) = self.extension_value_mut() {
            o.set_eom(param);
        }
    }

    /// Set the value to not affect the remaining cashflow.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_cv_passive(&mut self, param: bool) {
        if let ExtensionValue::CurrentValue(o) = self.extension_value_mut() {
            o.set_passive(param);
        }
    }

    /// Set the value to designate as present value.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_cv_present(&mut self, param: bool) {
        if let ExtensionValue::CurrentValue(o) = self.extension_value_mut() {
            o.set_present(param);
        }
    }

    /// Set the interest change.
    ///
    /// # Arguments
    ///
    /// * `interest_change_param` - See description.

    pub fn set_interest_change(&mut self, interest_change_param: ElemInterestChange) {
        self.extension_value = ExtensionValue::InterestChange(interest_change_param);
    }

    /// Set the interest method used.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_ic_method(&mut self, param: crate::MethodType) {
        if let ExtensionValue::InterestChange(o) = self.extension_value_mut() {
            o.set_method(param);
        }
    }

    /// Set the day count basis.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_ic_day_count_basis(&mut self, param: crate::DayCountType) {
        if let ExtensionValue::InterestChange(o) = self.extension_value_mut() {
            o.set_day_count_basis(param);
        }
    }

    /// Set the number of days in the year.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_ic_days_in_year(&mut self, param: usize) {
        if let ExtensionValue::InterestChange(o) = self.extension_value_mut() {
            o.set_days_in_year(param);
        }
    }

    /// Set the optional effective frequency.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_ic_effective_frequency(&mut self, param: crate::FrequencyType) {
        if let ExtensionValue::InterestChange(o) = self.extension_value_mut() {
            o.set_effective_frequency(param);
        }
    }

    /// Set the optional interest amortization frequency.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.
    ///     

    pub fn set_ic_interest_frequency(&mut self, param: crate::FrequencyType) {
        if let ExtensionValue::InterestChange(o) = self.extension_value_mut() {
            o.set_interest_frequency(param);
        }
    }

    /// Set the round intermediate balance results.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.
    ///     

    pub fn set_ic_round_balance(&mut self, param: crate::RoundType) {
        if let ExtensionValue::InterestChange(o) = self.extension_value_mut() {
            o.set_round_balance(param);
        }
    }

    /// Set the round decimal digits.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.
    ///     

    pub fn set_ic_round_decimal_digits(&mut self, param: Decimal) {
        if let ExtensionValue::InterestChange(o) = self.extension_value_mut() {
            o.set_round_decimal_digits(param);
        }
    }

    /// Set the statistic value.
    ///
    /// # Arguments
    ///
    /// * `statistic_value_param` - See description.

    pub fn set_statistic_value(&mut self, statistic_value_param: ElemStatisticValue) {
        self.extension_value = ExtensionValue::StatisticValue(statistic_value_param);
    }

    /// Set the name of the statistic event.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_sv_name(&mut self, param: &str) {
        if let ExtensionValue::StatisticValue(o) = self.extension_value_mut() {
            o.set_name(param);
        }
    }

    /// Set the value of adjust successive dates to end of month.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_sv_eom(&mut self, param: bool) {
        if let ExtensionValue::StatisticValue(o) = self.extension_value_mut() {
            o.set_eom(param);
        }
    }

    /// Set the value to final statistic event.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_sv_final(&mut self, param: bool) {
        if let ExtensionValue::StatisticValue(o) = self.extension_value_mut() {
            o.set_final(param);
        }
    }
}
