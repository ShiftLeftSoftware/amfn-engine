//! The cashflow statistics definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct ElemCashflowStats {
    /// Number of current value events
    current_values: usize,

    /// Number of interest change events
    interest_changes: usize,

    /// Number of principal change events
    principal_changes: usize,

    /// Number of statistic value events
    statistic_values: usize,
}

/// The cashflow statistics element implementation.

impl ElemCashflowStats {
    /// Create and return a new cashflow statistics element.
    ///
    /// # Arguments
    ///
    /// * `current_values_param` - Number of current value events.
    /// * `interest_changes_param` - Number of interest change events.
    /// * `principal_changes_param` - Number of principal change events.
    /// * `statistic_values_param` - Number of statistic value events.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        current_values_param: usize,
        interest_changes_param: usize,
        principal_changes_param: usize,
        statistic_values_param: usize,
    ) -> ElemCashflowStats {
        ElemCashflowStats {
            current_values: current_values_param,
            interest_changes: interest_changes_param,
            principal_changes: principal_changes_param,
            statistic_values: statistic_values_param,
        }
    }

    /// Return the number of current value events.

    pub fn current_values(&self) -> usize {
        self.current_values
    }

    /// Return the number of interest change events.

    pub fn interest_changes(&self) -> usize {
        self.interest_changes
    }

    /// Return the number of principal change events.

    pub fn principal_changes(&self) -> usize {
        self.principal_changes
    }

    /// Return the number of statistic value events.

    pub fn statistic_values(&self) -> usize {
        self.statistic_values
    }
}
