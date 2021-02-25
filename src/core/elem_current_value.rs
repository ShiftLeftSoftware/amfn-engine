//! The current value definition of an event.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct ElemCurrentValue {
    /// Adjust successive dates to end of month.
    eom: bool,
    /// Do not affect the remaining cashflow.
    passive: bool,
    /// Designate as present value.
    present: bool,
}

/// The current value implementation.

impl ElemCurrentValue {
    /// Create a new current value element.
    ///
    /// # Arguments
    ///
    /// * `eom_param` - End-of-month.
    /// * `passive_param` - Passive current value.
    /// * `present_param` - Present current value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(eom_param: bool, passive_param: bool, present_param: bool) -> ElemCurrentValue {
        ElemCurrentValue {
            eom: eom_param,
            passive: passive_param,
            present: present_param,
        }
    }

    /// Copy this current value element as a new current value element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ElemCurrentValue {
        ElemCurrentValue::new(self.eom, self.passive, self.present)
    }

    /// Tests if this current value object and another are equal.
    ///
    /// # Arguments
    ///
    /// * `elem_current_value` - Object to compare.
    ///
    /// # Return
    ///
    /// * True if equals, otherwise false.
    pub fn equal(&self, elem_current_value: &ElemCurrentValue) -> bool {
        self.eom == elem_current_value.eom
            && self.passive == elem_current_value.passive
            && self.present == elem_current_value.present
    }

    /// Get the value to adjust successive dates to end of month.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn eom(&self) -> bool {
        self.eom
    }

    /// Get the value to not affect the remaining cashflow.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn passive(&self) -> bool {
        self.passive
    }

    /// Get the value to designate as present value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn present(&self) -> bool {
        self.present
    }

    /// Set the value to adjust successive dates to end of month.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_eom(&mut self, param: bool) {
        self.eom = param;
    }

    /// Set the value to not affect the remaining cashflow.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_passive(&mut self, param: bool) {
        self.passive = param;
    }

    /// Set the value to designate as present value.
    ///
    /// # Arguments
    ///
    /// * `param` - See description.

    pub fn set_present(&mut self, param: bool) {
        self.present = param;
    }
}
