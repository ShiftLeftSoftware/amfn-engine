//! The statistic value definition of an event.
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

pub struct ElemStatisticValue {

  /// Name of the statistic event. 
  name: String,
  /// Adjust successive dates to end of month. 
  eom: bool,
  /// Final statistic event. 
  is_final: bool

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

    return ElemStatisticValue {
      name: String::from(name_param),
      eom: eom_param,
      is_final: final_param
    }
  }

  /// Copy this statistic value element as a new statistic value element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ElemStatisticValue {

    return ElemStatisticValue::new(
      self.name.as_str(), self.eom, self.is_final);
  }

  /// Tests if this statistic value object and another are equal.
  /// 
  /// # Arguments
  ///
  /// * `elem_statistic_value` - Object to compare.
  /// # Return
  ///
  /// * True if equals, otherwise false.
  
  pub fn equal(self: &Self, elem_statistic_value: &ElemStatisticValue) -> bool {
    
    return self.name == elem_statistic_value.name &&
      self.eom == elem_statistic_value.eom &&
      self.is_final == elem_statistic_value.is_final;
  }

  /// Get the name of the statistic event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the value of adjust successive dates to end of month.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn eom(self: &Self) -> bool {

    return self.eom;
  }

  /// Get the value to final statistic event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn is_final(self: &Self) -> bool {

    return self.is_final;
  }

  /// Set the name of the statistic event.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - See description.

  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the value of adjust successive dates to end of month.
  /// 
  /// # Arguments
  ///
  /// * `eom_param` - See description.

  pub fn set_eom(self: &mut Self, eom_param: bool) -> () {

    self.eom = eom_param;
  }

  /// Set the value to final statistic event.
  /// 
  /// # Arguments
  ///
  /// * `final_param` - See description.

  pub fn set_final(self: &mut Self, final_param: bool) -> () {

    self.is_final = final_param;
  }

}