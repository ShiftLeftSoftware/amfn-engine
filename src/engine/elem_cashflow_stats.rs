//! The cashflow statistics definition.
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

pub struct ElemCashflowStats {

  /// Number of current value events
  current_values: usize,

  /// Number of interest change events
  interest_changes: usize,

  /// Number of principal change events
  principal_changes: usize,

  /// Number of statistic value events
  statistic_values: usize

}

/// The cashflow statistics element implementation.

impl ElemCashflowStats {

  /// Create and return a new cashflow statistics element.
  /// 
  /// # Arguments
  ///
  /// * `current_values` - Number of current value events.
  /// * `interest_changes` - Number of interest change events.
  /// * `principal_changes` - Number of principal change events.
  /// * `statistic_values` - Number of statistic value events.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(current_values: usize, interest_changes: usize, 
      principal_changes: usize, statistic_values: usize) -> ElemCashflowStats {

    return ElemCashflowStats {
      current_values: current_values,
      interest_changes: interest_changes,
      principal_changes: principal_changes,
      statistic_values: statistic_values
    }
  }

  /// Return the number of current value events.

  pub fn current_values(self: &Self) -> usize {
    return self.current_values;
  }

  /// Return the number of interest change events.

  pub fn interest_changes(self: &Self) -> usize {
    return self.interest_changes;
  }

  /// Return the number of principal change events.

  pub fn principal_changes(self: &Self) -> usize {
    return self.principal_changes;
  }

  /// Return the number of statistic value events.

  pub fn statistic_values(self: &Self) -> usize {
    return self.statistic_values;
  }
  
}