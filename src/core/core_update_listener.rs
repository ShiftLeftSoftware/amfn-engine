//! The update listener element.
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

    return UpdateListener {
      update_listener: Box::new(update_listener_param)
    };
  }

  /// Call the update listener function.
  /// 
  /// # Arguments
  ///
  /// * `text` - The text to notify.

  pub fn notify(self: &Self, text: String) -> () {

    (self.update_listener)(text);
  }

}