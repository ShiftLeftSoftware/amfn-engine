/// The core traits.
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

use crate::core::ElemExtension;

/// Common trait for all lists.

pub trait ListTrait {

  /// Clear elements from the list.

  fn clear(self: &mut Self) -> ();

  /// Get the count of the list.
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn count(self: &Self) -> usize;

  /// Get the index of the selected element (starting from 0).
  /// 
  /// # Return
  ///
  /// * See description.
  
  fn index(self: &Self) -> usize;
    
  /// Select an element based upon an index value.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - The index value of the element to select (starting from 0).
  /// 
  /// # Return
  ///
  /// * True if successful, otherwise false.

  fn get_element(self: &Self, index_param: usize) -> bool;

  /// Set the element index.
  /// 
  /// # Arguments
  ///
  /// * `index_param` - The index element to set (starting from 0).

  fn set_index(self: &Self, index_param: usize) -> bool;

}

/// Common trait extension elements.

pub trait ExtensionTrait {

  /// Get the element's extension type.
  /// 
  /// # Return
  ///
  /// * See description.

  fn elem_type(self: &Self) -> crate::ExtensionType;

  /// Get the element's extension.
  /// 
  /// # Return
  ///
  /// * See description.

  fn elem_extension(self: &Self) -> &ElemExtension;

  /// Get the element's mut extension.
  /// 
  /// # Return
  ///
  /// * See description.

  fn elem_extension_mut(self: &mut Self) -> &mut ElemExtension;
  
  /// Set the element's extension.
  /// 
  /// # Arguments
  ///
  /// * `elem_extension_param` - See description.

  fn set_elem_extension(self: &mut Self, elem_extension_param: ElemExtension) -> ();
  
}