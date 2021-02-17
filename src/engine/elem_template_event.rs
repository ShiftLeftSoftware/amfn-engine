//! The template event element definition.
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

use crate::core::{CoreManager, ListEvent};

pub struct ElemTemplateEvent {

  /// CoreManager element. 
  core_manager: Rc<RefCell<CoreManager>>,

  /// Name of the template event. 
  name: String,
  /// List of events. 
  list_event: ListEvent,
  /// Initial template event. 
  initial_event: bool
    
}

/// The template event element implementation.

impl ElemTemplateEvent {

  /// Create and return a new template event.
  /// 
  /// # Arguments
  ///
  /// * `core_manager_param` - CoreManager element.
  /// * `name_param` - Name of template event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn new(core_manager_param: &Rc<RefCell<CoreManager>>, name_param: &str) -> ElemTemplateEvent {
    
    return ElemTemplateEvent {
      core_manager: Rc::clone(core_manager_param),
      name: String::from(name_param),
      list_event: ListEvent::new(core_manager_param, false),
      initial_event: false
    }
  }

  /// Get the name.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn name(self: &Self) -> &str {

    return self.name.as_str();
  }

  /// Get the list event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_event(self: &Self) -> &ListEvent {

    return &self.list_event;
  }

  /// Get the mutable list event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_event_mut(self: &mut Self) -> &mut ListEvent {

    return &mut self.list_event;
  }

  /// Get the initial event.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn initial_event(self: &Self) -> bool {

    return self.initial_event;
  }

  /// Set the name.
  /// 
  /// # Arguments
  ///
  /// * `name_param` - See description.
    
  pub fn set_name(self: &mut Self, name_param: &str) -> () {

    self.name = String::from(name_param);
  }

  /// Set the list event.
  /// 
  /// # Arguments
  ///
  /// * `list_event_param` - See description.
    
  pub fn set_list_event(self: &mut Self, list_event_param: ListEvent) -> () {

    self.list_event = list_event_param;
  }

  /// Set the initial event.
  /// 
  /// # Arguments
  ///
  /// * `initial_event_param` - See description.
    
  pub fn set_initial_event(self: &mut Self, initial_event_param: bool) -> () {

    self.initial_event = initial_event_param;
  }

}