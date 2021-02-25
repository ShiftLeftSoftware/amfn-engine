//! The statistic helper definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
    elem_am_index: usize,
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
    #[allow(clippy::too_many_arguments)]

    pub fn new(
        name_param: &str,
        principal_decrease_param: Decimal,
        principal_increase_param: Decimal,
        interest_param: Decimal,
        sl_interest_param: Decimal,
        value_to_interest_param: Decimal,
        value_to_principal_param: Decimal,
        last_date_param: usize,
        elem_am_index_param: usize,
    ) -> ElemStatisticHelper {
        ElemStatisticHelper {
            name: String::from(name_param),
            principal_decrease: principal_decrease_param,
            principal_increase: principal_increase_param,
            interest: interest_param,
            sl_interest: sl_interest_param,
            value_to_interest: value_to_interest_param,
            value_to_principal: value_to_principal_param,
            last_date: last_date_param,
            elem_am_index: elem_am_index_param,
        }
    }

    /// Get the name of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the principal decrease of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn principal_decrease(&self) -> Decimal {
        self.principal_decrease
    }

    /// Get the principal increase of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn principal_increase(&self) -> Decimal {
        self.principal_increase
    }

    /// Get the interest of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn interest(&self) -> Decimal {
        self.interest
    }

    /// Get the straight line interest of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sl_interest(&self) -> Decimal {
        self.sl_interest
    }

    /// Get the value to interest of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_to_interest(&self) -> Decimal {
        self.value_to_interest
    }

    /// Get the value to principal of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_to_principal(&self) -> Decimal {
        self.value_to_principal
    }

    /// Get the last date of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn last_date(&self) -> usize {
        self.last_date
    }

    /// Get the element amortization index of the statistic helper.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn elem_am_index(&self) -> usize {
        self.elem_am_index
    }

    /// Set the name of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.

    pub fn set_name(&mut self, name_param: &str) {
        self.name = String::from(name_param);
    }

    /// Set the principal decrease of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `principal_decrease_param` - See description.

    pub fn set_principal_decrease(&mut self, principal_decrease_param: Decimal) {
        self.principal_decrease = principal_decrease_param;
    }

    /// Set the principal increase of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `principal_increase_param` - See description.

    pub fn set_principal_increase(&mut self, principal_increase_param: Decimal) {
        self.principal_increase = principal_increase_param;
    }

    /// Set the interest of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `interest_param` - See description.

    pub fn set_interest(&mut self, interest_param: Decimal) {
        self.interest = interest_param;
    }

    /// Set the straight line interest of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `sl_interest_param` - See description.

    pub fn set_sl_interest(&mut self, sl_interest_param: Decimal) {
        self.sl_interest = sl_interest_param;
    }

    /// Set the value to interest of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `value_to_interest_param` - See description.

    pub fn set_value_to_interest(&mut self, value_to_interest_param: Decimal) {
        self.value_to_interest = value_to_interest_param;
    }

    /// Set the value to principal of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `value_to_principal_param` - See description.

    pub fn set_value_to_principal(&mut self, value_to_principal_param: Decimal) {
        self.value_to_principal = value_to_principal_param;
    }

    /// Set the last date of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `last_date_param` - See description.

    pub fn set_last_date(&mut self, last_date_param: usize) {
        self.last_date = last_date_param;
    }

    /// Set the element amortization index of the statistic helper.
    ///
    /// # Arguments
    ///
    /// * `elem_am_index_param` - See description.

    pub fn set_elem_am_index(&mut self, elem_am_index_param: usize) {
        self.elem_am_index = elem_am_index_param;
    }
}
