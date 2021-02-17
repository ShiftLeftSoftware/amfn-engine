//! List of columns element.
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
use rust_decimal::prelude::*;

use crate::ListTrait;
use super::ElemColumn;

pub struct ListColumn {

  /// The list of column elements. 
  list_column: Vec<ElemColumn>,
  
  /// The index of the currently selected column element. 
  list_index: Cell<usize>

}

/// List of columns list implementation.

impl ListTrait for ListColumn {

  /// Clear all columns from the column list.

  fn clear(self: &mut Self) -> () {
    
    self.list_column.clear();    
    self.list_index = Cell::new(usize::MAX);
  }

  /// Get the count of the columns.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_column.len();
  }

  /// Get the index of the selected column (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
  }

  /// Select a column based upon an index value.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - The index value of the column to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_column.len() {
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

    if index_param >= self.list_column.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true;
  }

}

/// List of columns implementation.

impl ListColumn {

  /// Create and return a new list column element.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new() -> ListColumn {

    return ListColumn {
      list_column: Vec::new(),
      list_index: Cell::new(usize::MAX)
    }
  }

  /// Add a new column into the column list.
  /// 
  /// # Arguments
  ///
  /// * `col_name_param` - Column name.
  /// * `col_name_index_param` - Column index.
  /// * `col_header_param` - Column header.
  /// * `col_description_param` - Column description.
  /// * `group_param` - Group parameter.
  /// * `name_param` - Name parameter.
  /// * `col_type_param` - Column type.
  /// * `code_param` - Code parameter.
  /// * `column_empty_value_param` - Column empty value.
  /// * `format_param` - Column format.
  /// * `decimal_digits_param` - Decimal digits.
  /// * `column_width_param` - Column width.
  /// * `column_exclude_param` - Column exclude.
  ///
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn add_column(self: &mut Self, col_name_param: &str, col_name_index_param: usize,
    col_header_param: &str, col_description_param: &str, group_param: &str,
    name_param: &str, col_type_param: &str, code_param: &str, column_empty_value_param: Decimal,
    format_param: crate::FormatType, decimal_digits_param: usize, column_width_param: usize, 
    column_exclude_param: bool) -> bool {

    let new_elem_column: ElemColumn = ElemColumn::new(
      col_name_param, col_name_index_param, col_header_param, col_description_param,
      group_param, name_param, col_type_param, code_param, column_empty_value_param,
      format_param, decimal_digits_param, column_width_param, column_exclude_param, false);

    let new_index: usize = self.list_column.len();
    self.list_column.push(new_elem_column);
    self.list_index.set(new_index);
    
    return true;
  }

  /// Get the name of the column.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_name(self: &Self) -> &str {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.col_name(); }
    }
  }

  /// Index of the column name.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_name_index(self: &Self) -> usize {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.col_name_index(); }
    }
  }

  /// Get the header text of the column.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_header(self: &Self) -> &str {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.col_header(); }
    }
  }

  /// Get the description of the column.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_description(self: &Self) -> &str {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.col_description(); }
    }
  }

  /// Get the group of the descriptor.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn group(self: &Self) -> &str {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.group(); }
    }
  }

  /// Get the name of the descriptor.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.name(); }
    }
  }

  /// Get the type of the descriptor.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_type(self: &Self) -> &str {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.col_type(); }
    }
  }

  /// Get the code of the descriptor.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn code(self: &Self) -> &str {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.code(); }
    }
  }
  
  /// Get the column empty value (Enabled when >= 0).
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_empty_value(self: &Self) -> Decimal {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.column_empty_value(); }
    }
  }

  /// Get the format of the column.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format(self: &Self) -> crate::FormatType {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.format(); }
    }
  }

  /// Get the number of significant decimal digits.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_digits(self: &Self) -> usize {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.decimal_digits(); }
    }
  }

  /// Get the width of the column.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_width(self: &Self) -> usize {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.column_width(); }
    }
  }

  /// Get the column exclude.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_exclude(self: &Self) -> bool {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.column_exclude(); }
    }
  }

  /// Get the column empty flag.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_empty(self: &Self) -> bool {

    match self.list_column.iter().nth(self.list_index.get()) {
      None => { panic!("Column list index not set"); }
      Some(o) => { return o.column_empty(); }
    }
  }

  /// Select the column that matches a column name index.
  /// 
  /// # Arguments
  ///
  /// * `col_name_index` - The column name index to select.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn get_element_by_col_name_index(self: &Self, col_name_index: usize) -> bool {
    
    let mut index: usize = 0;

    for elem_key in self.list_column.iter() {

      if elem_key.col_name_index() == col_name_index {
        self.set_index(index);
        return true;
      }

      index += 1;
    }

    return false;
  }

  /// Remove the selected column from the column list.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn remove(self: &mut Self) -> bool {
    if self.list_index.get() >= self.list_column.len() {
      return false;
    }

    self.list_column.remove(self.list_index.get());
    
    if self.list_index.get() > 0 {
      self.list_index.set(self.list_index.get() - 1);
    }
    
    return true;
  }

  /// Set the column empty flag.
  /// 
  /// # Arguments
  ///
  /// * `column_empty_param` - See description.

  pub fn set_column_empty(self: &mut Self, column_empty_param: bool) -> bool {

    match self.list_column.iter_mut().nth(self.list_index.get()) {
      None => {  return false; }
      Some(o) => { 
        o.set_column_empty(column_empty_param); 
        return true;
      }
    }
  }
  
}