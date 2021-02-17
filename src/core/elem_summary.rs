//! Element summary definition.
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
  result_expr: String

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

  pub fn new(name_param: &str, label_param: &str, label_expr_param: &str, 
    result_param: &str, result_expr_param: &str) -> ElemSummary {

    return ElemSummary {
      name: String::from(name_param),
      label: String::from(label_param),
      label_expr: String::from(label_expr_param),
      result: String::from(result_param),
      result_expr: String::from(result_expr_param)
    }
  }

  /// Get the name of the element summary.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the label of the element summary.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn label(self: &Self) -> &str {

    return self.label.as_str();
  }

  /// Get the label expression of the element summary.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn label_expr(self: &Self) -> &str {

    return self.label_expr.as_str();
  }

  /// Get the result of the element summary.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn result(self: &Self) -> &str {

    return self.result.as_str();
  }

  /// Get the result expression of the element summary.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn result_expr(self: &Self) -> &str {

    return self.result_expr.as_str();
  }

  /// Set the name of the element summary.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - See description.

  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the label of the element summary.
  /// 
  /// # Arguments
  ///
  /// * `label_param` - See description.

  pub fn set_label(self: &mut Self, label_param: &str) -> () {

    self.label = String::from(label_param);
  }

  /// Set the label expression of the element summary.
  /// 
  /// # Arguments
  ///
  /// * `label_expr_param` - See description.

  pub fn set_label_expr(self: &mut Self, label_expr_param: &str) -> () {

    self.label_expr = String::from(label_expr_param);
  }

  /// Set the result of the element summary.
  /// 
  /// # Arguments
  ///
  /// * `result_param` - See description.

  pub fn set_result(self: &mut Self, result_param: &str) -> () {

    self.result = String::from(result_param);
  }

  /// Set the result expression of the element summary.
  /// 
  /// # Arguments
  ///
  /// * `result_expr_param` - See description.

  pub fn set_result_expr(self: &mut Self, result_expr_param: &str) -> () {

    self.result_expr = String::from(result_expr_param);
  }

}