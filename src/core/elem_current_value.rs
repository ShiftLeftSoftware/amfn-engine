//! The current value definition of an event.
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

pub struct ElemCurrentValue {

  /// Adjust successive dates to end of month. 
  eom: bool,
  /// Do not affect the remaining cashflow. 
  passive: bool,
  /// Designate as present value. 
  present: bool
  
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

    return ElemCurrentValue {
      eom: eom_param,
      passive: passive_param,
      present: present_param
    }
  }

  /// Copy this current value element as a new current value element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self) -> ElemCurrentValue {

    return ElemCurrentValue::new(
      self.eom, self.passive, self.present);
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
  
  pub fn equal(self: &Self, elem_current_value: &ElemCurrentValue) -> bool {
    
    return self.eom == elem_current_value.eom &&
      self.passive == elem_current_value.passive &&
      self.present == elem_current_value.present;
  }

  /// Get the value to adjust successive dates to end of month.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn eom(self: &Self) -> bool {

    return self.eom;
  }

  /// Get the value to not affect the remaining cashflow.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn passive(self: &Self) -> bool {

    return self.passive;
  }

  /// Get the value to designate as present value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn present(self: &Self) -> bool {

    return self.present;
  }

  /// Set the value to adjust successive dates to end of month.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_eom(self: &mut Self, param: bool) -> () {

    self.eom = param;    
  }

  /// Set the value to not affect the remaining cashflow.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_passive(self: &mut Self, param: bool) -> () {

    self.passive = param;
  }

  /// Set the value to designate as present value.
  /// 
  /// # Arguments
  ///
  /// * `param` - See description.

  pub fn set_present(self: &mut Self, param: bool) -> () {

    self.present = param;
  }
  
}