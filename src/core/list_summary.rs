//! List of summary items.
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

use std::cell::Cell;

use crate::ListTrait;
use super::ElemSummary;

pub struct ListSummary {

  /// The list of summary items. 
  list_summary: Vec<ElemSummary>,

  /// The index of the currently selected summary item. 
  list_index: Cell<usize>

}

/// List of summary items list implementation.

impl ListTrait for ListSummary {

  /// Clear all summary items from the summary item list.

  fn clear(self: &mut Self) -> () {
    
    self.list_summary.clear();    
    self.list_index.set(usize::MAX);
  }

  /// Get the count of the summary list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_summary.len();
  }

  /// Get the index of the selected summary item (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
  }

  /// Select a summary item based upon an index value.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - The index value of the summary item to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_summary.len() {
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

    if index_param >= self.list_summary.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true;
  }

}

/// List of summary items implementation.

impl ListSummary {

  /// Create and return a new list of summary elements.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new() -> ListSummary {

    return ListSummary {
      list_summary: Vec::new(),
      list_index: Cell::new(usize::MAX)
    }
  }

  /// Add a new summary item into the summary item list.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - Summary element name.
  /// * `label_param` - Summary element label.
  /// * `label_expr_param` - Element label expression.
  /// * `result_param` - Summary element result.
  /// * `result_expr_param` - Element result expression.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn add_summary(self: &mut Self, name_param: &str, label_param: &str, label_expr_param: &str, 
    result_param: &str, result_expr_param: &str) -> bool {

    let new_elem_sum: ElemSummary = ElemSummary::new(
      name_param, label_param, label_expr_param, result_param, result_expr_param);

    self.list_summary.push(new_elem_sum);

    match self.list_summary.iter().position(|e| e.name() == name_param) {
      None => { return false; }
      Some(o) => {
        self.list_index.set(o);
        return true;
      }
    }
  }

  /// Get the name of the summary item.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    match self.list_summary.iter().nth(self.list_index.get()) {
      None => { panic!("Summary list index not set"); }
      Some(o) => { return o.name(); }
    }
 }

  /// Get the label of the summary item.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn label(self: &Self) -> &str {

    match self.list_summary.iter().nth(self.list_index.get()) {
      None => { panic!("Summary list index not set"); }
      Some(o) => { return o.label(); }
    }
  }

  /// Get the label expression of the summary item.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn label_expr(self: &Self) -> &str {

    match self.list_summary.iter().nth(self.list_index.get()) {
      None => { panic!("Summary list index not set"); }
      Some(o) => { return o.label_expr(); }
    }
  }

  /// Get the result of the summary item.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn result(self: &Self) -> &str {

    match self.list_summary.iter().nth(self.list_index.get()) {
      None => { panic!("Summary list index not set"); }
      Some(o) => { return o.result(); }
    }
  }

  /// Get the result expression of the summary item.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn result_expr(self: &Self) -> &str {

    match self.list_summary.iter().nth(self.list_index.get()) {
      None => { panic!("Summary list index not set"); }
      Some(o) => { return o.result_expr(); }
    }
  }

  /// Remove the selected summary item from the summary item list.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn remove(self: &mut Self) -> bool {
    if self.list_index.get() >= self.list_summary.len() {
      return false;
    }

    self.list_summary.remove(self.list_index.get());
    
    if self.list_index.get() > 0 {
      self.list_index.set(self.list_index.get() - 1);
    }
    
    return true;
  }
  
}