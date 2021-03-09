//! List of columns element.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::Cell;

use super::ElemColumn;
use crate::ListTrait;

pub struct ListColumn {
    /// The list of column elements.
    list_column: Vec<ElemColumn>,

    /// The index of the currently selected column element.
    list_index: Cell<usize>,
}

/// List of columns list implementation.

impl ListTrait for ListColumn {
    /// Clear all columns from the column list.

    fn clear(&mut self) {
        self.list_column.clear();
        self.list_index = Cell::new(usize::MAX);
    }

    /// Get the count of the columns.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_column.len()
    }

    /// Get the index of the selected column (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
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

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_column.len() {
            return false;
        }

        self.set_index(index_param);

        true
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

    fn set_index(&self, index_param: usize) -> bool {
        if index_param >= self.list_column.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of columns default implementation.

impl Default for ListColumn {
    /// Create and return a new list column element.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListColumn::new()
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
        ListColumn {
            list_column: Vec::new(),
            list_index: Cell::new(usize::MAX),
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
    #[allow(clippy::too_many_arguments)]

    pub fn add_column(
        &mut self,
        col_name_param: &str,
        col_name_index_param: usize,
        col_header_param: &str,
        col_description_param: &str,
        group_param: &str,
        name_param: &str,
        col_type_param: &str,
        code_param: &str,
        column_empty_value_param: Decimal,
        format_param: crate::FormatType,
        decimal_digits_param: usize,
        column_width_param: usize,
        column_exclude_param: bool,
    ) -> bool {
        let new_elem_column: ElemColumn = ElemColumn::new(
            col_name_param,
            col_name_index_param,
            col_header_param,
            col_description_param,
            group_param,
            name_param,
            col_type_param,
            code_param,
            column_empty_value_param,
            format_param,
            decimal_digits_param,
            column_width_param,
            column_exclude_param,
            false,
        );

        let new_index: usize = self.list_column.len();
        self.list_column.push(new_elem_column);
        self.list_index.set(new_index);
        true
    }

    /// Get the vector of columns.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list(&self) -> &Vec<ElemColumn> {
        &self.list_column
    }

    /// Get the name of the column.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn col_name(&self) -> &str {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.col_name(),
        }
    }

    /// Index of the column name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn col_name_index(&self) -> usize {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.col_name_index(),
        }
    }

    /// Get the header text of the column.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn col_header(&self) -> &str {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.col_header(),
        }
    }

    /// Get the description of the column.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn col_description(&self) -> &str {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.col_description(),
        }
    }

    /// Get the group of the descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn group(&self) -> &str {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.group(),
        }
    }

    /// Get the name of the descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.name(),
        }
    }

    /// Get the type of the descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn col_type(&self) -> &str {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.col_type(),
        }
    }

    /// Get the code of the descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn code(&self) -> &str {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.code(),
        }
    }

    /// Get the column empty value (Enabled when >= 0).
    ///
    /// # Return
    ///
    /// * See description.

    pub fn column_empty_value(&self) -> Decimal {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.column_empty_value(),
        }
    }

    /// Get the format of the column.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn format(&self) -> crate::FormatType {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.format(),
        }
    }

    /// Get the number of significant decimal digits.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn decimal_digits(&self) -> usize {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.decimal_digits(),
        }
    }

    /// Get the width of the column.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn column_width(&self) -> usize {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.column_width(),
        }
    }

    /// Get the column exclude.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn column_exclude(&self) -> bool {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.column_exclude(),
        }
    }

    /// Get the column empty flag.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn column_empty(&self) -> bool {
        match self.list_column.get(self.list_index.get()) {
            None => {
                panic!("Column list index not set");
            }
            Some(o) => o.column_empty(),
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

    pub fn get_element_by_col_name_index(&self, col_name_index: usize) -> bool {
        for (index, elem_key) in self.list_column.iter().enumerate() {
            if elem_key.col_name_index() == col_name_index {
                self.set_index(index);
                return true;
            }
        }

        false
    }

    /// Remove the selected column from the column list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_column.len() {
            return false;
        }

        self.list_column.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }
        true
    }

    /// Set the column empty flag.
    ///
    /// # Arguments
    ///
    /// * `column_empty_param` - See description.

    pub fn set_column_empty(&mut self, column_empty_param: bool) -> bool {
        match self.list_column.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_column_empty(column_empty_param);
                true
            }
        }
    }
}
