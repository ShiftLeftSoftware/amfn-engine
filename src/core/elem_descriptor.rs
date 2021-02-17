//! The descriptor element definition.
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

use std::cell::{Cell, RefCell};

pub struct ElemDescriptor {

  /// Group name of the descriptor. 
  group: String, 
  /// Name of the descriptor. 
  name: String, 
  /// Type of descriptor (locale | custom). 
  desc_type: String, 
  /// Code for the type of descriptor (ISO language code_ISO country code). 
  code: String,
  /// Constant value or the result of an expression. 
  value: RefCell<String>,
  /// Optional value expression. 
  value_expr: String,
  /// Propagate to the next level if applicable. 
  propagate: bool,
  /// Index of the event within the event list (applied by amortization). 
  list_event_index: Cell<usize>
  
}

/// The descriptor element implementation.

impl ElemDescriptor {

  /// Create a new descriptor element.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - Descriptor group.
  /// * `name_param` - Descriptor name.
  /// * `desc_type_param` - Descriptor type.
  /// * `code_param` - Descriptor code.
  /// * `value_param` - Value parameter.
  /// * `value_expr_param` - Value expression.
  /// * `propagate_param` - Propogate descriptor.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(group_param: &str, name_param: &str, 
    desc_type_param: &str, code_param: &str, value_param: &str,
    value_expr_param: &str, propagate_param: bool) -> ElemDescriptor {
    
    return ElemDescriptor {
      group: String::from(group_param),
      name: String::from(name_param),
      desc_type: String::from(desc_type_param),
      code: String::from(code_param),
      value: RefCell::new(String::from(value_param)),
      value_expr: String::from(value_expr_param),
      propagate: propagate_param,
      list_event_index: Cell::new(usize::MAX)
    }
  }

  /// Compare this descriptor to the descriptor parameter.
  /// 
  /// # Arguments
  ///
  /// * `descriptor` - Descriptor to compare.
  /// 
  /// # Return
  ///
  /// * True if equal, otherwise false.

  pub fn equal(self: &Self, descriptor: &ElemDescriptor) -> bool {
    
    if self.group != descriptor.group() { return false; }
    if self.name != descriptor.name() { return false; }

    return true;
  }

  /// Get the descriptor group.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn group(self: &Self) -> &str {

    return self.group.as_str();
  }

  /// Get the descriptor name.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the type of descriptor.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn desc_type(self: &Self) -> &str {

    return self.desc_type.as_str();
  }

  /// Get the descriptor code.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn code(self: &Self) -> &str {

    return self.code.as_str();
  }

  /// Get the descriptor value.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn value(self: &Self) -> String {

    return String::from(self.value.borrow().as_str());
  }

  /// Get the descriptor value expression.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn value_expr(self: &Self) -> String {

    return String::from(self.value_expr.as_str());
  }

  /// Get the descriptor propagate.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn propagate(self: &Self) -> bool {

    return self.propagate;
  }

  /// Get the descriptor list event index.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn list_event_index(self: &Self) -> usize {

    return self.list_event_index.get();
  }

  /// Set the descriptor group.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - See description.
    
  pub fn set_group(self: &mut Self, group_param: &str) -> () {

    self.group = String::from(group_param);
  }

  /// Set the descriptor name.
  /// 
  /// # Arguments
  ///
  /// * `name_name` - See description.
    
  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the descriptor type.
  /// 
  /// # Arguments
  ///
  /// * `desc_type_param` - See description.
    
  pub fn set_desc_type(self: &mut Self, desc_type_param: &str) -> () {

    self.desc_type = String::from(desc_type_param);
  }

  /// Set the descriptor code.
  /// 
  /// # Arguments
  ///
  /// * `code_param` - See description.
    
  pub fn set_code(self: &mut Self, code_param: &str) -> () {

    self.code = String::from(code_param);
  }

  /// Set the descriptor value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.
    
  pub fn set_value(self: &Self, value_param: &str) -> () {

    self.value.borrow_mut().clear();
    self.value.borrow_mut().push_str(value_param);
  }

  /// Set the descriptor value expression.
  /// 
  /// # Arguments
  ///
  /// * `value_expr_param` - See description.
    
  pub fn set_value_expr(self: &mut Self, value_expr_param: &str) -> () {

    self.value_expr = String::from(value_expr_param);
  }

  /// Set the descriptor propagate.
  /// 
  /// # Arguments
  ///
  /// * `propagate_param` - See description.
    
  pub fn set_propagate(self: &mut Self, propagate_param: bool) -> () {

    self.propagate = propagate_param;
  }

  /// Set the descriptor list event index.
  /// 
  /// # Arguments
  ///
  /// * `list_event_index_param` - See description.
    
  pub fn set_list_event_index(self: &Self, list_event_index_param: usize) -> () {

    self.list_event_index.set(list_event_index_param);
  }

}