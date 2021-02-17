//! The AmFn symbol table.
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

use std::collections::HashMap;

use super::ElemSymbol;

pub struct MapSymbol {

  /// The symbol table object. 
  symbol: HashMap<String, ElemSymbol>

}
/// The AmFn symbol table implementation.

impl MapSymbol {

  /// Create and return a new hash table element.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn new() -> MapSymbol {
    
    return MapSymbol {
      symbol: HashMap::new()
    }
  }

  /// Copy the hash table and return the new element.
  /// 
  /// # Return
  ///
  /// * See description.
  
  pub fn copy(self: &Self) -> MapSymbol {
    
    return MapSymbol {
      symbol: self.symbol.clone()
    }
  }

  /// Clear all symbols from the symbol table.

  pub fn clear(self: &mut Self) -> () {
    
    self.symbol.clear();
  }

  /// Find and return the symbol element by name.
  /// 
  /// # Arguments
  ///
  /// * `name` - The name of the symbol to find.
  /// 
  /// # Return
  ///
  /// * The found symbol, otherwise None.

  pub fn get_symbol(self: &Self, name: &str) -> Option<&ElemSymbol> {

    return self.symbol.get(name);
  }

  /// Find and return the mut symbol element by name.
  /// 
  /// # Arguments
  ///
  /// * `name` - The name of the symbol to find.
  /// 
  /// # Return
  ///
  /// * The found symbol, otherwise None.

  pub fn get_symbol_mut(self: &mut Self, name: &str) -> Option<&mut ElemSymbol> {

    return self.symbol.get_mut(name);
  }

  /// Add the symbol to the symbol table.
  /// 
  /// # Arguments
  ///
  /// * `name` - The symbol name.
  /// * `elem_symbol` - The symbol element.
  
  pub fn add_symbol(self: &mut Self, name: &str, elem_symbol: ElemSymbol) -> () {
    
    self.symbol.insert(String::from(name), elem_symbol);
  }

}