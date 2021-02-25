//! The statistic value definition of an event.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct ElemStatisticValue {
    /// Name of the statistic event.
    name: String,
    /// Adjust successive dates to end of month.
    eom: bool,
    /// Final statistic event.
    is_final: bool,
}

/// The statistic value implementation.

impl ElemStatisticValue {
    /// Create a new statistic value element.
    ///
    /// # Arguments
    ///
    /// * `name_param` - Statistic event name.
    /// * `eom_param` - End-of-month.
    /// * `present_param` - Present statistic.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(name_param: &str, eom_param: bool, final_param: bool) -> ElemStatisticValue {
        ElemStatisticValue {
            name: String::from(name_param),
            eom: eom_param,
            is_final: final_param,
        }
    }

    /// Copy this statistic value element as a new statistic value element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ElemStatisticValue {
        ElemStatisticValue::new(self.name.as_str(), self.eom, self.is_final)
    }

    /// Tests if this statistic value object and another are equal.
    ///
    /// # Arguments
    ///
    /// * `elem_statistic_value` - Object to compare.
    /// # Return
    ///
    /// * True if equals, otherwise false.

    pub fn equal(&self, elem_statistic_value: &ElemStatisticValue) -> bool {
        self.name == elem_statistic_value.name
            && self.eom == elem_statistic_value.eom
            && self.is_final == elem_statistic_value.is_final
    }

    /// Get the name of the statistic event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the value of adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn eom(&self) -> bool {
        self.eom
    }

    /// Get the value to final statistic event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn is_final(&self) -> bool {
        self.is_final
    }

    /// Set the name of the statistic event.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.

    pub fn set_name(&mut self, name_param: &str) {
        self.name = String::from(name_param);
    }

    /// Set the value of adjust successive dates to end of month.
    ///
    /// # Arguments
    ///
    /// * `eom_param` - See description.

    pub fn set_eom(&mut self, eom_param: bool) {
        self.eom = eom_param;
    }

    /// Set the value to final statistic event.
    ///
    /// # Arguments
    ///
    /// * `final_param` - See description.

    pub fn set_final(&mut self, final_param: bool) {
        self.is_final = final_param;
    }
}
