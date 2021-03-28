//! List of template event definitions.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;
use std::cmp::Ordering::Equal;

use super::ElemTemplateEvent;
use crate::core::ListEvent;
use crate::ListTrait;

pub struct ListTemplateEvent {
    /// The list of template events.
    list_template_event: Vec<ElemTemplateEvent>,

    /// The index of the currently selected template event element.
    list_index: Cell<usize>,

    /// If true sort when an template event is added, otherwise do not sort (for bulk adds).
    sort_on_add: bool,

    /// Updated while sort_on_add was false.
    sort_updated: bool,
}

/// List of template events default implementation.

impl Default for ListTemplateEvent {
    /// Create a new symbol element.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        ListTemplateEvent::new()
    }
}

/// List of template events list implementation.

impl ListTrait for ListTemplateEvent {
    /// Clear all template events from the template event list.

    fn clear(&mut self) {
        self.list_template_event.clear();
        self.list_index.set(usize::MAX);
        self.sort_on_add = true;
        self.sort_updated = false;
    }

    /// Get the count of the template event list.
    ///
    /// # Return
    ///
    /// * See description.

    fn count(&self) -> usize {
        self.list_template_event.len()
    }

    /// Get the index of the selected template event (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.

    fn index(&self) -> usize {
        self.list_index.get()
    }

    /// Select an template event based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index value of the template event to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool {
        if index_param >= self.list_template_event.len() {
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
        if index_param >= self.list_template_event.len() {
            return false;
        }

        self.list_index.set(index_param);

        true
    }
}

/// Implementation for the list of template event definitions.

impl ListTemplateEvent {
    /// Create and return a new list template event elements.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new() -> ListTemplateEvent {
        ListTemplateEvent {
            list_template_event: Vec::new(),
            list_index: Cell::new(usize::MAX),
            sort_on_add: true,
            sort_updated: false,
        }
    }

    /// Add a new template event into the template event list.
    /// If the group and name results in a duplicate entry, an
    /// incrementing number starting from 2 is appended to the
    /// name until a non-duplicate entry is found.
    ///
    /// # Arguments
    ///
    /// * `name_param` - Name of the template event.
    /// * `initial` - Initial template event.
    /// * `list_event` - The list of events (or None).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn add_template_event(
        &mut self,
        name_param: &str,
        initial: bool,
        mut list_event: Option<ListEvent>,
    ) -> bool {
        let mut name = String::from(name_param);

        if self.get_element_by_name(name.as_str(), false) {
            // Check for duplicate name
            let mut temp_name: String;
            let mut name_index: usize = 2;

            loop {
                temp_name = format!("{}{}", name, name_index);

                if !self.get_element_by_name(temp_name.as_str(), false) {
                    break;
                }

                name_index += 1;
            }

            name = temp_name;
        }
        let mut new_elem_template_event = ElemTemplateEvent::new(name.as_str());
        if list_event.is_none() {
            list_event = Option::from(ListEvent::new(false));
        }

        new_elem_template_event.set_name(name.as_str());
        new_elem_template_event.set_initial_event(initial);

        match list_event {
            None => {
                return false;
            }
            Some(o) => {
                new_elem_template_event.set_list_event(o);
            }
        }

        self.list_template_event.push(new_elem_template_event);
        if self.sort_on_add {
            self.sort();
        }

        match self
            .list_template_event
            .iter()
            .position(|e| e.name() == name)
        {
            None => {
                return false;
            }
            Some(o) => {
                self.list_index.set(o);
            }
        }
        if !self.sort_on_add {
            self.sort_updated = true;
        }
        true
    }

    /// Performs a deep copy of the template event list and
    /// returns a new template event list.
    ///
    /// # Arguments
    ///
    /// * `updating_json` - Updating from Json.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn copy(&self, updating_json: bool) -> ListTemplateEvent {
        let mut list_template_event = ListTemplateEvent::new();
        list_template_event.set_sort_on_add(false);

        for elem in self.list_template_event.iter() {
            let new_list_event = elem.list_event().copy(updating_json);
            list_template_event.add_template_event(
                elem.name(),
                elem.initial_event(),
                Option::from(new_list_event),
            );
        }
        list_template_event.set_sort_on_add(true); // Sorts list

        list_template_event
    }

    /// Get the name of the template event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        match self.list_template_event.get(self.list_index.get()) {
            None => {
                panic!("Template event list index not set");
            }
            Some(o) => o.name(),
        }
    }

    /// Get the initial template event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn initial_event(&self) -> bool {
        match self.list_template_event.get(self.list_index.get()) {
            None => {
                panic!("Template event list index not set");
            }
            Some(o) => o.initial_event(),
        }
    }

    /// Get the list of events.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event(&self) -> &ListEvent {
        match self.list_template_event.get(self.list_index.get()) {
            None => {
                panic!("Template event list index not set");
            }
            Some(o) => o.list_event(),
        }
    }

    /// Get the mut list of events.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_mut(&mut self) -> &mut ListEvent {
        match self.list_template_event.get_mut(self.list_index.get()) {
            None => {
                panic!("Template event list index not set");
            }
            Some(o) => o.list_event_mut(),
        }
    }

    /// Select an template event based upon a group and name.
    ///
    /// # Arguments
    ///
    /// * `name_param` - The name of the template event to select.
    /// * `select_param` - If true select element, otherwise restore current element.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn get_element_by_name(&self, name_param: &str, select_param: bool) -> bool {
        for (index, elem) in self.list_template_event.iter().enumerate() {
            if name_param == elem.name() {
                if select_param {
                    self.set_index(index);
                }
                return true;
            }
        }
        false
    }

    /// Remove the selected template event from the template event list.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn remove(&mut self) -> bool {
        if self.list_index.get() >= self.list_template_event.len() {
            return false;
        }

        self.list_template_event.remove(self.list_index.get());
        if self.list_index.get() > 0 {
            self.list_index.set(self.list_index.get() - 1);
        }

        true
    }

    /// Set the name of the template event.
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

        match self.list_template_event.get_mut(self.list_index.get()) {
            None => {
                panic!("Template event list index not set");
            }
            Some(o) => {
                o.set_name(name_param);
            }
        }

        if self.sort_on_add {
            self.sort();
        }

        match self
            .list_template_event
            .iter()
            .position(|e| e.name() == name_param)
        {
            None => {
                return false;
            }
            Some(o) => {
                self.list_index.set(o);
            }
        }

        if !self.sort_on_add {
            self.sort_updated = true;
        }

        true
    }

    /// Set the initial template event.
    ///
    /// # Arguments
    ///
    /// * `initial_event_param` - See description.
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_initial_event(&mut self, initial_event_param: bool) -> bool {
        match self.list_template_event.get_mut(self.list_index.get()) {
            None => false,
            Some(o) => {
                o.set_initial_event(initial_event_param);
                true
            }
        }
    }

    /// Determines when the template event list is sorted.
    ///
    /// # Arguments
    ///
    /// * `sort_on_add_param` - If true sort when an template event is added,
    ///     otherwise do not sort (for bulk adds).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    pub fn set_sort_on_add(&mut self, sort_on_add_param: bool) -> bool {
        if sort_on_add_param == self.sort_on_add {
            return false;
        }
        self.sort_on_add = sort_on_add_param;
        if sort_on_add_param && self.sort_updated {
            self.sort();

            match self.list_template_event.get(self.list_index.get()) {
                None => {
                    return false;
                }
                Some(o) => {
                    match self
                        .list_template_event
                        .iter()
                        .position(|e| e.name() == o.name())
                    {
                        None => {
                            return false;
                        }
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

    /// Sort the template event list.

    fn sort(&mut self) {
        self.list_template_event
            .sort_by(|a, b| ListTemplateEvent::cmp(a, b));
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

    fn cmp(a: &ElemTemplateEvent, b: &ElemTemplateEvent) -> std::cmp::Ordering {
        let result = Ord::cmp(a.name(), b.name());
        if result != Equal {
            return result;
        }

        Equal
    }
}
