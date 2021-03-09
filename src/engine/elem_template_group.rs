//! The template event group element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::cell::RefCell;
use std::rc::Rc;

use super::{ElemPreferences, ListTemplateEvent};
use crate::core::CoreManager;

pub struct ElemTemplateGroup {
  /// CoreManager element.
  core_manager: Rc<RefCell<CoreManager>>,

  /// Group name of the template group.
  group: String,
  /// Preferences element.
  elem_preferences: ElemPreferences,
  /// List of template events.
  list_template_event: ListTemplateEvent,
}

/// The template event group element implementation.

impl ElemTemplateGroup {
  /// Create and return a new template group.
  ///
  /// # Arguments
  ///
  /// * `core_manager_param` - CoreManager element.
  /// * `group_param` - Name of template group.
  /// * `preferences_param` - Group preferences.
  ///
  /// # Return
  ///
  /// * See description.

  pub fn new(
    core_manager_param: &Rc<RefCell<CoreManager>>,
    group_param: &str,
    preferences_param: ElemPreferences,
  ) -> ElemTemplateGroup {
    ElemTemplateGroup {
      core_manager: Rc::clone(core_manager_param),
      group: String::from(group_param),
      elem_preferences: preferences_param,
      list_template_event: ListTemplateEvent::new(core_manager_param),
    }
  }

  /// Copy this template group and return a new template group.
  ///
  /// # Arguments
  ///
  /// * `updating_json` - Updating from Json.
  ///
  /// # Return
  ///
  /// * See description.

  pub fn copy(&self, updating_json: bool) -> ElemTemplateGroup {
    let mut template_group = ElemTemplateGroup::new(
      &self.core_manager,
      self.group.as_str(),
      self
        .elem_preferences
        .copy(updating_json),
    );

    let list_template_event = self.list_template_event.copy(updating_json);

    template_group.set_list_event(list_template_event);

    template_group
  }

  /// Get the group.
  ///
  /// # Return
  ///
  /// * See description.

  pub fn group(&self) -> &str {
    self.group.as_str()
  }

  /// Get the preferences.
  ///
  /// # Return
  ///
  /// * See description.

  pub fn preferences(&self) -> &ElemPreferences {
    &self.elem_preferences
  }

  /// Get the mut preferences.
  ///
  /// # Return
  ///
  /// * See description.

  pub fn preferences_mut(&mut self) -> &mut ElemPreferences {
    &mut self.elem_preferences
  }

  /// Get the list event.
  ///
  /// # Return
  ///
  /// * See description.

  pub fn list_template_event(&self) -> &ListTemplateEvent {
    &self.list_template_event
  }

  /// Get the mut list event.
  ///
  /// # Return
  ///
  /// * See description.

  pub fn list_template_event_mut(&mut self) -> &mut ListTemplateEvent {
    &mut self.list_template_event
  }

  /// Set the group.
  ///
  /// # Arguments
  ///
  /// * `group_param` - See description.

  pub fn set_group(&mut self, group_param: &str) {
    self.group = String::from(group_param);
  }

  /// Set the preferences.
  ///
  /// # Arguments
  ///
  /// * `preferences_param` - See description.

  pub fn set_preferences(&mut self, preferences_param: ElemPreferences) {
    self.elem_preferences = preferences_param;
  }

  /// Set the list event.
  ///
  /// # Arguments
  ///
  /// * `list_event_param` - See description.

  pub fn set_list_event(&mut self, list_event_param: ListTemplateEvent) {
    self.list_template_event = list_event_param;
  }
}
