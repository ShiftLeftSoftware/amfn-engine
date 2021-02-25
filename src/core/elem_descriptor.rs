//! The descriptor element definition.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::{Cell, RefCell};

pub struct ElemDescriptor {
    /// Group name of the descriptor.
    group: String,
    /// Name of the descriptor.
    name: String,
    /// Type of descriptor (locale | custom).
    desc_type: String,
    /// Code for the type of descriptor (ISO language code_ISO country code).
    code: String,
    /// Constant value or the result of an expression.
    value: RefCell<String>,
    /// Optional value expression.
    value_expr: String,
    /// Propagate to the next level if applicable.
    propagate: bool,
    /// Index of the event within the event list (applied by amortization).
    list_event_index: Cell<usize>,
}

/// The descriptor element implementation.

impl ElemDescriptor {
    /// Create a new descriptor element.
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
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        group_param: &str,
        name_param: &str,
        desc_type_param: &str,
        code_param: &str,
        value_param: &str,
        value_expr_param: &str,
        propagate_param: bool,
    ) -> ElemDescriptor {
        ElemDescriptor {
            group: String::from(group_param),
            name: String::from(name_param),
            desc_type: String::from(desc_type_param),
            code: String::from(code_param),
            value: RefCell::new(String::from(value_param)),
            value_expr: String::from(value_expr_param),
            propagate: propagate_param,
            list_event_index: Cell::new(usize::MAX),
        }
    }

    /// Compare this descriptor to the descriptor parameter.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Descriptor to compare.
    ///
    /// # Return
    ///
    /// * True if equal, otherwise false.

    pub fn equal(&self, descriptor: &ElemDescriptor) -> bool {
        if self.group != descriptor.group() {
            return false;
        }
        if self.name != descriptor.name() {
            return false;
        }

        true
    }

    /// Get the descriptor group.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn group(&self) -> &str {
        self.group.as_str()
    }

    /// Get the descriptor name.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the type of descriptor.
    ///
    /// # Return
    ///
    /// * See description.
    ///     

    pub fn desc_type(&self) -> &str {
        self.desc_type.as_str()
    }

    /// Get the descriptor code.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// Get the descriptor value.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value(&self) -> String {
        String::from(self.value.borrow().as_str())
    }

    /// Get the descriptor value expression.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn value_expr(&self) -> String {
        String::from(self.value_expr.as_str())
    }

    /// Get the descriptor propagate.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn propagate(&self) -> bool {
        self.propagate
    }

    /// Get the descriptor list event index.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_event_index(&self) -> usize {
        self.list_event_index.get()
    }

    /// Set the descriptor group.
    ///
    /// # Arguments
    ///
    /// * `group_param` - See description.

    pub fn set_group(&mut self, group_param: &str) {
        self.group = String::from(group_param);
    }

    /// Set the descriptor name.
    ///
    /// # Arguments
    ///
    /// * `name_name` - See description.

    pub fn set_name(&mut self, name_param: &str) {
        self.name = String::from(name_param);
    }

    /// Set the descriptor type.
    ///
    /// # Arguments
    ///
    /// * `desc_type_param` - See description.

    pub fn set_desc_type(&mut self, desc_type_param: &str) {
        self.desc_type = String::from(desc_type_param);
    }

    /// Set the descriptor code.
    ///
    /// # Arguments
    ///
    /// * `code_param` - See description.

    pub fn set_code(&mut self, code_param: &str) {
        self.code = String::from(code_param);
    }

    /// Set the descriptor value.
    ///
    /// # Arguments
    ///
    /// * `value_param` - See description.

    pub fn set_value(&self, value_param: &str) {
        self.value.borrow_mut().clear();
        self.value.borrow_mut().push_str(value_param);
    }

    /// Set the descriptor value expression.
    ///
    /// # Arguments
    ///
    /// * `value_expr_param` - See description.

    pub fn set_value_expr(&mut self, value_expr_param: &str) {
        self.value_expr = String::from(value_expr_param);
    }

    /// Set the descriptor propagate.
    ///
    /// # Arguments
    ///
    /// * `propagate_param` - See description.

    pub fn set_propagate(&mut self, propagate_param: bool) {
        self.propagate = propagate_param;
    }

    /// Set the descriptor list event index.
    ///
    /// # Arguments
    ///
    /// * `list_event_index_param` - See description.

    pub fn set_list_event_index(&self, list_event_index_param: usize) {
        self.list_event_index.set(list_event_index_param);
    }
}
