//! The template event element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::core::ListEvent;

pub struct ElemTemplateEvent {
    /// Name of the template event.
    name: String,
    /// List of events.
    list_event: ListEvent,
    /// Initial template event.
    initial_event: bool,
}

/// The template event element implementation.

impl ElemTemplateEvent {
    /// Create and return a new template event.
    ///
    /// # Arguments
    ///
    /// * `name_param` - Name of template event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(name_param: &str) -> ElemTemplateEvent {
        ElemTemplateEvent {
            name: String::from(name_param),
            list_event: ListEvent::new(false),
            initial_event: false,
        }
    }

    /// Get the name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the list event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event(&self) -> &ListEvent {
        &self.list_event
    }

    /// Get the mutable list event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_mut(&mut self) -> &mut ListEvent {
        &mut self.list_event
    }

    /// Get the initial event.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn initial_event(&self) -> bool {
        self.initial_event
    }

    /// Set the name.
    ///
    /// # Arguments
    ///
    /// * `name_param` - See description.

    pub fn set_name(&mut self, name_param: &str) {
        self.name = String::from(name_param);
    }

    /// Set the list event.
    ///
    /// # Arguments
    ///
    /// * `list_event_param` - See description.

    pub fn set_list_event(&mut self, list_event_param: ListEvent) {
        self.list_event = list_event_param;
    }

    /// Set the initial event.
    ///
    /// # Arguments
    ///
    /// * `initial_event_param` - See description.

    pub fn set_initial_event(&mut self, initial_event_param: bool) {
        self.initial_event = initial_event_param;
    }
}
