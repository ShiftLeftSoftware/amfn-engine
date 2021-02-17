//! The exchange rate element definition.
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

pub struct ElemExchangeRate {

  /// International currency code "from" 
  from_code: String,
  /// International currency code "to" 
  to_code: String,
  /// The exchange rate in "from" (unit) / "to" (unit). 
  exchange_rate: Decimal
  
}

/// The exchange rate element implementation.

impl ElemExchangeRate {

  /// Create and return a new exchange rate element.
  /// 
  /// # Arguments
  ///
  /// * `from_code_param` - From code.
  /// * `to_code_param` - To code.
  /// * `exchange_rate_param` - Exchange rate.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(from_code_param: &str, to_code_param: &str, exchange_rate_param: Decimal) -> ElemExchangeRate {
    
    return ElemExchangeRate {
      from_code: String::from(from_code_param),
      to_code: String::from(to_code_param),
      exchange_rate: exchange_rate_param
    }
  }

  /// Get the from code.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn from_code(self: &Self) -> &str {

    return self.from_code.as_str();
  }

  /// Get the to code.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn to_code(self: &Self) -> &str {

    return self.to_code.as_str();
  }

  /// Get the exchange_rate.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn exchange_rate(self: &Self) -> Decimal {

    return self.exchange_rate;
  }

  /// Set the from code.
  /// 
  /// # Arguments
  ///
  /// * `from_code` - See description.
    
  pub fn set_from_code(self: &mut Self, from_code: &str) -> () {

    self.from_code = String::from(from_code);
  }

  /// Set the to code.
  /// 
  /// # Arguments
  ///
  /// * `to_code` - See description.
    
  pub fn set_to_code(self: &mut Self, to_code: &str) -> () {

    self.to_code = String::from(to_code);
  }

  /// Set the exchange rate.
  /// 
  /// # Arguments
  ///
  /// * `exchange_rate_param` - See description.
    
  pub fn set_exchange_rate(self: &mut Self, exchange_rate_param: Decimal) -> () {

    self.exchange_rate = exchange_rate_param;
  }

}