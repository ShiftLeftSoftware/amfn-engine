//! Utility element representing a list of keys.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;
use std::vec::Vec;

use super::ElemKey;
use crate::ListTrait;

pub struct ListKey {
    /// The list of keys.
    list_key: Vec<ElemKey>,

    /// The index of the currently selected key element.
    list_index: Cell<usize>,
}

/// List of keys list implementation.

impl ListTrait for ListKey {
    /// Clear all keys from the key list.

    fn clear(&mut self) {
        self.list_key.clear();
        self.list_index.set(usize::MAX);
    }

    /// Get the count of the key list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_key.len()
    }

    /// Get the index of the selected key (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
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

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_key.len() {
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
        if index_param >= self.list_key.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// Utility element representing a list of keys default implementation.

impl Default for ListKey {
    /// Create and return a new list of keys.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListKey::new()
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
        ListKey {
            list_key: Vec::new(),
            list_index: Cell::new(usize::MAX),
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

    pub fn add_key(&mut self, key: &str, value: usize, value_ext: usize) -> bool {
        let mut new_elem_key: ElemKey = ElemKey::new();
        new_elem_key.set_key(key);
        new_elem_key.set_value(value);
        new_elem_key.set_value_ext(value_ext);

        self.list_key.push(new_elem_key);

        match self.list_key.iter().position(|e| e.key() == key) {
            None => {
                return false;
            }
            Some(o) => {
                self.list_index.set(o);
            }
        }

        true
    }

    /// Get the name of the key.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn key(&self) -> &str {
        match self.list_key.get(self.list_index.get()) {
            None => {
                panic!("Key list index not set");
            }
            Some(o) => o.key(),
        }
    }

    /// Get the value of the key.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value(&self) -> usize {
        match self.list_key.get(self.list_index.get()) {
            None => {
                panic!("Key list index not set");
            }
            Some(o) => o.value(),
        }
    }

    /// Get the extension value of the key.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_ext(&self) -> usize {
        match self.list_key.get(self.list_index.get()) {
            None => {
                panic!("Key list index not set");
            }
            Some(o) => o.value_ext(),
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

    pub fn get_element_by_key(&self, key: &str) -> bool {
        for (index, elem) in self.list_key.iter().enumerate() {
            let tokens: Vec<_> = elem.key().split('(').collect();

            if key == tokens[0].trim() {
                self.set_index(index);
                return true;
            }
        }

        false
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

    pub fn get_element_by_value(&self, value: usize) -> bool {
        for (index, elem) in self.list_key.iter().enumerate() {
            if elem.value() == value {
                self.set_index(index);
                return true;
            }
        }

        false
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

    pub fn get_value_ext_by_index(&self, index_param: usize) -> usize {
        match self.list_key.get(index_param) {
            None => 0,
            Some(o) => o.value_ext(),
        }
    }
}
