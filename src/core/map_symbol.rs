//! The AmFn symbol table.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;

use super::ElemSymbol;

pub struct MapSymbol {
    /// The symbol table object.
    symbol: HashMap<String, ElemSymbol>,
}

/// The AmFn symbol table default implementation.

impl Default for MapSymbol {
    /// Create and return a new symbol table.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        MapSymbol::new()
    }
}

/// The AmFn symbol table implementation.

impl MapSymbol {
    /// Create and return a new symbol table.
    ///
    /// # Return
    ///
    /// * See description.
    pub fn new() -> MapSymbol {
        MapSymbol {
            symbol: HashMap::new(),
        }
    }

    /// Copy and return the new symbol table.
    ///
    /// # Return
    ///
    /// * See description.
    pub fn copy(&self) -> MapSymbol {
        MapSymbol {
            symbol: self.symbol.clone(),
        }
    }

    /// Clear all symbols from the symbol table.

    pub fn clear(&mut self) {
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

    pub fn get_symbol(&self, name: &str) -> Option<&ElemSymbol> {
        self.symbol.get(name)
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

    pub fn get_symbol_mut(&mut self, name: &str) -> Option<&mut ElemSymbol> {
        self.symbol.get_mut(name)
    }

    /// Add the symbol to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The symbol name.
    /// * `elem_symbol` - The symbol element.
    pub fn add_symbol(&mut self, name: &str, elem_symbol: ElemSymbol) {
        self.symbol.insert(String::from(name), elem_symbol);
    }
}
