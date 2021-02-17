//! The template event group element definition.
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
 
use std::rc::Rc;
use std::cell::RefCell;

use crate::ElemLevelType;
use crate::core::CoreManager;
use super::{ElemPreferences, ListTemplateEvent};

pub struct ElemTemplateGroup {

  /// CoreManager element. 
  core_manager: Rc<RefCell<CoreManager>>,

  /// Group name of the template group. 
  group: String,
  /// Preferences element. 
  elem_preferences: ElemPreferences,
  /// List of template events. 
  list_template_event: ListTemplateEvent
    
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

  pub fn new(core_manager_param: &Rc<RefCell<CoreManager>>, group_param: &str, 
    preferences_param: ElemPreferences) -> ElemTemplateGroup {
    
    return ElemTemplateGroup {
      core_manager: Rc::clone(core_manager_param),
      group: String::from(group_param),
      elem_preferences: preferences_param,
      list_template_event: ListTemplateEvent::new(core_manager_param)
    };
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

  pub fn copy(self: &Self, updating_json: bool) -> ElemTemplateGroup {

    let mut template_group = ElemTemplateGroup::new(&self.core_manager, self.group.as_str(), 
      self.elem_preferences.copy(ElemLevelType::Cashflow, updating_json));

    let list_template_event = self.list_template_event.copy(updating_json);

    template_group.set_list_event(list_template_event);

    return template_group;
  }

  /// Get the group.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn group(self: &Self) -> &str {

    return self.group.as_str();
  }

  /// Get the preferences.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn preferences(self: &Self) -> &ElemPreferences {

    return &self.elem_preferences;
  }

  /// Get the mut preferences.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn preferences_mut(self: &mut Self) -> &mut ElemPreferences {

    return &mut self.elem_preferences;
  }

  /// Get the list event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_template_event(self: &Self) -> &ListTemplateEvent {

    return &self.list_template_event;
  }

  /// Get the mut list event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_template_event_mut(self: &mut Self) -> &mut ListTemplateEvent {

    return &mut self.list_template_event;
  }

  /// Set the group.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - See description.
    
  pub fn set_group(self: &mut Self, group_param: &str) -> () {

    self.group = String::from(group_param);
  }

  /// Set the preferences.
  /// 
  /// # Arguments
  ///
  /// * `preferences_param` - See description.
    
  pub fn set_preferences(self: &mut Self, preferences_param: ElemPreferences) -> () {

    self.elem_preferences = preferences_param;
  }

  /// Set the list event.
  /// 
  /// # Arguments
  ///
  /// * `list_event_param` - See description.
    
  pub fn set_list_event(self: &mut Self, list_event_param: ListTemplateEvent) -> () {

    self.list_template_event = list_event_param;
  }

}