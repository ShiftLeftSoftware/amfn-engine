//! List of parameters.
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

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use rust_decimal::prelude::*;

use crate::{ListTrait, ElemUpdateType, ElemLevelType};
use super::{CoreManager, CoreUtility, ElemParameter};

pub struct ListParameter {

  /// CoreManager element. 
  core_manager: Rc<RefCell<CoreManager>>,

  /// The list of parameters. 
  list_parameter: Vec<ElemParameter>,

  /// The index of the currently selected parameter element. 
  list_index: Cell<usize>,

  /// Element level. 
  elem_level: ElemLevelType

}

/// List of parameters list implementation.

impl ListTrait for ListParameter {

  /// Clear all parameters from the parameter list.

  fn clear(self: &mut Self) -> () {
    
    self.list_parameter.clear();
    self.list_index.set(usize::MAX);
    
    self.set_updated();
  }

  /// Get the count of the parameter list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_parameter.len();
  }

  /// Get the index of the selected parameter (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
  }

  /// Select a parameter based upon an index value.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - Index value of the parameter to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_parameter.len() {
      return false;
    }

    self.set_index(index_param);

    return true;
  }

  /// Set the list index.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - See description.

  fn set_index(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_parameter.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true;
  }

}

/// List of parameters implementation.

impl ListParameter {

  /// Create a new parameter list.
  /// 
  /// # Arguments
  ///
  /// * `core_manager` - CoreManager element.
  /// * `elem_level_param` - Element level
  ///
  /// # Return
  ///
  /// * See description.
  
  pub fn new(core_manager: &Rc<RefCell<CoreManager>>, elem_level_param: ElemLevelType) -> ListParameter {

    return ListParameter {
      core_manager: Rc::clone(core_manager),
      list_parameter: Vec::new(),
      list_index: Cell::new(usize::MAX),
      elem_level: elem_level_param
    }
  }

  /// Add a new parameter into the parameter list.
  /// If the name results in a duplicate entry, an
  /// incrementing number starting from 2 is appended to the
  /// name until a non-duplicate entry is found.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - Name of the parameter.
  /// * `elem_level_param` - Element level
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn add_parameter(self: &mut Self, name_param: &str, updating_json_param: bool) -> bool {

    let mut name: String = String::from(name_param);
    let mut update_element: bool = false;
    
    if self.get_element_by_name(name_param, false) { // Check for duplicate name
      if updating_json_param {
        self.get_element_by_name(name_param, true);
        update_element = true;
      } else {
        let mut temp_name;
        let mut name_index: usize = 2;
        loop {
          temp_name = format!("{}{}", name_param, name_index);
          if !self.get_element_by_name(temp_name.as_str(), false) {
            break;
          }
          name_index += 1;
        }
        name = temp_name;
      }
    }
        
    if update_element {
      match self.list_parameter.iter_mut().nth(self.list_index.get()) {
        None => { return false; }
        Some(o) => {
          o.set_name(name.as_str());
          self.set_updated();
          return true;
        }
      }
    }
    
    let new_elem_param: ElemParameter = ElemParameter::new(name.as_str());

    self.list_parameter.push(new_elem_param);

    match self.list_parameter.iter().position(|e| e.name() == name.as_str()) {
      None => { return false; }
      Some(o) => {
        self.list_index.set(o);    
        self.set_updated();    
        return true;
      }
    }
  }

  /// Performs a deep copy of this parameter list and returns to new parameter list.
  /// 
  /// # Arguments
  ///
  /// * `elem_level_param` - Element level
  /// * `updating_json_param` - Updating from json.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn copy(self: &Self, elem_level_param: ElemLevelType, 
      updating_json_param: bool) -> ListParameter {

    let mut list_parameter = ListParameter::new(&self.core_manager, elem_level_param);

    self.copy_list_parameter(&mut list_parameter, updating_json_param);

    return list_parameter;
  }

  /// Performs a deep copy of this parameter list into the parameter list parameter.
  /// 
  /// # Arguments
  ///
  /// * `list_parameter` - The parameter list to copy into.
  /// * `updating_json_param` - Updating from json.

  pub fn copy_list_parameter(self: &Self, list_parameter: &mut ListParameter, updating_json_param: bool) -> () {

    for elem in self.list_parameter.iter() {

      if list_parameter.get_element_by_name(elem.name(), false) {
        continue; // Already present
      }

      list_parameter.add_parameter(elem.name(), updating_json_param);

      match elem.param_type() {
        crate::TokenType::Integer => { list_parameter.set_integeri(elem.param_integeri()); }
        crate::TokenType::Decimal => { list_parameter.set_decimal(elem.param_float()); }
        _ => { list_parameter.set_string(elem.param_string()); }
      }

    }
  }

  /// Tests if this parameter list and another are equal.
  /// 
  /// # Arguments
  ///
  /// * `list_parameter` - List to compare.
  /// 
  /// # Return
  ///
  /// * True if equals, otherwise false.

  pub fn equal(self: &Self, list_parameter: &ListParameter) -> bool {

    if self.count() != list_parameter.count() {
      return false;
    }

    let mut index: usize = 0;
    while index < self.count() {

      match self.list_parameter.iter().nth(index) {
        None => { return false; }
        Some(o) => {
          match list_parameter.list().iter().nth(index) {
            None => { return false; }
            Some(o2) => { if !o.equal(o2) { return false; } }
          }
        }
      }

      index += 1;
    }

    return true;
  }

  /// Get the vector of parameters.
  /// 
  /// # Return
  ///
  /// * See description.

  fn list(self: &Self) -> &Vec<ElemParameter> {

    return &self.list_parameter;
  }

  /// Get the element level.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn elem_level(self: &Self) -> ElemLevelType {

    return self.elem_level;
  }

  /// Get the name of the parameter.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    match self.list_parameter.iter().nth(self.list_index.get()) {
      None => { panic!("Parameter list index not set"); }
      Some(o) => { return o.name(); }
    }
  }

  /// Get the type of the parameter.
  /// 
  /// # Return
  ///
  /// * See description.
  ///     

  pub fn param_type(self: &Self) -> crate::TokenType {

    match self.list_parameter.iter().nth(self.list_index.get()) {
      None => { panic!("Parameter list index not set"); }
      Some(o) => { return o.param_type(); }
    }
  }

  /// Get the integer value of the parameter.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn param_integeri(self: &Self) -> i32 {

    match self.list_parameter.iter().nth(self.list_index.get()) {
      None => { panic!("Parameter list index not set"); }
      Some(o) => { return o.param_integeri(); }
    }
  }

  /// Get the integer value of the parameter.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn param_integer(self: &Self) -> usize {

    return self.param_integeri() as usize;
  }

  /// Get the decimal value of the parameter.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn param_float(self: &Self) -> Decimal {

    match self.list_parameter.iter().nth(self.list_index.get()) {
      None => { panic!("Parameter list index not set"); }
      Some(o) => { return o.param_float(); }
    }
  }

  /// Get the string value of the parameter.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn param_string(self: &Self) -> &str {

    match self.list_parameter.iter().nth(self.list_index.get()) {
      None => { panic!("Parameter list index not set"); }
      Some(o) => { return o.param_string(); }
    }
  }

  /// Select a parameter based upon a name.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - The name of the parameter to select.
  /// * `select_param` - If true select element, otherwise restore current element.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn get_element_by_name(self: &Self, name_param: &str, select_param: bool) -> bool {
    
    let mut index: usize = 0;

    for elem in self.list_parameter.iter() {
      if name_param == elem.name() {
        if select_param {
          self.set_index(index);
        }
        return true;
      }

      index += 1;
    }

    return false;
  }

  /// Move the selected parameter up or down in the parameter list.
  /// 
  /// # Arguments
  ///
  /// * `is_up` - Move up, otherwise down.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn move_param(self: &mut Self, is_up: bool) -> bool {
    
    let name: String;
    match self.list_parameter.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => { name = String::from(o.name()); }
    }

    if is_up {
      if self.list_index.get() <= 0 {
        return false;
      }
      let elem = self.list_parameter.remove(self.list_index.get());
      self.list_parameter.insert(self.list_index.get() - 1, elem);
    } else {
      if self.list_index.get() + 1 >= self.list_parameter.len() {
        return false;
      }
      let elem = self.list_parameter.remove(self.list_index.get());
      self.list_parameter.insert(self.list_index.get() + 1, elem);
    }

    match self.list_parameter.iter().position(|e| e.name() == name) {
      None => { return false; }
      Some(o) => { 
        self.list_index.set(o);
        self.set_updated();    
        return true;
      }
    }        
  }

  /// Remove the selected parameter from the parameter list.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn remove(self: &mut Self) -> bool {
    if self.list_index.get() >= self.list_parameter.len() {
      return false;
    }

    self.list_parameter.remove(self.list_index.get());
    
    if self.list_index.get() > 0 {
      self.list_index.set(self.list_index.get() - 1);
    }
    
    self.set_updated();
    
    return true;
  }

  /// Set the name of the parameter.
  /// Duplicate names are not allowed.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn set_name(self: &mut Self, name_param: &str) -> bool {

    if self.get_element_by_name(name_param, false) {
      return false;
    }

    match self.list_parameter.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_name(name_param);
        self.set_updated();
        return true;
      }
    }
  }

  /// Set the type of parameter.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
    
  pub fn set_type(self: &mut Self, value_param: crate::TokenType) -> bool {

    match self.list_parameter.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_type(value_param);
        self.set_updated();
        return true;
      }
    }
  }

  /// Set the integer value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
    
  pub fn set_integeri(self: &mut Self, value_param: i32) -> bool {

    match self.list_parameter.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_integeri(value_param);
        self.set_updated();
        return true;
      }
    }
  }

  /// Set the integer value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
    
  pub fn set_integer(self: &mut Self, value_param: usize) -> bool {

    return self.set_integeri(value_param as i32);
  }

  /// Set the decimal value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
    
  pub fn set_decimal(self: &mut Self, value_param: Decimal) -> bool {

    match self.list_parameter.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_decimal(value_param);
        self.set_updated();
        return true;
      }
    }
  }

  /// Set the string value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
    
  pub fn set_string(self: &mut Self, value_param: &str) -> bool {

    match self.list_parameter.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_string(value_param);
        self.set_updated();
        return true;
      }
    }
  }

  /// Call the updated signal.
  
  fn set_updated(self: &Self) -> () {

    self.core_manager.borrow().notify(
      CoreUtility::format_update(ElemUpdateType::Parameter, self.elem_level));
  }

}