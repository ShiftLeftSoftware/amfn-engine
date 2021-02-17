//! The symbol element definition.
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

#[derive(Clone)]
pub struct ElemSymbol {

  /// Type of symbol. 
  sym_type: crate::TokenType,
  /// Integer value. 
  int_value: i32,
  /// Decimal value. 
  dec_value: Decimal,
  /// String value. 
  str_value: String

}

/// The symbol element implementation.

impl ElemSymbol {

  /// Create a new symbol element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new() -> ElemSymbol {
    
    return ElemSymbol {
      sym_type: crate::TokenType::Integer,
      int_value: 0,
      dec_value: dec!(0.0),
      str_value: String::from("")
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
      str_value: String::from("")
    }
  }

  /// Copy this symbol and return a new symbol.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ElemSymbol {
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
      _ => {        
      }
    }

    return sym;
  }

  /// Get the type of symbol.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn sym_type(self: &Self) -> crate::TokenType {

    return self.sym_type;
  }

  /// Get the integer value.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn sym_integeri(self: &Self) -> i32 {

    return self.int_value;
  }

  /// Get the integer value.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn sym_integer(self: &Self) -> usize {

    return self.sym_integeri() as usize;
  }

  /// Get the decimal value.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn sym_decimal(self: &Self) -> Decimal {

    return self.dec_value;
  }

  /// Get the string value.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn sym_string(self: &Self) -> &str {

    return self.str_value.as_str();
  }

  /// Set the type of symbol.
  /// 
  /// # Arguments
  ///
  /// * `sym_type_param` - See description.
    
  pub fn set_type(self: &mut Self, sym_type_param: crate::TokenType) -> () {

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
    
  pub fn set_operator(self: &mut Self, op_value_param: usize) -> () {

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
    
  pub fn set_integeri(self: &mut Self, int_value_param: i32) -> () {

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
    
  pub fn set_integer(self: &mut Self, int_value_param: usize) -> () {

    self.set_integeri(int_value_param as i32);
  }

  /// Set the decimal value.
  /// 
  /// # Arguments
  ///
  /// * `flt_value_param` - See description.
    
  pub fn set_decimal(self: &mut Self, dec_value_param: Decimal) -> () {

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
    
  pub fn set_string(self: &mut Self, str_value_param: &str) -> () {

    self.sym_type = crate::TokenType::String;
    self.int_value = 0;
    self.dec_value = dec!(0.0);
    self.str_value = String::from(str_value_param);
  }

}