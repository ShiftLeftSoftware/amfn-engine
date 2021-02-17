//! Utility element representing a list of keys.
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

use std::vec::Vec;
use std::cell::Cell;

use crate::ListTrait;
use super::ElemKey;

pub struct ListKey {

  /// The list of keys. 
  list_key: Vec<ElemKey>,

  /// The index of the currently selected key element. 
  list_index: Cell<usize>

}

/// List of keys list implementation.

impl ListTrait for ListKey {

  /// Clear all keys from the key list.

  fn clear(self: &mut Self) -> () {
    
    self.list_key.clear();
    self.list_index.set(usize::MAX);
  }

  /// Get the count of the key list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_key.len();
  }

  /// Get the index of the selected key (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
  }

  /// Select a key based upon an index.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - Index of the key to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_key.len() {
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
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn set_index(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_key.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true;
  }

}

/// Utility element representing a list of keys implementation.

impl ListKey {

  /// Create and return a new list of keys.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new() -> ListKey {
    
    return ListKey {
      list_key: Vec::new(),
      list_index: Cell::new(usize::MAX)
    }
  }

  /// Add a new key into the keys list.
  /// Duplicate keys are allowed.
  /// 
  /// # Arguments
  ///
  /// * `key` - Name of the key.
  /// * `value` - Value of the key.
  /// * `value_ext` - Extension value of the key.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn add_key(self: &mut Self, key: &str, value: usize, value_ext: usize) -> bool {
    let mut new_elem_key: ElemKey = ElemKey::new();
    
    new_elem_key.set_key(key);
    new_elem_key.set_value(value);
    new_elem_key.set_value_ext(value_ext);

    self.list_key.push(new_elem_key);

    match self.list_key.iter().position(|e| e.key() == key) {
      None => { return false; }
      Some(o) => { self.list_index.set(o); }
    }

    return true;
  }

  /// Get the name of the key.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn key(self: &Self) -> &str {

    match self.list_key.iter().nth(self.list_index.get()) {
      None => { panic!("Key list index not set"); }
      Some(o) => { return o.key(); }
    }
  }

  /// Get the value of the key.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn value(self: &Self) -> usize {

    match self.list_key.iter().nth(self.list_index.get()) {
      None => { panic!("Key list index not set"); }
      Some(o) => { return o.value(); }
    }
  }

  /// Get the extension value of the key.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn value_ext(self: &Self) -> usize {

    match self.list_key.iter().nth(self.list_index.get()) {
      None => { panic!("Key list index not set"); }
      Some(o) => { return o.value_ext(); }
    }
  }

  /// Select the first key that matches a name.
  /// 
  /// # Arguments
  ///
  /// * `key` - The name of the key to select.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn get_element_by_key(self: &Self, key: &str) -> bool {

    let mut index: usize = 0;
    
    for elem in self.list_key.iter() {

      let tokens: Vec<_> =  elem.key().split('(').collect();

      if key == tokens[0].trim() {
        self.set_index(index);
        return true;
      }

      index += 1;
    }

    return false;
  }

  /// Select the first key that matches a value.
  /// 
  /// # Arguments
  ///
  /// * `value` - The value of the key to select.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn get_element_by_value(self: &Self, value: usize) -> bool {

    let mut index: usize = 0;
    
    for elem in self.list_key.iter() {

      if elem.value() == value {
        self.set_index(index);
        return true;
      }

      index += 1;
    }

    return false;
  }

  /// Get the extension value of a key based upon an index.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - Index of the key to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn get_value_ext_by_index(self: &Self, index_param: usize) -> usize {

    match self.list_key.iter().nth(index_param) {
      None => { return 0; }
      Some(o) => { return o.value_ext(); }
    }
  }

}