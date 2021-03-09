//! List of descriptors.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::{Cell, RefCell};
use std::cmp::Ordering::Equal;
use std::rc::Rc;

use super::{CoreManager, ElemDescriptor};
use crate::{ListTrait};

pub struct ListDescriptor {
    /// CoreManager element.
    core_manager: Rc<RefCell<CoreManager>>,

    /// The list of descriptors.
    list_descriptor: Vec<ElemDescriptor>,

    /// The index of the currently selected descriptor element.
    list_index: Cell<usize>,

    /// If true sort when a descriptor is added, otherwise do not sort (for bulk adds).
    sort_on_add: bool,

    /// Updated while sort_on_add was false.
    sort_updated: bool,
}

/// List of descriptors list implementation.

impl ListTrait for ListDescriptor {
    /// Clear all descriptors from the descriptor list.

    fn clear(&mut self) {
        self.list_descriptor.clear();
        self.list_index.set(usize::MAX);
        self.sort_on_add = true;
        self.sort_updated = false;
    }

    /// Get the count of the descriptor list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_descriptor.len()
    }

    /// Get the index of the selected descriptor (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
    }

    /// Select a descriptor based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index value of the descriptor to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_descriptor.len() {
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
        if index_param >= self.list_descriptor.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// List of descriptors implementation.

impl ListDescriptor {
    /// Create a new descriptor list.
    ///
    /// # Arguments
    ///
    /// * `core_manager` - CoreManager element.
    /// * `elem_level_param` - Element level
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        core_manager: &Rc<RefCell<CoreManager>>
    ) -> ListDescriptor {
        ListDescriptor {
            core_manager: Rc::clone(core_manager),
            list_descriptor: Vec::new(),
            list_index: Cell::new(usize::MAX),
            sort_on_add: true,
            sort_updated: false,
        }
    }

    /// Add a new descriptor into the descriptor list.
    /// If the name results in a duplicate entry, an
    /// incrementing number starting from 2 is appended to the
    /// name until a non-duplicate entry is found.
    ///
    /// # Arguments
    ///
    /// * `group_param` - Descriptor group.
    /// * `name_param` - Descriptor name.
    /// * `desc_type_param` - Descriptor type.
    /// * `code_param` - Descriptor code.
    /// * `value_param` - Value parameter.
    /// * `value_expr_param` - Value expression.
    /// * `propagate_param` - Propogate descriptor.
    /// * `updating_json_param` - Updating from json.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.
    #[allow(clippy::too_many_arguments)]

    pub fn add_descriptor(
        &mut self,
        group_param: &str,
        name_param: &str,
        desc_type_param: &str,
        code_param: &str,
        value_param: String,
        value_expr_param: &str,
        propagate_param: bool,
        updating_json_param: bool,
    ) -> bool {
        let mut name: String = String::from(name_param);
        let mut update_element: bool = false;
        if self.get_element_by_name(group_param, name_param, desc_type_param, code_param, false) {
            if updating_json_param {
                self.get_element_by_name(
                    group_param,
                    name_param,
                    desc_type_param,
                    code_param,
                    true,
                );
                update_element = true;
            } else {
                let mut temp_name: String;
                let mut name_index: usize = 2;
                loop {
                    temp_name = format!("{}{}", name_param, name_index);
                    if !self.get_element_by_name(
                        group_param,
                        temp_name.as_str(),
                        desc_type_param,
                        code_param,
                        false,
                    ) {
                        break;
                    }
                    name_index += 1;
                }
                name = temp_name;
            }
        }

        if update_element {
            match self.list_descriptor.get_mut(self.list_index.get()) {
                None => {}
                Some(o) => {
                    o.set_group(group_param);
                    o.set_name(name.as_str());
                    o.set_desc_type(desc_type_param);
                    o.set_code(code_param);
                    o.set_value(value_param.as_str());
                    o.set_value_expr(value_expr_param);
                    o.set_propagate(propagate_param);
                    o.set_list_event_index(usize::MAX);
                    self.set_sort_updated(true);
                }
            }
            return true;
        }
        let new_elem_desc: ElemDescriptor = ElemDescriptor::new(
            group_param,
            name_param,
            desc_type_param,
            code_param,
            value_param.as_str(),
            value_expr_param,
            propagate_param,
        );

        self.list_descriptor.push(new_elem_desc);
        if self.sort_on_add {
            self.sort();
        }

        match self.list_descriptor.iter().position(|e| {
            e.group() == group_param
                && e.name() == name_param
                && e.desc_type() == desc_type_param
                && e.code() == code_param
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }
        if !self.sort_on_add {
            self.sort_updated = true;
        }

        true
    }

    /// Performs a deep copy of this descriptor list and returns and new descriptor list.
    ///
    /// # Arguments
    ///
    /// * `copy_propagate` - Copy only descriptors marked as "propagate"
    ///     (otherwise copy all descriptors).
    /// * `elem_level_param` - Element level
    /// * `updating_json_param` - Updating from json.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(
        &self,
        copy_propagate: bool,
        updating_json_param: bool,
    ) -> ListDescriptor {
        let mut list_descriptor = ListDescriptor::new(&self.core_manager);
        self.copy_list_descriptor(&mut list_descriptor, copy_propagate, updating_json_param);

        list_descriptor
    }

    /// Performs a deep copy of this descriptor list into the descriptor list parameter.
    ///
    /// # Arguments
    ///
    /// * `list_descriptor` - The descriptor list to copy into.
    /// * `copy_propagate` - Copy only descriptors marked as "propagate"
    ///     (otherwise copy all descriptors).
    /// * `updating_json_param` - Updating from json

    pub fn copy_list_descriptor(
        &self,
        list_descriptor: &mut ListDescriptor,
        copy_propagate: bool,
        updating_json_param: bool,
    ) {
        list_descriptor.set_sort_on_add(false);

        for elem in self.list_descriptor.iter() {
            if copy_propagate && !elem.propagate() {
                continue;
            }

            if list_descriptor.get_element_by_name(
                elem.group(),
                elem.name(),
                elem.desc_type(),
                elem.code(),
                false,
            ) {
                continue; // Already present
            }

            list_descriptor.add_descriptor(
                elem.group(),
                elem.name(),
                elem.desc_type(),
                elem.code(),
                elem.value(),
                elem.value_expr().as_str(),
                elem.propagate(),
                updating_json_param,
            );
        }

        list_descriptor.set_sort_on_add(true); // Sorts list
    }

    /// Tests if this descriptor list and another are equal.
    ///
    /// # Arguments
    ///
    /// * `list_descriptor` - List to compare.
    ///
    /// # Return
    ///
    /// * True if equals, otherwise false.

    pub fn equal(&self, list_descriptor: &ListDescriptor) -> bool {
        if self.count() != list_descriptor.count() {
            return false;
        }

        let mut index: usize = 0;
        while index < self.count() {
            match self.list_descriptor.get(index) {
                None => {
                    return false;
                }
                Some(o) => match list_descriptor.list().get(index) {
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

    /// Get the vector of descriptors.
    ///
    /// # Return
    ///
    /// * See description.

    fn list(&self) -> &Vec<ElemDescriptor> {
        &self.list_descriptor
    }

    /// Get the group name of the descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn group(&self) -> &str {
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
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
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
            }
            Some(o) => o.name(),
        }
    }

    /// Get the type of descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn desc_type(&self) -> &str {
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
            }
            Some(o) => o.desc_type(),
        }
    }

    /// Get the code for the type of descriptor.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn code(&self) -> &str {
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
            }
            Some(o) => o.code(),
        }
    }

    /// Get the constant value or the result of an expression.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value(&self) -> String {
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
            }
            Some(o) => o.value(),
        }
    }

    /// Get the optional value expression.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_expr(&self) -> String {
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
            }
            Some(o) => o.value_expr(),
        }
    }

    /// Get the propagate to the next level if applicable.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn propagate(&self) -> bool {
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
            }
            Some(o) => o.propagate(),
        }
    }

    /// Get the index of the event within the event list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_index(&self) -> usize {
        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                panic!("Descriptor list index not set");
            }
            Some(o) => o.list_event_index(),
        }
    }

    /// Select a descriptor based upon a group, name, type, and code.
    ///
    /// # Arguments
    ///
    /// * `group_param` - Group to find.
    /// * `name_param` - Name to find.
    /// * `type_param` - Type to find.
    /// * `code_param` - Code to find.
    /// * `select_param` - Select the found element, otherwise just return result.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_name(
        &self,
        group_param: &str,
        name_param: &str,
        type_param: &str,
        code_param: &str,
        select_param: bool,
    ) -> bool {
        for (index, elem) in self.list_descriptor.iter().enumerate() {
            if group_param == elem.group()
                && name_param == elem.name()
                && type_param == elem.desc_type()
                && code_param == elem.code()
            {
                if select_param {
                    self.set_index(index);
                }
                return true;
            }
        }
        false
    }

    /// Remove the selected descriptor from the descriptor list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_descriptor.len() {
            return false;
        }

        self.list_descriptor.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }

        true
    }

    /// Set the group name of the descriptor.
    /// Duplicate group/name are not allowed.
    ///
    /// # Arguments
    ///
    /// * `group` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_group(&mut self, group: &str) -> bool {
        let name: String;
        let desc_type: String;
        let code: String;

        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                name = String::from(o.name());
                desc_type = String::from(o.desc_type());
                code = String::from(o.code());
            }
        }

        if self.get_element_by_name(
            group,
            name.as_str(),
            desc_type.as_str(),
            code.as_str(),
            false,
        ) {
            return false;
        }

        match self.list_descriptor.get_mut(self.list_index.get()) {
            None => {}
            Some(o) => {
                o.set_group(group);
            }
        }

        if self.sort_on_add {
            self.sort();
        }

        match self.list_descriptor.iter().position(|e| {
            e.group() == group && e.name() == name && e.desc_type() == desc_type && e.code() == code
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add {
            self.set_sort_updated(true);
        }

        true
    }

    /// Set the name of the descriptor.
    /// Duplicate group/name are not allowed.
    ///
    /// # Arguments
    ///
    /// * `name` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_name(&mut self, name: &str) -> bool {
        let group: String;
        let desc_type: String;
        let code: String;

        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                group = String::from(o.group());
                desc_type = String::from(o.desc_type());
                code = String::from(o.code());
            }
        }

        if self.get_element_by_name(
            group.as_str(),
            name,
            desc_type.as_str(),
            code.as_str(),
            false,
        ) {
            return false;
        }

        match self.list_descriptor.get_mut(self.list_index.get()) {
            None => {}
            Some(o) => {
                o.set_name(name);
            }
        }

        if self.sort_on_add {
            self.sort();
        }

        match self.list_descriptor.iter().position(|e| {
            e.group() == group && e.name() == name && e.desc_type() == desc_type && e.code() == code
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add {
            self.set_sort_updated(true);
        }

        true
    }

    /// Set the type of descriptor.
    ///
    /// # Arguments
    ///
    /// * `desc_type` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_type(&mut self, desc_type: &str) -> bool {
        let group: String;
        let name: String;
        let code: String;

        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                group = String::from(o.group());
                name = String::from(o.name());
                code = String::from(o.code());
            }
        }

        if self.get_element_by_name(
            group.as_str(),
            name.as_str(),
            desc_type,
            code.as_str(),
            false,
        ) {
            return false;
        }

        match self.list_descriptor.get_mut(self.list_index.get()) {
            None => {}
            Some(o) => {
                o.set_desc_type(desc_type);
            }
        }

        if self.sort_on_add {
            self.sort();
        }

        match self.list_descriptor.iter().position(|e| {
            e.group() == group && e.name() == name && e.desc_type() == desc_type && e.code() == code
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add {
            self.set_sort_updated(true);
        }

        true
    }

    /// Set the code for the type of descriptor.
    ///
    /// # Arguments
    ///
    /// * `code` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_code(&mut self, code: &str) -> bool {
        let group: String;
        let name: String;
        let desc_type: String;

        match self.list_descriptor.get(self.list_index.get()) {
            None => {
                return false;
            }
            Some(o) => {
                group = String::from(o.group());
                name = String::from(o.name());
                desc_type = String::from(o.desc_type());
            }
        }

        if self.get_element_by_name(
            group.as_str(),
            name.as_str(),
            desc_type.as_str(),
            code,
            false,
        ) {
            return false;
        }

        match self.list_descriptor.get_mut(self.list_index.get()) {
            None => {}
            Some(o) => {
                o.set_code(code);
            }
        }

        if self.sort_on_add {
            self.sort();
        }

        match self.list_descriptor.iter().position(|e| {
            e.group() == group && e.name() == name && e.desc_type() == desc_type && e.code() == code
        }) {
            None => {}
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if self.sort_on_add {
        } else {
            self.set_sort_updated(true);
        }
        true
    }

    /// Set the constant value.
    ///
    /// # Arguments
    ///
    /// * `value` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value(&self, value: &str) -> bool {
        match self.list_descriptor.get(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value(value);
                true
            }
        }
    }

    /// Set the result of an expression.
    ///
    /// # Arguments
    ///
    /// * `value` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_result(&mut self, value: &str) -> bool {
        match self.list_descriptor.get(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value(value);
                true
            }
        }
    }

    /// Set the optional value expression.
    ///
    /// # Arguments
    ///
    /// * `value_expr` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_value_expr(&mut self, value_expr: &str) -> bool {
        match self.list_descriptor.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_value_expr(value_expr);
                true
            }
        }
    }

    /// Set the propagate to the next level if applicable.
    ///
    /// # Arguments
    ///
    /// * `propagate` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_propagate(&mut self, propagate: bool) -> bool {
        match self.list_descriptor.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_propagate(propagate);
                true
            }
        }
    }

    /// Set the index of the event within the event list.
    ///
    /// # Arguments
    ///
    /// * `list_event_index` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_list_event_index(&self, list_event_index: usize) -> bool {
        match self.list_descriptor.get(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_list_event_index(list_event_index);
                true
            }
        }
    }

    /// Determines when the descriptor list is sorted.
    ///
    /// # Arguments
    ///
    /// * `sort_on_add_param` - If true sort when a descriptor is added, otherwise do not sort (for bulk adds).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_sort_on_add(&mut self, sort_on_add_param: bool) -> bool {
        if self.sort_on_add == sort_on_add_param {
            return false;
        }
        self.sort_on_add = sort_on_add_param;
        if self.sort_on_add && self.sort_updated {
            self.sort();

            match self.list_descriptor.get(self.list_index.get()) {
                None => {
                    return false;
                }
                Some(o) => {
                    match self.list_descriptor.iter().position(|e| {
                        e.group() == o.group()
                            && e.name() == o.name()
                            && e.desc_type() == o.desc_type()
                            && e.code() == o.code()
                    }) {
                        None => {}
                        Some(o2) => {
                            self.list_index.set(o2);
                        }
                    }
                }
            }
        }

        self.sort_updated = false;

        true
    }

    /// Set sort updated.
    ///
    /// # Arguments
    ///
    /// * `sort_updated_param` - If true sort updated otherwise false.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_sort_updated(&mut self, sort_updated_param: bool) -> bool {
        if self.sort_updated == sort_updated_param {
            return false;
        }
        self.sort_updated = sort_updated_param;

        true
    }

    /// Sort the descriptor list.

    fn sort(&mut self) {
        self.list_descriptor
            .sort_by(|a, b| ListDescriptor::cmp(a, b));
    }

    /// Sort compare function.
    ///
    /// # Arguments
    ///
    /// * `a` - Event element.
    /// * `b` - Event element.
    ///
    /// # Return
    ///
    /// * Sort order.

    fn cmp(a: &ElemDescriptor, b: &ElemDescriptor) -> std::cmp::Ordering {
        let result = Ord::cmp(a.group(), b.group());
        if result != Equal {
            return result;
        }

        let result = Ord::cmp(a.name(), b.name());
        if result != Equal {
            return result;
        }

        let result = Ord::cmp(a.desc_type(), b.desc_type());
        if result != Equal {
            return result;
        }

        let result = Ord::cmp(a.code(), b.code());
        if result != Equal {
            return result;
        }

        Equal
    }
}
