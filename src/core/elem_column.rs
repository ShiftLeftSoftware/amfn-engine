//! The column element definition.
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

use rust_decimal::prelude::*;

pub struct ElemColumn {
  /// Name of the column. 
  col_name: String, 
  /// Index of the column name. 
  col_name_index: usize,
  /// Header text of the column. 
  col_header: String,
  /// Description of the column. 
  col_description: String,
  /// Group of the descriptor. 
  group: String,
  /// Name of the descriptor. 
  name: String,
  /// Type of the descriptor. 
  col_type: String,
  /// Code of the descriptor. 
  code: String,
  /// Column empty value (Enabled when >= 0). 
  column_empty_value: Decimal,
  /// Format of the column. 
  format: crate::FormatType,
  /// Number of significant decimal digits. 
  decimal_digits: usize,
  /// Width of column. 
  column_width: usize,
  /// Column exclude. 
  column_exclude: bool,
  /// Column empty. 
  column_empty: bool
  
}

/// The column element implementation.

impl ElemColumn {

  /// Create a new column element.
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
  /// * `column_empty_param` - Column empty.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(col_name_param: &str, col_name_index_param: usize, col_header_param: &str, 
    col_description_param: &str, group_param: &str, name_param: &str, col_type_param: &str, 
    code_param: &str, column_empty_value_param: Decimal, format_param: crate::FormatType, 
    decimal_digits_param: usize, column_width_param: usize, column_exclude_param: bool, 
    column_empty_param: bool) -> ElemColumn {

    return ElemColumn {
      col_name: String::from(col_name_param), 
      col_name_index: col_name_index_param, 
      col_header: String::from(col_header_param), 
      col_description: String::from(col_description_param),
      group: String::from(group_param), 
      name: String::from(name_param), 
      col_type: String::from(col_type_param), 
      code: String::from(code_param), 
      column_empty_value: column_empty_value_param,
      format: format_param, 
      decimal_digits: decimal_digits_param, 
      column_width: column_width_param, 
      column_exclude: column_exclude_param, 
      column_empty: column_empty_param
    }
  }

  /// Get the column name.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_name(self: &Self) -> &str {

    return self.col_name.as_str();
  }

  /// Get the column name index.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_name_index(self: &Self) -> usize {

    return self.col_name_index;
  }

  /// Get the column header.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_header(self: &Self) -> &str {

    return self.col_header.as_str();
  }

  /// Get the column description.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_description(self: &Self) -> &str {

    return self.col_description.as_str();
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

  /// Get the descriptor type.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn col_type(self: &Self) -> &str {

    return self.col_type.as_str();
  }

  /// Get the descriptor code.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn code(self: &Self) -> &str {

    return self.code.as_str();
  }

  /// Get the column empty value.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_empty_value(self: &Self) -> Decimal {

    return self.column_empty_value;
  }

  /// Get the column format.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn format(self: &Self) -> crate::FormatType {

    return self.format;
  }

  /// Get the decimal digits.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn decimal_digits(self: &Self) -> usize {

    return self.decimal_digits;
  }

  /// Get the column width.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_width(self: &Self) -> usize {

    return self.column_width;
  }

  /// Get the column exclude.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_exclude(self: &Self) -> bool {

    return self.column_exclude;
  }

  /// Get the column empty.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn column_empty(self: &Self) -> bool {

    return self.column_empty;
  }

  /// Set the column name.
  /// 
  /// # Arguments
  ///
  /// * `col_name_param` - See description.
    
  pub fn set_col_name(self: &mut Self, col_name_param: &str) -> () {

    self.col_name = String::from(col_name_param);
  }

  /// Set the column name index.
  /// 
  /// # Arguments
  ///
  /// * `col_name_index_param` - See description.
    
  pub fn set_col_name_index(self: &mut Self, col_name_index_param: usize) -> () {

    self.col_name_index = col_name_index_param;
  }

  /// Set the column header.
  /// 
  /// # Arguments
  ///
  /// * `col_header_param` - See description.
    
  pub fn set_col_header(self: &mut Self, col_header_param: &str) -> () {

    self.col_header = String::from(col_header_param);
  }

  /// Set the column description.
  /// 
  /// # Arguments
  ///
  /// * `col_desc_param` - See description.
    
  pub fn set_col_description(self: &mut Self, col_desc_param: &str) -> () {

    self.col_description = String::from(col_desc_param);
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
  /// * `name_param` - See description.
    
  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the descriptor type.
  /// 
  /// # Arguments
  ///
  /// * `col_type_param` - See description.
    
  pub fn set_col_type(self: &mut Self, col_type_param: &str) -> () {

    self.col_type = String::from(col_type_param);
  }

  /// Set the descriptor code.
  /// 
  /// # Arguments
  ///
  /// * `code_param` - See description.
    
  pub fn set_code(self: &mut Self, code_param: &str) -> () {

    self.code = String::from(code_param);
  }

  /// Set the empty value.
  /// 
  /// # Arguments
  ///
  /// * `empty_value_param` - See description.
    
  pub fn set_empty_value(self: &mut Self, empty_value_param: Decimal) -> () {

    self.column_empty_value = empty_value_param;
  }

  /// Set the column format.
  /// 
  /// # Arguments
  ///
  /// * `format_param` - See description.
    
  pub fn set_format(self: &mut Self, format_param: crate::FormatType) -> () {

    self.format = format_param;
  }

  /// Set the decimal digits.
  /// 
  /// # Arguments
  ///
  /// * `dec_digits_param` - See description.
    
  pub fn set_decimal_digits(self: &mut Self, dec_digits_param: usize) -> () {

    self.decimal_digits = dec_digits_param;
  }

  /// Set the column width.
  /// 
  /// # Arguments
  ///
  /// * `width_param` - See description.
    
  pub fn set_column_width(self: &mut Self, width_param: usize) -> () {

    self.column_width = width_param;
  }

  /// Set the column exclude.
  /// 
  /// # Arguments
  ///
  /// * `exclude_param` - See description.
    
  pub fn set_column_exclude(self: &mut Self, exclude_param: bool) -> () {

    self.column_exclude = exclude_param;
  }

  /// Set the column empty.
  /// 
  /// # Arguments
  ///
  /// * `column_empty_param` - See description.
    
  pub fn set_column_empty(self: &mut Self, column_empty_param: bool) -> () {

    self.column_empty = column_empty_param;
  }

}