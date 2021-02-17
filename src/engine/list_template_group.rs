//! List of template groups.
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
use std::cell::{Cell, Ref, RefMut, RefCell};
use std::cmp::Ordering::Equal;

use crate::{ListTrait, ElemUpdateType, ElemLevelType};
use crate::core::{CoreUtility};
use super::{CalcManager, CalcExpression, CalcUtility, ElemPreferences, ElemTemplateGroup, ListTemplateEvent};

pub struct ListTemplateGroup {

  /// Calculator manager element. 
  calc_manager: Option<Rc<RefCell<CalcManager>>>,

  /// The list of template groups. 
  list_template_group: Vec<ElemTemplateGroup>,

  /// The index of the currently selected template group element. 
  list_index: Cell<usize>,

  /// If true sort when an template event is added, otherwise do not sort (for bulk adds). 
  sort_on_add: Cell<bool>,

  /// Updated while sort_on_add was false. 
  sort_updated: Cell<bool>

}

/// List of template groups list implementation.

impl ListTrait for ListTemplateGroup {

  /// Clear all template groups from the template group list.

  fn clear(self: &mut Self) -> () {
    
    self.list_template_group.clear();
    self.list_index.set(usize::MAX);
    self.sort_on_add = Cell::new(true);
    self.sort_updated = Cell::new(false);

    self.set_updated();
  }

  /// Get the count of the template group list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize {
    
    return self.list_template_group.len();
  }

  /// Get the index of the selected template group (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize {
    
    return self.list_index.get();
  }

  /// Select a template group based upon an index value.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - The index value of the template group to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_template_group.len() {
      return false;
    }

    self.set_index(index_param);

    return true;
  }

  fn set_index(self: &Self, index_param: usize) -> bool {

    if index_param >= self.list_template_group.len() {
      return false;
    }

    self.list_index.set(index_param);

    return true
  }

}

/// Implementation for the list of template groups.

impl ListTemplateGroup {

  /// Create and return a new list template group.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new() -> ListTemplateGroup {
    
    return ListTemplateGroup {
      calc_manager: None,
      list_template_group: Vec::new(),
      list_index: Cell::new(usize::MAX),
      sort_on_add: Cell::new(true),
      sort_updated: Cell::new(false)
    };

  }

  /// Returns the calculation manager element.
  /// 
  /// # Return
  ///
  /// * See description.

  fn calc_manager(self: &Self) -> &Rc<RefCell<CalcManager>> {

    match self.calc_manager.as_ref() {
      None => { panic!("Missing calc manager"); }
      Some(o) => { return o; }
    }
  }

  /// Returns the calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.

  fn calc_reg(self: &Self) -> Ref<CalcManager> {

    match self.calc_manager.as_ref() {
      None => { panic!("Missing calc manager"); }
      Some(o) => { return o.borrow(); }
    }
  }

  /// Returns the matable calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.

  fn calc_reg_mut(self: &Self) -> RefMut<CalcManager> {

    match self.calc_manager.as_ref() {
      None => { panic!("Missing calc manager"); }
      Some(o) => { return o.borrow_mut(); }
    }
  }

  /// Set the calculation manager.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.

  pub fn set_calc_reg(self: &mut Self, calc_manager_param: &Rc<RefCell<CalcManager>>) -> () {

    self.calc_manager = Option::from(Rc::clone(calc_manager_param));
  }

  /// Add a new template group into the template group list.
  /// If the group name results in a duplicate entry, an
  /// incrementing number starting from 2 is appended to the
  /// group until a non-duplicate entry is found.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - Group name of the template group.
  /// 
  /// # Return
  ///
  /// * ERROR_NONE if successful, otherwise error code.
  
  pub fn add_template_group(self: &Self, group_param: &str) -> Result<ElemTemplateGroup, crate::ErrorType> {
      
    let prefs = self.calc_reg().preferences().copy(ElemLevelType::Cashflow, self.calc_reg().updating_json());

    return self.create_template_group(group_param, Option::from(prefs), true);
  }

  /// Add a new template group into the template group list.
  /// If the group name results in a duplicate entry, an
  /// incrementing number starting from 2 is appended to the
  /// group until a non-duplicate entry is found.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - Group name of the template group.
  /// * `elem_preferences_orig_opt` - Original preferences element to copy (or None).
  /// * `copy_propagate` - Copy only descriptors marked as "propagate"
  ///     (otherwise copy all descriptors).
  ///
  /// # Return
  ///
  /// * ERROR_NONE if successful, otherwise error code.
  
  pub fn create_template_group(self: &Self, group_param: &str, 
      elem_preferences_orig_opt: Option<ElemPreferences>, 
      copy_propagate: bool) -> Result<ElemTemplateGroup, crate::ErrorType> {

    let mut group = String::from(group_param);
    let mut update_element: bool = false;
    let updating_json = self.calc_reg().updating_json();
    
    if self.get_element_by_group(group_param, false) { // Check for duplicate name
      if updating_json {
        self.get_element_by_group(group_param, true);
        update_element = true;
      } else {
        let mut temp_group: String;
        let mut group_index: usize = 2;

        loop {
          temp_group = format!("{}{}", group_param, group_index);

          if !self.get_element_by_group(temp_group.as_str(), false) {
            break;
          }

          group_index += 1;
        }

        group = String::from(temp_group.as_str());
      }
    }
    
    let elem_preferences: ElemPreferences;
    match elem_preferences_orig_opt {
      None => { 
        elem_preferences = ElemPreferences::new(
          self.calc_manager(), "", "", "", "", 0, crate::DEFAULT_DECIMAL_DIGITS, 
          -1, -1, -1, None, None, false, ElemLevelType::Cashflow, updating_json);
        }
      Some(o) => {
        elem_preferences = ElemPreferences::new(
          self.calc_manager(), o.locale_str(), o.cross_rate_code(), o.default_encoding(), o.group(),
          o.fiscal_year_start(), o.decimal_digits(), o.combine_principal(), o.compress_descriptor(), 
          o.statistic_events(), Option::from(o.list_parameter()), Option::from(o.list_descriptor()),
          copy_propagate, ElemLevelType::Cashflow, updating_json);
      }
    }

    let new_elem_template_group;
    if update_element {
      match self.list_template_group.iter().nth(self.list_index.get()) {
        None => { 
          new_elem_template_group = ElemTemplateGroup::new(
            self.calc_reg().core_manager(), group.as_str(), elem_preferences); 
        }
        Some(o) => { new_elem_template_group = o.copy(updating_json); }
      }
    } else {
      new_elem_template_group = ElemTemplateGroup::new(
        self.calc_reg().core_manager(), group.as_str(), elem_preferences);
    }
    
    if update_element {
      self.sort_updated.set(true);
    }
    
    return Ok(new_elem_template_group);
  }

  /// Performs a deep copy of the template group list and 
  /// returns a new template group list.
  /// 
  /// # Arguments
  ///
  /// * `calc_manager_param` - Calculation manager.
  /// 
  /// # Return
  ///
  /// * See description.    

  pub fn copy_with_calc_manager(
    self: &Self, calc_manager_param: &Rc<RefCell<CalcManager>>) -> ListTemplateGroup {

    let mut list_template_group = ListTemplateGroup::new();        
    let mut index: usize = 0;

    list_template_group.set_calc_reg(calc_manager_param);
    
    list_template_group.set_sort_on_add(false);
    
    loop {

      if !self.get_element(index) { break; }

      let template_group = self.copy_selected();

      list_template_group.list_mut().push(template_group);

      index += 1;
    }
    
    list_template_group.set_sort_on_add(true);

    return list_template_group;
  }

  /// Performs a deep copy of the template group element and 
  /// returns a new template group element.
  /// 
  /// # Return
  ///
  /// * ERROR_NONE if successful, otherwise error code.

  pub fn copy_selected(self: &Self) -> ElemTemplateGroup {
    let updating_json = self.calc_reg().updating_json();
    let group = String::from(self.group());
    let prefs = Option::from(self.preferences().copy(ElemLevelType::Cashflow, updating_json));

    let result = self.create_template_group(group.as_str(), prefs, false);      
    match result {
      Err(_e) => { 
        return ElemTemplateGroup::new(self.calc_reg().core_manager(), 
          self.group(), self.preferences().copy(ElemLevelType::Cashflow, updating_json)); 
      }
      Ok(o) => { 
        return o;
      }
    }
  }

  /// Evaluate all of the descriptors in the template group.
  /// For each descriptor that specifies an expression,
  /// execute the expression using the list of parameters.
  
  pub fn evaluate_descriptors(self: &Self) -> () {
    
    match self.list_template_group.iter().nth(self.list_index.get()) {
      None => { }
      Some(o2) => {
        let calc_expression = CalcExpression::new(self.calc_manager(), 
          self.calc_reg().fiscal_year_start(false), self.calc_reg().decimal_digits(false));

        let preferences = o2.preferences();
        let expression = RefCell::new(calc_expression);
    
        CalcUtility::evaluate_descriptors(self.calc_manager(), 
          &expression, preferences.list_parameter(), preferences.list_descriptor());    
      }
    }
  }

  /// Get the list of cashflows.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn list(self: &Self) -> &Vec<ElemTemplateGroup> {
    
    return &self.list_template_group;
  }

  /// Get the mut list of cashflows.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn list_mut(self: &mut Self) -> &mut Vec<ElemTemplateGroup> {
    
    return &mut self.list_template_group;
  }

  /// Get the group name of the template group.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn group(self: &Self) -> &str {

    match self.list_template_group.iter().nth(self.list_index.get()) {
      None => { return ""; }
      Some(o) => { return o.group(); }
    }
  }

  /// Get the preferences element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn preferences(self: &Self) -> &ElemPreferences {

    match self.list_template_group.iter().nth(self.list_index.get()) {
      None => { panic!("Template group list index not set"); }
      Some(o) => { return o.preferences(); }
    }
  }

  /// Get the mutable preferences element.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn preferences_mut(self: &mut Self) -> &mut ElemPreferences {

    match self.list_template_group.iter_mut().nth(self.list_index.get()) {
      None => { panic!("Template group list index not set"); }
      Some(o) => { return o.preferences_mut(); }
    }
  }

  /// Get the list of template events.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_template_event(self: &Self) -> &ListTemplateEvent {

    match self.list_template_group.iter().nth(self.list_index.get()) {
      None => { panic!("Template group list index not set"); }
      Some(o) => { return o.list_template_event(); }
    }
  }

  /// Get the mutable list of template events.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn list_template_event_mut(self: &mut Self) -> &mut ListTemplateEvent {

    match self.list_template_group.iter_mut().nth(self.list_index.get()) {
      None => { panic!("Template group list index not set"); }
      Some(o) => { return o.list_template_event_mut(); }
    }
  }

  /// Retrieve when the template group list is sorted.
  /// 
  /// # Return
  ///
  /// * See description.

  pub fn sort_on_add(self: &mut Self) -> bool {

    return self.sort_on_add.get();
  }

  /// Select a template group based upon a group name.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - The group name of the template group to select.
  /// * `select_param` - If true select element, otherwise restore current element.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.
  
  pub fn get_element_by_group(self: &Self, group_param: &str, select_param: bool) -> bool {
    let mut index: usize = 0;

    for elem in self.list_template_group.iter() {
      if group_param == elem.group() {
        if select_param {
          self.set_index(index);
        }
        return true;
      }      

      index += 1;
    }
    
    return false;
  }

  /// Remove the selected template group from the template group list.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn remove(self: &mut Self) -> bool {
    if self.list_index.get() >= self.list_template_group.len() {
      return false;
    }

    self.list_template_group.remove(self.list_index.get());
    
    if self.list_index.get() > 0 {
      self.list_index.set(self.list_index.get() - 1);
    }
    
    self.set_updated();
    
    return true;
  }

  /// Set the group name of the template group.
  /// 
  /// # Arguments
  ///
  /// * `group_param` - See description.
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn set_group(self: &mut Self, group_param: &str) -> bool {

    if self.list_index.get() >= self.list_template_group.len() ||
        self.get_element_by_group(group_param, false) {
      return false;
    }

    match self.list_template_group.iter_mut().nth(self.list_index.get()) {
      None => { return false; }
      Some(o) => { o.set_group(group_param); }
    }

    if self.sort_on_add.get() {
      self.sort();
    }

    match self.list_template_group.iter().position(|e| e.group() == group_param) {
      None => { return false; }
      Some(o) => { 
        self.list_index.set(o);

        if self.sort_on_add.get() {
          self.set_updated();
        } else {
          self.sort_updated.set(true);
        }
    
        return true;
      }
    }
  }

  /// Determines when the template group list is sorted.
  /// 
  /// # Arguments
  ///
  /// * `sort_on_add_param` - If true sort when an template group is added,
  ///     otherwise do not sort (for bulk adds).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  pub fn set_sort_on_add(self: &mut Self, sort_on_add_param: bool) -> bool {
    
    if sort_on_add_param == self.sort_on_add.get() {
      return false;
    }
    
    self.sort_on_add.set(sort_on_add_param);
    
    if sort_on_add_param && self.sort_updated.get() {
      self.sort();

      match self.list_template_group.iter().nth(self.list_index.get()) {
        None => { return false; }
        Some(o) => {
          match self.list_template_group.iter().position(|e| e.group() == o.group()) {
            None => { return false; }
            Some(o2) => { 
              self.list_index.set(o2); 
              self.set_updated();
            }
          }          
        }
      }
    }
    
    self.sort_updated.set(false);
    
    return true;
  }

  /// Set sort updated.
  /// 
  /// # Arguments
  ///
  /// * `sort_updated_param` - If true sort updated otherwise false.
  
  pub fn set_sort_updated(self: &mut Self, sort_updated_param: bool) -> bool {
    if self.sort_updated.get() == sort_updated_param {
      return false;
    }
    
    self.sort_updated.set(sort_updated_param);

    return true;
  }
  
  /// Call the updated signal.
  
  pub fn set_updated(self: &Self) -> () {

    self.calc_reg().reg().notify(
      CoreUtility::format_update(ElemUpdateType::Template, ElemLevelType::Engine));
  }

  /// Sort the template group list.
  
  pub fn sort(self: &mut Self) -> () {
    
    self.list_template_group.sort_by(|a, b| ListTemplateGroup::cmp(a, b));
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
  
  fn cmp(a: &ElemTemplateGroup, b: &ElemTemplateGroup) -> std::cmp::Ordering {

    let result = Ord::cmp(a.group(), b.group());
    if result != Equal {
      return result;
    }

    return Equal;
  }

}