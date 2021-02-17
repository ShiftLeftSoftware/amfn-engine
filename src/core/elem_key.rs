//! The key element definition.
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

pub struct ElemKey {

    /// Name of the key. 
    key: String,
    /// Value of the key. 
    value: usize,
    /// Extension value of the key. 
    value_ext: usize

}

/// The key implementation.

impl ElemKey {

  /// Create a new object.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new() -> ElemKey {
    
    return ElemKey {
      key: String::from(""),
      value: 0,
      value_ext: 0
    }
  }

  /// Get the key of symbol.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn key(self: &Self) -> &str {

    return self.key.as_str();
  }

  /// Get the value.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn value(self: &Self) -> usize {

    return self.value;
  }

  /// Get the value extension.
  /// 
  /// # Return
  ///
  /// * See description.
    
  pub fn value_ext(self: &Self) -> usize {

    return self.value_ext;
  }

  /// Set the key value.
  /// 
  /// # Arguments
  ///
  /// * `key_param` - See description.
    
  pub fn set_key(self: &mut Self, key_param: &str) -> () {

    self.key = String::from(key_param);
  }

  /// Set the value.
  /// 
  /// # Arguments
  ///
  /// * `value_param` - See description.
    
  pub fn set_value(self: &mut Self, value_param: usize) -> () {

    self.value = value_param;
  }

  /// Set the value extension.
  /// 
  /// # Arguments
  ///
  /// * `value_ext_param` - See description.
    
  pub fn set_value_ext(self: &mut Self, value_ext_param: usize) -> () {

    self.value_ext = value_ext_param;
  }

}