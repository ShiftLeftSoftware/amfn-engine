/// The core traits.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use crate::core::ElemExtension;

/// Common trait for all lists.

pub trait ListTrait {
    /// Clear elements from the list.

    fn clear(&mut self);

    /// Get the count of the list.
    ///
    /// # Return
    ///
    /// * See description.
    fn count(&self) -> usize;

    /// Get the index of the selected element (starting from 0).
    ///
    /// # Return
    ///
    /// * See description.
    fn index(&self) -> usize;

    /// Select an element based upon an index value.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index value of the element to select (starting from 0).
    ///
    /// # Return
    ///
    /// * True if successful, otherwise false.

    fn get_element(&self, index_param: usize) -> bool;

    /// Set the element index.
    ///
    /// # Arguments
    ///
    /// * `index_param` - The index element to set (starting from 0).

    fn set_index(&self, index_param: usize) -> bool;
}

/// Common trait extension elements.

pub trait ExtensionTrait {
    /// Get the element's extension type.
    ///
    /// # Return
    ///
    /// * See description.

    fn elem_type(&self) -> crate::ExtensionType;

    /// Get the element's extension.
    ///
    /// # Return
    ///
    /// * See description.

    fn elem_extension(&self) -> &ElemExtension;

    /// Get the element's mut extension.
    ///
    /// # Return
    ///
    /// * See description.

    fn elem_extension_mut(&mut self) -> &mut ElemExtension;

    /// Set the element's extension.
    ///
    /// # Arguments
    ///
    /// * `elem_extension_param` - See description.

    fn set_elem_extension(&mut self, elem_extension_param: ElemExtension);
}
