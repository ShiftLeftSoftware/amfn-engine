//! The symbol element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;

#[derive(Clone)]
pub struct ElemSymbol {
    /// Type of symbol.
    sym_type: crate::TokenType,
    /// Integer value.
    int_value: i32,
    /// Decimal value.
    dec_value: Decimal,
    /// String value.
    str_value: String,
}

/// The symbol element default implementation.

impl Default for ElemSymbol {
    /// Create a new symbol element.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ElemSymbol::new()
    }
}

/// The symbol element implementation.

impl ElemSymbol {
    /// Create a new symbol element.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new() -> ElemSymbol {
        ElemSymbol {
            sym_type: crate::TokenType::Integer,
            int_value: 0,
            dec_value: dec!(0.0),
            str_value: String::from(""),
        }
    }

    /// Create a new object with a specific type.
    ///
    /// # Arguments
    ///
    /// * `type_param` - Type of symbol.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new_with_token_type(type_param: crate::TokenType) -> ElemSymbol {
        ElemSymbol {
            sym_type: type_param,
            int_value: 0,
            dec_value: dec!(0.0),
            str_value: String::from(""),
        }
    }

    /// Copy this symbol and return a new symbol.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self) -> ElemSymbol {
        let mut sym = ElemSymbol::new();
        match self.sym_type {
            crate::TokenType::Integer => {
                sym.set_integeri(self.int_value);
            }
            crate::TokenType::Decimal => {
                sym.set_decimal(self.dec_value);
            }
            crate::TokenType::String => {
                sym.set_string(self.str_value.as_str());
            }
            _ => {}
        }

        sym
    }

    /// Get the type of symbol.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn sym_type(&self) -> crate::TokenType {
        self.sym_type
    }

    /// Get the integer value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sym_integeri(&self) -> i32 {
        self.int_value
    }

    /// Get the integer value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sym_integer(&self) -> usize {
        self.sym_integeri() as usize
    }

    /// Get the decimal value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sym_decimal(&self) -> Decimal {
        self.dec_value
    }

    /// Get the string value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn sym_string(&self) -> &str {
        self.str_value.as_str()
    }

    /// Set the type of symbol.
    ///
    /// # Arguments
    ///
    /// * `sym_type_param` - See description.

    pub fn set_type(&mut self, sym_type_param: crate::TokenType) {
        self.sym_type = sym_type_param;
        self.int_value = 0;
        self.dec_value = dec!(0.0);
        self.str_value = String::from("");
    }

    /// Set the operator value.
    ///
    /// # Arguments
    ///
    /// * `op_value_param` - See description.

    pub fn set_operator(&mut self, op_value_param: usize) {
        self.sym_type = crate::TokenType::Operator;
        self.int_value = op_value_param as i32;
        self.dec_value = dec!(0.0);
        self.str_value = String::from("");
    }

    /// Set the integer value.
    ///
    /// # Arguments
    ///
    /// * `int_value_param` - See description.

    pub fn set_integeri(&mut self, int_value_param: i32) {
        self.sym_type = crate::TokenType::Integer;
        self.int_value = int_value_param;
        self.dec_value = dec!(0.0);
        self.str_value = String::from("");
    }

    /// Set the integer value.
    ///
    /// # Arguments
    ///
    /// * `int_value_param` - See description.

    pub fn set_integer(&mut self, int_value_param: usize) {
        self.set_integeri(int_value_param as i32);
    }

    /// Set the decimal value.
    ///
    /// # Arguments
    ///
    /// * `flt_value_param` - See description.

    pub fn set_decimal(&mut self, dec_value_param: Decimal) {
        self.sym_type = crate::TokenType::Decimal;
        self.int_value = 0;
        self.dec_value = dec_value_param;
        self.str_value = String::from("");
    }

    /// Set the string value.
    ///
    /// # Arguments
    ///
    /// * `str_value_param` - See description.

    pub fn set_string(&mut self, str_value_param: &str) {
        self.sym_type = crate::TokenType::String;
        self.int_value = 0;
        self.dec_value = dec!(0.0);
        self.str_value = String::from(str_value_param);
    }
}
