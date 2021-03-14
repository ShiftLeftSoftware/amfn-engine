//! List of parameters.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::{Cell};

use super::{ElemParameter};
use crate::{ListTrait};

pub struct ListParameter {
    /// The list of parameters.
    list_parameter: Vec<ElemParameter>,

    /// The index of the currently selected parameter element.
    list_index: Cell<usize>,
}

/// List of parameters default implementation.

impl Default for ListParameter {
    /// Create a new symbol element.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListParameter::new()
    }
}

/// List of parameters list implementation.

impl ListTrait for ListParameter {
    /// Clear all parameters from the parameter list.

    fn clear(&mut self) {
        self.list_parameter.clear();
        self.list_index.set(usize::MAX);
    }

    /// Get the count of the parameter list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_parameter.len()
    }

    /// Get the index of the selected parameter (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
    }

    /// Select a parameter based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - Index value of the parameter to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_parameter.len() {
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

    fn set_index(&self, index_param: usize) -> bool {
        if index_param >= self.list_parameter.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of parameters implementation.

impl ListParameter {
    /// Create a new parameter list.
    ///
    /// # Arguments
    ///
    /// * `elem_level_param` - Element level
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new() -> ListParameter {
        ListParameter {
            list_parameter: Vec::new(),
            list_index: Cell::new(usize::MAX)
        }
    }

    /// Add a new parameter into the parameter list.
    /// If the name results in a duplicate entry, an
    /// incrementing number starting from 2 is appended to the
    /// name until a non-duplicate entry is found.
    ///
    /// # Arguments
    ///
    /// * `name_param` - Name of the parameter.
    /// * `elem_level_param` - Element level
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn add_parameter(&mut self, name_param: &str, updating_json_param: bool) -> bool {
        let mut name: String = String::from(name_param);
        let mut update_element: bool = false;

        if self.get_element_by_name(name_param, false) {
            // Check for duplicate name
            if updating_json_param {
                self.get_element_by_name(name_param, true);
                update_element = true;
            } else {
                let mut temp_name;
                let mut name_index: usize = 2;
                loop {
                    temp_name = format!("{}{}", name_param, name_index);
                    if !self.get_element_by_name(temp_name.as_str(), false) {
                        break;
                    }
                    name_index += 1;
                }
                name = temp_name;
            }
        }
        if update_element {
            match self.list_parameter.get_mut(self.list_index.get()) {
                None => {
                    return false;
                }
                Some(o) => {
                    o.set_name(name.as_str());
                    return true;
                }
            }
        }
        let new_elem_param: ElemParameter = ElemParameter::new(name.as_str());

        self.list_parameter.push(new_elem_param);

        match self
            .list_parameter
            .iter()
            .position(|e| e.name() == name.as_str())
        {
            None => false,
            Some(o) => {
                self.list_index.set(o);
                true
            }
        }
    }

    /// Performs a deep copy of this parameter list and returns to new parameter list.
    ///
    /// # Arguments
    ///
    /// * `elem_level_param` - Element level
    /// * `updating_json_param` - Updating from json.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(
        &self,
        updating_json_param: bool
    ) -> ListParameter {
        let mut list_parameter = ListParameter::new();

        self.copy_list_parameter(&mut list_parameter, updating_json_param);

        list_parameter
    }

    /// Performs a deep copy of this parameter list into the parameter list parameter.
    ///
    /// # Arguments
    ///
    /// * `list_parameter` - The parameter list to copy into.
    /// * `updating_json_param` - Updating from json.

    pub fn copy_list_parameter(
        &self,
        list_parameter: &mut ListParameter,
        updating_json_param: bool,
    ) {
        for elem in self.list_parameter.iter() {
            if list_parameter.get_element_by_name(elem.name(), false) {
                continue; // Already present
            }

            list_parameter.add_parameter(elem.name(), updating_json_param);

            match elem.param_type() {
                crate::TokenType::Integer => {
                    list_parameter.set_integeri(elem.param_integeri());
                }
                crate::TokenType::Decimal => {
                    list_parameter.set_decimal(elem.param_decimal());
                }
                _ => {
                    list_parameter.set_string(elem.param_string());
                }
            }
        }
    }

    /// Tests if this parameter list and another are equal.
    ///
    /// # Arguments
    ///
    /// * `list_parameter` - List to compare.
    ///
    /// # Return
    ///
    /// * True if equals, otherwise false.

    pub fn equal(&self, list_parameter: &ListParameter) -> bool {
        if self.count() != list_parameter.count() {
            return false;
        }

        let mut index: usize = 0;
        while index < self.count() {
            match self.list_parameter.get(index) {
                None => {
                    return false;
                }
                Some(o) => match list_parameter.list().get(index) {
                    None => {
                        return false;
                    }
                    Some(o2) => {
                        if !o.equal(o2) {
                            return false;
                        }
                    }
                },
            }

            index += 1;
        }

        true
    }

    /// Get the vector of parameters.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list(&self) -> &Vec<ElemParameter> {
        &self.list_parameter
    }

    /// Get the name of the parameter.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        match self.list_parameter.get(self.list_index.get()) {
            None => {
                panic!("Parameter list index not set");
            }
            Some(o) => o.name(),
        }
    }

    /// Get the type of the parameter.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn param_type(&self) -> crate::TokenType {
        match self.list_parameter.get(self.list_index.get()) {
            None => {
                panic!("Parameter list index not set");
            }
            Some(o) => o.param_type(),
        }
    }

    /// Get the integer value of the parameter.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_integeri(&self) -> i32 {
        match self.list_parameter.get(self.list_index.get()) {
            None => {
                panic!("Parameter list index not set");
            }
            Some(o) => o.param_integeri(),
        }
    }

    /// Get the integer value of the parameter.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_integer(&self) -> usize {
        self.param_integeri() as usize
    }

    /// Get the decimal value of the parameter.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_decimal(&self) -> Decimal {
        match self.list_parameter.get(self.list_index.get()) {
            None => {
                panic!("Parameter list index not set");
            }
            Some(o) => o.param_decimal(),
        }
    }

    /// Get the string value of the parameter.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn param_string(&self) -> &str {
        match self.list_parameter.get(self.list_index.get()) {
            None => {
                panic!("Parameter list index not set");
            }
            Some(o) => o.param_string(),
        }
    }

    /// Select a parameter based upon a name.
    ///
    /// # Arguments
    ///
    /// * `name_param` - The name of the parameter to select.
    /// * `select_param` - If true select element, otherwise restore current element.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_name(&self, name_param: &str, select_param: bool) -> bool {
        for (index, elem) in self.list_parameter.iter().enumerate() {
            if name_param == elem.name() {
                if select_param {
                    self.set_index(index);
                }
                return true;
            }
        }

        false
    }

    /// Move the selected parameter up or down in the parameter list.
    ///
    /// # Arguments
    ///
    /// * `is_up` - Move up, otherwise down.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn move_param(&mut self, is_up: bool) -> bool {
        let name: String;
        match self.list_parameter.get_mut(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                name = String::from(o.name());
            }
        }

        if is_up {
            if self.list_index.get() == 0 {
                return false;
            }
            let elem = self.list_parameter.remove(self.list_index.get());
            self.list_parameter.insert(self.list_index.get() - 1, elem);
        } else {
            if self.list_index.get() + 1 >= self.list_parameter.len() {
                return false;
            }
            let elem = self.list_parameter.remove(self.list_index.get());
            self.list_parameter.insert(self.list_index.get() + 1, elem);
        }

        match self.list_parameter.iter().position(|e| e.name() == name) {
            None => false,
            Some(o) => {
                self.list_index.set(o);
                true
            }
        }
    }

    /// Remove the selected parameter from the parameter list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_parameter.len() {
            return false;
        }

        self.list_parameter.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }

        true
    }

    /// Set the name of the parameter.
    /// Duplicate names are not allowed.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_name(&mut self, name_param: &str) -> bool {
        if self.get_element_by_name(name_param, false) {
            return false;
        }

        match self.list_parameter.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_name(name_param);
                true
            }
        }
    }

    /// Set the type of parameter.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_type(&mut self, value_param: crate::TokenType) -> bool {
        match self.list_parameter.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_type(value_param);
                true
            }
        }
    }

    /// Set the integer value.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_integeri(&mut self, value_param: i32) -> bool {
        match self.list_parameter.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_integeri(value_param);
                true
            }
        }
    }

    /// Set the integer value.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_integer(&mut self, value_param: usize) -> bool {
        self.set_integeri(value_param as i32)
    }

    /// Set the decimal value.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_decimal(&mut self, value_param: Decimal) -> bool {
        match self.list_parameter.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_decimal(value_param);
                true
            }
        }
    }

    /// Set the string value.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_string(&mut self, value_param: &str) -> bool {
        match self.list_parameter.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_string(value_param);
                true
            }
        }
    }
}
