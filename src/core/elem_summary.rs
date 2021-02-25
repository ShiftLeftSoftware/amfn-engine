//! Element summary definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct ElemSummary {
    /// Name of the summary item.
    name: String,
    /// Label text of the summary item.
    label: String,
    /// Label expression of the summary item.
    label_expr: String,
    /// Result text of the summary item.
    result: String,
    /// Result expression of the summary item.
    result_expr: String,
}

/// Element summary implementation.

impl ElemSummary {
    /// Create a new summary element.
    ///
    /// # Arguments
    ///
    /// * `name_param` - Summary element name.
    /// * `label_param` - Label parameter.
    /// * `label_expr_param` - Label expression.
    /// * `result_param` - Result parameter.
    /// * `result_expr_param` - Result expression.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        name_param: &str,
        label_param: &str,
        label_expr_param: &str,
        result_param: &str,
        result_expr_param: &str,
    ) -> ElemSummary {
        ElemSummary {
            name: String::from(name_param),
            label: String::from(label_param),
            label_expr: String::from(label_expr_param),
            result: String::from(result_param),
            result_expr: String::from(result_expr_param),
        }
    }

    /// Get the name of the element summary.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the label of the element summary.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Get the label expression of the element summary.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn label_expr(&self) -> &str {
        self.label_expr.as_str()
    }

    /// Get the result of the element summary.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn result(&self) -> &str {
        self.result.as_str()
    }

    /// Get the result expression of the element summary.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn result_expr(&self) -> &str {
        self.result_expr.as_str()
    }

    /// Set the name of the element summary.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.

    pub fn set_name(&mut self, name_param: &str) {
        self.name = String::from(name_param);
    }

    /// Set the label of the element summary.
    ///
    /// # Arguments
    ///
    /// * `label_param` - See description.

    pub fn set_label(&mut self, label_param: &str) {
        self.label = String::from(label_param);
    }

    /// Set the label expression of the element summary.
    ///
    /// # Arguments
    ///
    /// * `label_expr_param` - See description.

    pub fn set_label_expr(&mut self, label_expr_param: &str) {
        self.label_expr = String::from(label_expr_param);
    }

    /// Set the result of the element summary.
    ///
    /// # Arguments
    ///
    /// * `result_param` - See description.

    pub fn set_result(&mut self, result_param: &str) {
        self.result = String::from(result_param);
    }

    /// Set the result expression of the element summary.
    ///
    /// # Arguments
    ///
    /// * `result_expr_param` - See description.

    pub fn set_result_expr(&mut self, result_expr_param: &str) {
        self.result_expr = String::from(result_expr_param);
    }
}
