//! The key element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct ElemKey {
    /// Name of the key.
    key: String,
    /// Value of the key.
    value: usize,
    /// Extension value of the key.
    value_ext: usize,
}

/// The key implementation.

impl ElemKey {
    /// Create a new object.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new() -> ElemKey {
        ElemKey {
            key: String::from(""),
            value: 0,
            value_ext: 0,
        }
    }

    /// Get the key of symbol.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    /// Get the value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value(&self) -> usize {
        self.value
    }

    /// Get the value extension.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_ext(&self) -> usize {
        self.value_ext
    }

    /// Set the key value.
    ///
    /// # Arguments
    ///
    /// * `key_param` - See description.

    pub fn set_key(&mut self, key_param: &str) {
        self.key = String::from(key_param);
    }

    /// Set the value.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.

    pub fn set_value(&mut self, value_param: usize) {
        self.value = value_param;
    }

    /// Set the value extension.
    ///
    /// # Arguments
    ///
    /// * `value_ext_param` - See description.

    pub fn set_value_ext(&mut self, value_ext_param: usize) {
        self.value_ext = value_ext_param;
    }
}
