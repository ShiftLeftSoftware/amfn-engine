//! The update listener element.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct UpdateListener {
    update_listener: Box<dyn Fn(String)>,
}

/// The update listener implementation.

impl UpdateListener {
    /// Create and return a new update listener.
    ///
    /// # Arguments
    ///
    /// * `update_listener_param` - The update listener callback function.
    ///
    /// # Return
    ///
    /// * See description.    

    pub fn new(update_listener_param: impl Fn(String) + 'static) -> UpdateListener {
        UpdateListener {
            update_listener: Box::new(update_listener_param),
        }
    }

    /// Call the update listener function.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to notify.

    pub fn notify(&self, text: String) {
        (self.update_listener)(text);
    }
}
