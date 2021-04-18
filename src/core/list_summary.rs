//! List of summary items.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;

use super::ElemSummary;
use crate::ListTrait;

pub struct ListSummary {
    /// The list of summary items.
    list_summary: Vec<ElemSummary>,

    /// The index of the currently selected summary item.
    list_index: Cell<usize>,
}

/// List of summary items list implementation.

impl ListTrait for ListSummary {
    /// Clear all summary items from the summary item list.

    fn clear(&mut self) {
        self.list_summary.clear();
        self.list_index.set(usize::MAX);
    }

    /// Get the count of the summary list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_summary.len()
    }

    /// Get the index of the selected summary item (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
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

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_summary.len() {
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
        if index_param >= self.list_summary.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of summary items default implementation.

impl Default for ListSummary {
    /// Create and return a new list of summary elements.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListSummary::new()
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
        ListSummary {
            list_summary: Vec::new(),
            list_index: Cell::new(usize::MAX),
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

    pub fn add_summary(
        &mut self,
        name_param: &str,
        label_param: &str,
        label_expr_param: &str,
        result_param: &str,
        result_expr_param: &str,
    ) -> bool {
        let new_elem_sum: ElemSummary = ElemSummary::new(
            name_param,
            label_param,
            label_expr_param,
            result_param,
            result_expr_param,
        );

        self.list_summary.push(new_elem_sum);

        match self
            .list_summary
            .iter()
            .position(|e| e.name() == name_param)
        {
            None => false,
            Some(o) => {
                self.list_index.set(o);
                true
            }
        }
    }

    /// Get the name of the summary item.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        match self.list_summary.get(self.list_index.get()) {
            None => {
                panic!("Summary list index not set");
            }
            Some(o) => o.name(),
        }
    }

    /// Get the label of the summary item.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn label(&self) -> &str {
        match self.list_summary.get(self.list_index.get()) {
            None => {
                panic!("Summary list index not set");
            }
            Some(o) => o.label(),
        }
    }

    /// Get the label expression of the summary item.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn label_expr(&self) -> &str {
        match self.list_summary.get(self.list_index.get()) {
            None => {
                panic!("Summary list index not set");
            }
            Some(o) => o.label_expr(),
        }
    }

    /// Get the result of the summary item.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn result(&self) -> &str {
        match self.list_summary.get(self.list_index.get()) {
            None => {
                panic!("Summary list index not set");
            }
            Some(o) => o.result(),
        }
    }

    /// Get the result expression of the summary item.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn result_expr(&self) -> &str {
        match self.list_summary.get(self.list_index.get()) {
            None => {
                panic!("Summary list index not set");
            }
            Some(o) => o.result_expr(),
        }
    }

    /// Remove the selected summary item from the summary item list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_summary.len() {
            return false;
        }

        self.list_summary.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }
        true
    }
}
