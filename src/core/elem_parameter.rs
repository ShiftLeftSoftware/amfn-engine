//! The parameter element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;

use super::ElemSymbol;

pub struct ElemParameter {
    /// Name of the parameter.
    name: String,
    /// Parameter symbol.
    symbol: ElemSymbol,
}

/// The parameter element implementation.

impl ElemParameter {
    /// Create a new parameter element.
    ///
    /// # Arguments
    ///
    /// * `name_param` - Parameter name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(name_param: &str) -> ElemParameter {
        ElemParameter {
            name: String::from(name_param),
            symbol: ElemSymbol::new(),
        }
    }

    /// Compare this parameter to the parameter parameter.
    ///
    /// # Arguments
    ///
    /// * `parameter` - Parameter to compare.
    ///
    /// # Return
    ///
    /// * True if equal, otherwise false.

    pub fn equal(&self, parameter: &ElemParameter) -> bool {
        if self.name != parameter.name() {
            return false;
        }

        true
    }

    /// Get the parameter name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the parameter type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_type(&self) -> crate::TokenType {
        self.symbol.sym_type()
    }

    /// Get the parameter integer.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_integeri(&self) -> i32 {
        self.symbol.sym_integeri()
    }

    /// Get the parameter integer.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_integer(&self) -> usize {
        self.symbol.sym_integer()
    }

    /// Get the parameter decimal.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_decimal(&self) -> Decimal {
        self.symbol.sym_decimal()
    }

    /// Get the parameter string.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_string(&self) -> &str {
        self.symbol.sym_string()
    }

    /// Set the parameter name.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.

    pub fn set_name(&mut self, name_param: &str) {
        self.name = String::from(name_param);
    }

    /// Set the parameter type.
    ///
    /// # Arguments
    ///
    /// * `type_param` - See description.

    pub fn set_type(&mut self, type_param: crate::TokenType) {
        self.symbol.set_type(type_param);
    }

    /// Set the parameter integer.
    ///
    /// # Arguments
    ///
    /// * `integer_param` - See description.

    pub fn set_integeri(&mut self, integer_param: i32) {
        self.symbol.set_integeri(integer_param);
    }

    /// Set the parameter integer.
    ///
    /// # Arguments
    ///
    /// * `integer_param` - See description.

    pub fn set_integer(&mut self, integer_param: usize) {
        self.symbol.set_integer(integer_param);
    }

    /// Set the parameter decimal.
    ///
    /// # Arguments
    ///
    /// * `float_param` - See description.

    pub fn set_decimal(&mut self, float_param: Decimal) {
        self.symbol.set_decimal(float_param);
    }

    /// Set the parameter string.
    ///
    /// # Arguments
    ///
    /// * `string_param` - See description.

    pub fn set_string(&mut self, string_param: &str) {
        self.symbol.set_string(string_param);
    }
}
