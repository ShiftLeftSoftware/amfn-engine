//! The parameter element definition.
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

use super::ElemSymbol;

pub struct ElemParameter {

    /// Name of the parameter. 
    name: String,
    /// Parameter symbol. 
    symbol: ElemSymbol

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
    
    return ElemParameter {
      name: String::from(name_param),
      symbol: ElemSymbol::new()
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

  pub fn equal(self: &Self, parameter: &ElemParameter) -> bool {
    
    if self.name != parameter.name() { return false; }

    return true;
  }

  /// Get the parameter name.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the parameter type.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn param_type(self: &Self) -> crate::TokenType {

    return self.symbol.sym_type();
  }

  /// Get the parameter integer.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn param_integeri(self: &Self) -> i32 {

    return self.symbol.sym_integeri();
  }

  /// Get the parameter integer.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn param_integer(self: &Self) -> usize {

    return self.symbol.sym_integer();
  }

  /// Get the parameter decimal.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn param_float(self: &Self) -> Decimal {

    return self.symbol.sym_decimal();
  }

  /// Get the parameter string.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn param_string(self: &Self) -> &str {

    return self.symbol.sym_string();
  }

  /// Set the parameter name.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - See description.
    
  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the parameter type.
  /// 
  /// # Arguments
  ///
  /// * `type_param` - See description.
    
  pub fn set_type(self: &mut Self, type_param: crate::TokenType) -> () {

    self.symbol.set_type(type_param);
  }

  /// Set the parameter integer.
  /// 
  /// # Arguments
  ///
  /// * `integer_param` - See description.
    
  pub fn set_integeri(self: &mut Self, integer_param: i32) -> () {

    self.symbol.set_integeri(integer_param);
  }

  /// Set the parameter integer.
  /// 
  /// # Arguments
  ///
  /// * `integer_param` - See description.
    
  pub fn set_integer(self: &mut Self, integer_param: usize) -> () {

    self.symbol.set_integer(integer_param);
  }

  /// Set the parameter decimal.
  /// 
  /// # Arguments
  ///
  /// * `float_param` - See description.
    
  pub fn set_decimal(self: &mut Self, float_param: Decimal) -> () {

    self.symbol.set_decimal(float_param);
  }

  /// Set the parameter string.
  /// 
  /// # Arguments
  ///
  /// * `string_param` - See description.
    
  pub fn set_string(self: &mut Self, string_param: &str) -> () {

    self.symbol.set_string(string_param);
  }

}