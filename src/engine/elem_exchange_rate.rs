//! The exchange rate element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;

pub struct ElemExchangeRate {
    /// International currency code "from"
    from_code: String,
    /// International currency code "to"
    to_code: String,
    /// The exchange rate in "from" (unit) / "to" (unit).
    exchange_rate: Decimal,
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

    pub fn new(
        from_code_param: &str,
        to_code_param: &str,
        exchange_rate_param: Decimal,
    ) -> ElemExchangeRate {
        ElemExchangeRate {
            from_code: String::from(from_code_param),
            to_code: String::from(to_code_param),
            exchange_rate: exchange_rate_param,
        }
    }

    /// Get the from code.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn from_code(&self) -> &str {
        self.from_code.as_str()
    }

    /// Get the to code.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn to_code(&self) -> &str {
        self.to_code.as_str()
    }

    /// Get the exchange_rate.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn exchange_rate(&self) -> Decimal {
        self.exchange_rate
    }

    /// Set the from code.
    ///
    /// # Arguments
    ///
    /// * `from_code` - See description.

    pub fn set_from_code(&mut self, from_code: &str) {
        self.from_code = String::from(from_code);
    }

    /// Set the to code.
    ///
    /// # Arguments
    ///
    /// * `to_code` - See description.

    pub fn set_to_code(&mut self, to_code: &str) {
        self.to_code = String::from(to_code);
    }

    /// Set the exchange rate.
    ///
    /// # Arguments
    ///
    /// * `exchange_rate_param` - See description.

    pub fn set_exchange_rate(&mut self, exchange_rate_param: Decimal) {
        self.exchange_rate = exchange_rate_param;
    }
}
