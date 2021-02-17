//! List of template event definitions.
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
use std::cell::{Cell, RefCell};
use std::cmp::Ordering::Equal;

use crate::{ListTrait,ElemUpdateType, ElemLevelType};
use crate::core::{CoreManager, CoreUtility, ListEvent};
use super::ElemTemplateEvent;

pub struct ListTemplateEvent {

  /// CoreManager element. 
  core_manager: Rc<RefCell<CoreManager>>,

  /// The list of template events. 
  list_template_event: Vec<ElemTemplateEvent>,

  /// The index of the currently selected template event element. 
  list_index: Cell<usize>,

  /// If true sort when an template event is added, otherwise do not sort (for bulk adds). 
  sort_on_add: bool,

  /// Updated while sort_on_add was false. 
  sort_updated: bool

}

/// List of template events list implementation.

impl ListTrait for ListTemplateEvent {

  /// Clear all template events from the template event list.

  fn clear(self: &mut Self) -> () {
    
    self.list_template_event.clear();
    self.list_index.set(usize::MAX);
    self.sort_on_add = true;
    self.sort_updated = false;

    self.set_updated();
  }

  /// Get the count of the template event list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_template_event.len();
  }

  /// Get the index of the selected template event (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
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

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_template_event.len() {
      return false;
    }

    self.set_index(index_param);

    return true;
  }

  fn set_index(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_template_event.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true;
  }

}

/// Implementation for the list of template event definitions.

impl ListTemplateEvent {

  /// Create and return a new list template event elements.
  /// 
  /// # Arguments
  ///
  /// * `core_manager_param` - CoreManager element.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new(core_manager_param: &Rc<RefCell<CoreManager>>) -> ListTemplateEvent {
    
    return ListTemplateEvent {
      core_manager: Rc::clone(core_manager_param),
      list_template_event: Vec::new(),
      list_index: Cell::new(usize::MAX),
      sort_on_add: true,
      sort_updated: false 
    };
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
    self: &mut Self, name_param: &str, initial: bool, mut list_event: Option<ListEvent>) -> bool {
    
    let mut name = String::from(name_param);

    if self.get_element_by_name(name.as_str(), false) { // Check for duplicate name
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
    
    let mut new_elem_template_event = ElemTemplateEvent::new(&self.core_manager, name.as_str());
    
    if list_event.is_none() {
      list_event = Option::from(ListEvent::new(&self.core_manager, false));
    }

    new_elem_template_event.set_name(name.as_str());
    new_elem_template_event.set_initial_event(initial);

    match list_event {
      None => { return false; }
      Some(o) => { new_elem_template_event.set_list_event(o); }
    }

    self.list_template_event.push(new_elem_template_event);
    
    if self.sort_on_add {
      self.sort();
    }

    match self.list_template_event.iter().position(|e| e.name() == name) {
      None => { return false; }
      Some(o) => { self.list_index.set(o); }
    }
    
    if self.sort_on_add {
      self.set_updated();
    } else {
      self.sort_updated = true;
    }
    
    return true;
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

  pub fn copy(self: &Self, updating_json: bool) -> ListTemplateEvent {    
    let mut list_template_event = ListTemplateEvent::new(&self.core_manager);
        
    list_template_event.set_sort_on_add(false);

    for elem in self.list_template_event.iter() {
      let new_list_event = elem.list_event().copy(updating_json);
      list_template_event.add_template_event(
        elem.name(), elem.initial_event(), Option::from(new_list_event));
    }
    
    list_template_event.set_sort_on_add(true); // Sorts list

    return list_template_event;
  }

  /// Get the name of the template event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    match self.list_template_event.iter().nth(self.list_index.get()) {
      None => { panic!("Template event list index not set"); }
      Some(o) => { return o.name(); }
    }
  }

  /// Get the initial template event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn initial_event(self: &Self) -> bool {

    match self.list_template_event.iter().nth(self.list_index.get()) {
      None => { panic!("Template event list index not set"); }
      Some(o) => { return o.initial_event(); }
    }
  }

  /// Get the list of events.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_event(self: &Self) -> &ListEvent {

    match self.list_template_event.iter().nth(self.list_index.get()) {
      None => { panic!("Template event list index not set"); }
      Some(o) => { return o.list_event(); }
    }
  }

  /// Get the mut list of events.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_event_mut(self: &mut Self) -> &mut ListEvent {

    match self.list_template_event.iter_mut().nth(self.list_index.get()) {
      None => { panic!("Template event list index not set"); }
      Some(o) => { return o.list_event_mut(); }
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
  
  pub fn get_element_by_name(self: &Self, name_param: &str, select_param: bool) -> bool {
    let mut index: usize = 0;

    for elem in self.list_template_event.iter() {
      if name_param == elem.name() {
        if select_param {
          self.set_index(index);
        }
        return true;
      }      

      index += 1;
    }
    
    return false;
  }

  /// Remove the selected template event from the template event list.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn remove(self: &mut Self) -> bool {
    if self.list_index.get() >= self.list_template_event.len() {
      return false;
    }

    self.list_template_event.remove(self.list_index.get());
    
    if self.list_index.get() > 0 {
      self.list_index.set(self.list_index.get() - 1);
    }
    
    self.set_updated();
    
    return true;
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

  pub fn set_name(self: &mut Self, name_param: &str) -> bool {

    if self.get_element_by_name(name_param, false) {
      return false;
    }

    match self.list_template_event.iter_mut().nth(self.list_index.get()) {
      None => { panic!("Template event list index not set"); }
      Some(o) => { o.set_name(name_param); }  
    }

    if self.sort_on_add {
      self.sort();
    }

    match self.list_template_event.iter().position(|e| e.name() == name_param) {
      None => { return false; }
      Some(o) => { self.list_index.set(o); }
    }

    if self.sort_on_add {
      self.set_updated();
    } else {
      self.sort_updated = true;
    }

    return true;
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

  pub fn set_initial_event(self: &mut Self, initial_event_param: bool) -> bool {

    match self.list_template_event.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => {
        o.set_initial_event(initial_event_param);
        self.set_updated();
        return true;
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
  
  pub fn set_sort_on_add(self: &mut Self, sort_on_add_param: bool) -> bool {
    
    if sort_on_add_param == self.sort_on_add {
      return false;
    }
    
    self.sort_on_add = sort_on_add_param;
    
    if sort_on_add_param && self.sort_updated {
      self.sort();

      match self.list_template_event.iter().nth(self.list_index.get()) {
        None => { return false; }
        Some(o) => { 
          match self.list_template_event.iter().position(|e| e.name() == o.name()) {
            None => { return false; }
            Some(o2) => {
              self.list_index.set(o2);
              self.set_updated();
            }
          }
        }
      }
    }
    
    self.sort_updated = false;
    
    return true;
  }
  
  /// Call the updated signal.
  
  fn set_updated(self: &Self) -> () {

    self.core_manager.borrow().notify(
      CoreUtility::format_update(ElemUpdateType::Template, ElemLevelType::Cashflow));
  }

  /// Sort the template event list.
  
  fn sort(self: &mut Self) -> () {
    
    self.list_template_event.sort_by(|a, b| ListTemplateEvent::cmp(a, b));
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

    return Equal;
  }

}