//! The AmFn expression mechanism.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rust_decimal::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

use super::{CalcManager, CalcScan};
use crate::core::{
    CoreUtility, ElemBalanceResult, ElemSymbol, ListAmortization, ListDescriptor, ListParameter,
    MapSymbol,
};
use crate::{ListTrait};

pub struct CalcExpression {
    /// Calculator manager element.
    calc_manager: Rc<RefCell<CalcManager>>,

    /// Symbol table.
    symbol_table: Rc<RefCell<MapSymbol>>,
    /// Token scanner.
    scanner: RefCell<CalcScan>,
    /// User descriptor list.
    list_descriptor_user: ListDescriptor,
    /// Cashflow descriptor list.
    list_descriptor_cashflow: Option<ListDescriptor>,
    /// Event descriptor list.
    list_descriptor_event: Option<ListDescriptor>,
    /// Start of fiscal year in MMDD format.
    fiscal_year_start: usize,
    /// Number of significant decimal digits.
    decimal_digits: usize,
}

/// The AmFn expression mechanism implementation.

impl CalcExpression {
    /// Create and return a new AmFn expression.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `fiscal_year_start_param` - Start of fiscal year in MMDD format.
    /// * `decimal_digits_param` - Number of significant decimal digits.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        fiscal_year_start_param: usize,
        decimal_digits_param: usize,
    ) -> CalcExpression {
        let updating_json = calc_manager_param.borrow().updating_json();
        let tlist_descriptor_user = calc_manager_param
            .borrow()
            .preferences()
            .list_descriptor()
            .copy(false, updating_json);

        CalcExpression {
            calc_manager: Rc::clone(calc_manager_param),
            symbol_table: Rc::new(RefCell::new(MapSymbol::new())),
            scanner: RefCell::new(CalcScan::new("")),
            list_descriptor_user: tlist_descriptor_user,
            list_descriptor_cashflow: None,
            list_descriptor_event: None,
            fiscal_year_start: fiscal_year_start_param,
            decimal_digits: decimal_digits_param,
        }
    }

    /// Create and return a new AmFn expression for an internal sub-expression.
    ///
    /// # Arguments
    ///
    /// * `calc_manager_param` - Calculation manager.
    /// * `symbol_param` - Symbol table reference from base instance.
    /// * `fiscal_year_start_param` - Start of fiscal year in MMDD format.
    /// * `decimal_digits_param` - Number of significant decimal digits.
    /// * `expr_str` - The sub-expression to execute.
    ///
    /// # Return
    ///
    /// * See description.

    fn new_with_symbol_table(
        calc_manager_param: &Rc<RefCell<CalcManager>>,
        symbol_param: &Rc<RefCell<MapSymbol>>,
        fiscal_year_start_param: usize,
        decimal_digits_param: usize,
        expr_str: &str,
    ) -> CalcExpression {
        let updating_json = calc_manager_param.borrow().updating_json();
        let tlist_descriptor_user = calc_manager_param
            .borrow()
            .preferences()
            .list_descriptor()
            .copy(false, updating_json);

        CalcExpression {
            calc_manager: Rc::clone(calc_manager_param),
            symbol_table: symbol_param.clone(),
            scanner: RefCell::new(CalcScan::new(expr_str)),
            list_descriptor_user: tlist_descriptor_user,
            list_descriptor_cashflow: None,
            list_descriptor_event: None,
            fiscal_year_start: fiscal_year_start_param,
            decimal_digits: decimal_digits_param,
        }
    }

    /// Get the calculation manager.
    ///
    /// # Return
    ///
    /// * See description.

    fn calc_mgr(&self) -> Ref<CalcManager> {
        self.calc_manager.borrow()
    }

    /// Get the scanner.
    ///
    /// # Return
    ///
    /// * See description.

    fn scanner(&self) -> Ref<CalcScan> {
        self.scanner.borrow()
    }

    /// Get the mutable scanner.
    ///
    /// # Return
    ///
    /// * See description.

    fn scanner_mut(&self) -> RefMut<CalcScan> {
        self.scanner.borrow_mut()
    }

    /// Get the symbol table.
    ///
    /// # Return
    ///
    /// * See description.

    fn symbol_table(&self) -> Ref<MapSymbol> {
        self.symbol_table.borrow()
    }

    /// Get the mutable symbol table.
    ///
    /// # Return
    ///
    /// * See description.

    fn symbol_table_mut(&self) -> RefMut<MapSymbol> {
        self.symbol_table.borrow_mut()
    }

    /// Clear the symbol table.

    pub fn clear(&self) {
        self.symbol_table_mut().clear();
    }

    /// Initialize the instance variables with a new expression, optional
    /// parameters, optional cashflow descriptors, and optional event descriptors.
    ///
    /// # Arguments
    ///
    /// * `list_descriptor_cashflow_param` - The cashflow descriptor (or None).
    /// * `list_descriptor_event_param` - The event descriptor (or None).
    /// * `list_parameter_param` - The parameters for the expression (or None).
    /// * `expression` - The expression to execute.

    pub fn init_expression(
        &mut self,
        list_descriptor_cashflow_param: Option<&ListDescriptor>,
        list_descriptor_event_param: Option<&ListDescriptor>,
        list_parameter_param: Option<&ListParameter>,
        expression: &str,
    ) {
        let updating_json = self.calc_mgr().updating_json();
        match list_descriptor_cashflow_param {
            None => {}
            Some(o) => {
                self.list_descriptor_cashflow =
                    Option::from(o.copy(false, updating_json));
            }
        }
        match list_descriptor_event_param {
            None => {}
            Some(o) => {
                self.list_descriptor_event =
                    Option::from(o.copy(false, updating_json));
            }
        }

        self.init_list_parameter(list_parameter_param);
        self.scanner_mut().init_scan(expression);
    }

    /// Initialize the symbol table with passed parameters.
    ///
    /// # Arguments
    ///
    /// * `list_parameter_param` - The parameters to the expression.

    pub fn init_list_parameter(&mut self, list_parameter_param: Option<&ListParameter>) {
        let list_parameter: &ListParameter;
        match list_parameter_param {
            None => {
                return;
            } // Empty list parameter
            Some(o) => {
                list_parameter = o;
            }
        }

        let orig_index = list_parameter.index();
        let mut index: usize = 0;

        loop {
            if !list_parameter.get_element(index) {
                break;
            }

            let name = list_parameter.name();
            let mut elem_symbol = ElemSymbol::new();

            match list_parameter.param_type() {
                crate::TokenType::Integer => {
                    elem_symbol.set_integer(list_parameter.param_integer());
                }
                crate::TokenType::Decimal => {
                    elem_symbol.set_decimal(list_parameter.param_decimal());
                }
                _ => {
                    elem_symbol.set_string(list_parameter.param_string());
                }
            }

            self.symbol_table_mut().add_symbol(name, elem_symbol);

            index += 1;
        }

        list_parameter.set_index(orig_index);
    }

    /// Create a new integer symbol and add it into the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    /// * `value` - The integer value of the symbol.

    pub fn set_symbol_integeri(&self, name: &str, value: i32) {
        let mut elem_symbol = ElemSymbol::new();

        elem_symbol.set_integeri(value);

        self.symbol_table_mut().add_symbol(name, elem_symbol);
    }

    /// Create a new integer symbol and add it into the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    /// * `value` - The integer value of the symbol.

    pub fn set_symbol_integer(&self, name: &str, value: usize) {
        self.set_symbol_integeri(name, value as i32);
    }

    /// Create a new Decimal symbol and add it into the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    /// * `value` - The Decimal value of the symbol.

    pub fn set_symbol_decimal(&self, name: &str, value: Decimal) {
        let mut elem_symbol = ElemSymbol::new();

        elem_symbol.set_decimal(value);

        self.symbol_table_mut().add_symbol(name, elem_symbol);
    }

    /// Create a new string symbol and add it into the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    /// * `value` - The string value of the symbol.

    pub fn set_symbol_string(&self, name: &str, value: &str) {
        let mut elem_symbol = ElemSymbol::new();

        elem_symbol.set_string(value);

        self.symbol_table_mut().add_symbol(name, elem_symbol);
    }

    /// Executes the currently initialized expression.
    /// The expression is first parsed into postfix notation.
    /// Then the resulting symbols are executed on a stack with
    /// the final result being returned from this method.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn evaluate(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut expression = VecDeque::new();
        let mut stack = Vec::new();
        let mut elem_last_symbol: Option<ElemSymbol>;
        let mut text: String;
        let mut ch1: char;
        let mut level: i32 = 0;
        let mut operator_needed: bool;

        loop {
            expression.clear();
            stack.clear();
            operator_needed = false;

            while self.scanner_mut().scan_token() != crate::TokenType::Unknown {
                let scan_type = self.scanner().get_type();

                if level == 0
                    && scan_type == crate::TokenType::Punctuation
                    && self.scanner().get_token().starts_with(',')
                {
                    break;
                }

                text = String::from(self.scanner().get_token());

                ch1 = '\x00';
                if !text.is_empty() {
                    ch1 = text.chars().next().unwrap_or(' ');
                }
                match scan_type {
                    crate::TokenType::Punctuation => {
                        match ch1 {
                            '(' => {
                                level += 1;
                                stack.push(ElemSymbol::new_with_token_type(
                                    crate::TokenType::LeftParen,
                                ));
                            }
                            ')' => {
                                level -= 1;
                                if level < 0 {
                                    return Err(crate::ErrorType::LeftParen);
                                }
                                loop {
                                    let elem_symbol: ElemSymbol;
                                    match stack.pop() {
                                        None => {
                                            break;
                                        }
                                        Some(o) => {
                                            elem_symbol = o;
                                        }
                                    }
                                    if elem_symbol.sym_type() == crate::TokenType::LeftParen {
                                        break;
                                    }
                                    expression.push_back(elem_symbol);
                                }
                            }
                            _ => {
                                if (ch1 == '<' || ch1 == '>')
                                    && self.scanner_mut().scan_token() != crate::TokenType::Unknown
                                {
                                    let ch2 =
                                        self.scanner().get_token().chars().next().unwrap_or(' ');
                                    if ch2 == '=' || (ch1 == '<' && ch2 == '>') {
                                        text += self.scanner().get_token();
                                    } else {
                                        self.scanner_mut().rescan();
                                    }
                                }
                                if !self
                                    .calc_mgr()
                                    .mgr()
                                    .operators()
                                    .get_element_by_key(text.as_str())
                                {
                                    return Err(crate::ErrorType::InvalidOperator);
                                }
                                if operator_needed
                                    && CoreUtility::get_operator(
                                        self.calc_mgr().mgr().operators().value(),
                                    ) == crate::OperatorType::UnaryNot
                                {
                                    return Err(crate::ErrorType::InvalidOperator);
                                }
                                if !operator_needed {
                                    let op_type = CoreUtility::get_operator(
                                        self.calc_mgr().mgr().operators().value(),
                                    );
                                    match op_type {
                                        crate::OperatorType::Minus => {
                                            if !self
                                                .calc_mgr()
                                                .mgr()
                                                .operators()
                                                .get_element_by_key("~") {
                                                // Unary minus
                                                return Err(crate::ErrorType::InvalidOperator);
                                            }
                                        }
                                        crate::OperatorType::UnaryNot => {}
                                        _ => {
                                            return Err(crate::ErrorType::Operand);
                                        }
                                    }
                                }
                                while !stack.is_empty() {
                                    let elem = &stack[stack.len() - 1];
                                    if elem.sym_type() == crate::TokenType::LeftParen
                                        || (elem.sym_type() == crate::TokenType::Operator
                                            && self
                                                .calc_mgr()
                                                .mgr()
                                                .operators()
                                                .get_value_ext_by_index(elem.sym_integer())
                                                < self.calc_mgr().mgr().operators().value_ext())
                                    {
                                        break;
                                    }
                                    match stack.pop() {
                                        None => {}
                                        Some(o) => {
                                            expression.push_back(o);
                                        }
                                    }
                                }
                                let mut elem =
                                    ElemSymbol::new_with_token_type(crate::TokenType::Operator);
                                elem.set_operator(self.calc_mgr().mgr().operators().index());
                                stack.push(elem);
                                operator_needed = false;
                            }
                        }
                    }
                    crate::TokenType::Integer => {
                        if operator_needed {
                            return Err(crate::ErrorType::Operator);
                        }
                        let mut elem_symbol = ElemSymbol::new();
                        elem_symbol.set_integeri(CoreUtility::parse_integeri(text.as_str()));
                        expression.push_back(elem_symbol);
                        operator_needed = true;
                    }
                    crate::TokenType::Decimal => {
                        if operator_needed {
                            return Err(crate::ErrorType::Operator);
                        }
                        let mut elem_symbol = ElemSymbol::new();
                        elem_symbol.set_decimal(CoreUtility::parse_decimal(text.as_str()));
                        expression.push_back(elem_symbol);
                        operator_needed = true;
                    }
                    crate::TokenType::String => {
                        if operator_needed {
                            return Err(crate::ErrorType::Operator);
                        }
                        let mut elem_symbol = ElemSymbol::new();
                        elem_symbol.set_string(text.as_str());
                        expression.push_back(elem_symbol);
                        operator_needed = true;
                    }
                    crate::TokenType::Alpha => {
                        if self
                            .calc_mgr()
                            .mgr()
                            .operators()
                            .get_element_by_key(text.as_str())
                        {
                            while !stack.is_empty() {
                                let elem_symbol = &stack[stack.len() - 1];
                                if elem_symbol.sym_type() == crate::TokenType::LeftParen
                                    || (elem_symbol.sym_type() == crate::TokenType::Operator
                                        && self
                                            .calc_mgr()
                                            .mgr()
                                            .operators()
                                            .get_value_ext_by_index(elem_symbol.sym_integer())
                                            < self.calc_mgr().mgr().operators().value_ext())
                                {
                                    break;
                                }
                                match stack.pop() {
                                    None => {}
                                    Some(o) => {
                                        expression.push_back(o);
                                    }
                                }
                            }
                            let mut elem_symbol =
                                ElemSymbol::new_with_token_type(crate::TokenType::Operator);
                            elem_symbol.set_operator(self.calc_mgr().mgr().operators().index());
                            stack.push(elem_symbol);
                            operator_needed = false;
                        } else {
                            if operator_needed {
                                return Err(crate::ErrorType::Operator);
                            }
                            let token = self.scanner_mut().scan_token();
                            if token == crate::TokenType::Punctuation
                                && self.scanner().get_token().starts_with('(')
                            {
                                let result = self.function(
                                    text.as_str(),
                                    list_am_opt,
                                    elem_balance_result_opt,
                                );
                                match result {
                                    Err(e) => {
                                        return Err(e);
                                    }
                                    Ok(o) => {
                                        expression.push_back(o);
                                    }
                                }
                            } else {
                                if !(self.scanner().get_type() == crate::TokenType::Punctuation
                                    && self.scanner().get_token().starts_with('['))
                                {
                                    self.scanner_mut().rescan();
                                } else {
                                    self.scanner_mut().scan_token();
                                    let result =
                                        self.get_expr_integer(list_am_opt, elem_balance_result_opt);
                                    match result {
                                        Err(e) => {
                                            return Err(e);
                                        }
                                        Ok(o) => {
                                            if !(self.scanner().get_type()
                                                == crate::TokenType::Punctuation
                                                && self.scanner().get_token().starts_with(']'))
                                            {
                                                return Err(crate::ErrorType::RightBracket);
                                            }
                                            text += format!("[{}]", o.sym_integer()).as_str();
                                        }
                                    }
                                }
                                let sym_table = self.symbol_table();
                                let sym_opt = sym_table.get_symbol(text.as_str());
                                if sym_opt.is_none() {
                                    return Err(crate::ErrorType::InvalidSymbol);
                                }
                                match sym_opt {
                                    None => {}
                                    Some(o) => {
                                        let mut sym2 = ElemSymbol::new();
                                        match o.sym_type() {
                                            crate::TokenType::Integer => {
                                                sym2.set_integer(o.sym_integer());
                                            }
                                            crate::TokenType::Decimal => {
                                                sym2.set_decimal(o.sym_decimal());
                                            }
                                            crate::TokenType::String => {
                                                sym2.set_string(o.sym_string());
                                            }
                                            _ => {}
                                        }
                                        expression.push_back(sym2);
                                    }
                                }
                            }
                            operator_needed = true;
                        }
                    }
                    _ => {
                        return Err(crate::ErrorType::InvalidToken);
                    }
                }
            }

            if level > 0 {
                return Err(crate::ErrorType::RightParen);
            }

            loop {
                match stack.pop() {
                    None => {
                        break;
                    }
                    Some(o) => {
                        expression.push_back(o);
                    }
                }
            }

            let mut elem_symbol1_opt: Option<ElemSymbol>;
            let mut elem_symbol2_opt: Option<ElemSymbol>;
            for elem_symbol in expression.iter_mut() {
                match elem_symbol.sym_type() {
                    crate::TokenType::Operator => {
                        if !self
                            .calc_mgr()
                            .mgr()
                            .operators()
                            .get_element(elem_symbol.sym_integer())
                        {
                            return Err(crate::ErrorType::InvalidOperator);
                        }
                        if stack.is_empty() {
                            return Err(crate::ErrorType::MissingOperand);
                        }
                        elem_symbol2_opt = stack.pop();
                        let sym_val = CoreUtility::get_operator(elem_symbol.sym_integer());
                        if sym_val == crate::OperatorType::UnaryMinus
                            || sym_val == crate::OperatorType::UnaryNot
                        {
                            match elem_symbol2_opt.as_mut() {
                                None => {
                                    return Err(crate::ErrorType::MissingOperand);
                                }
                                Some(o) => {
                                    elem_symbol1_opt = Option::from(o.copy());
                                }
                            }
                        } else {
                            if stack.is_empty() {
                                return Err(crate::ErrorType::MissingOperand);
                            }
                            elem_symbol1_opt = stack.pop();
                            let elem_symbol1: &mut ElemSymbol;
                            match elem_symbol1_opt.as_mut() {
                                None => {
                                    return Err(crate::ErrorType::MissingOperand);
                                }
                                Some(o) => {
                                    elem_symbol1 = o;
                                }
                            }
                            let elem_symbol2: &mut ElemSymbol;
                            match elem_symbol2_opt.as_mut() {
                                None => {
                                    return Err(crate::ErrorType::MissingOperand);
                                }
                                Some(o) => {
                                    elem_symbol2 = o;
                                }
                            }
                            if elem_symbol1.sym_type() == crate::TokenType::Integer
                                && elem_symbol2.sym_type() == crate::TokenType::Decimal
                            {
                                elem_symbol1.set_decimal(dec!(elem_symbol1.sym_integer()));
                            } else if elem_symbol1.sym_type() == crate::TokenType::Decimal
                                && elem_symbol2.sym_type() == crate::TokenType::Integer
                            {
                                elem_symbol2.set_decimal(dec!(elem_symbol2.sym_integer()));
                            }
                        }
                        let elem_symbol1: &mut ElemSymbol;
                        match elem_symbol1_opt.as_mut() {
                            None => {
                                return Err(crate::ErrorType::MissingOperand);
                            }
                            Some(o) => {
                                elem_symbol1 = o;
                            }
                        }
                        let elem_symbol2: &mut ElemSymbol;
                        match elem_symbol2_opt.as_mut() {
                            None => {
                                return Err(crate::ErrorType::MissingOperand);
                            }
                            Some(o) => {
                                elem_symbol2 = o;
                            }
                        }
                        match CoreUtility::get_operator(self.calc_mgr().mgr().operators().value()) {
                            crate::OperatorType::And => {
                                if elem_symbol1.sym_type() != crate::TokenType::Integer
                                    || elem_symbol2.sym_type() != crate::TokenType::Integer
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_integer() != 0
                                    && elem_symbol2.sym_integer() != 0
                                {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::Or => {
                                if elem_symbol1.sym_type() != crate::TokenType::Integer
                                    || elem_symbol2.sym_type() != crate::TokenType::Integer
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_integer() != 0
                                    || elem_symbol2.sym_integer() != 0
                                {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::Greater => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol1.sym_integer() > elem_symbol2.sym_integer() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_decimal() > elem_symbol2.sym_decimal() {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::Less => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol1.sym_integer() < elem_symbol2.sym_integer() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_decimal() < elem_symbol2.sym_decimal() {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::GreaterEqual => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol1.sym_integer() >= elem_symbol2.sym_integer() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_decimal() >= elem_symbol2.sym_decimal() {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::LessEqual => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol1.sym_integer() <= elem_symbol2.sym_integer() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_decimal() <= elem_symbol2.sym_decimal() {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::Equal => {
                                if elem_symbol1.sym_type() != elem_symbol2.sym_type() {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol1.sym_integer() == elem_symbol2.sym_integer() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_type() == crate::TokenType::Decimal {
                                    if elem_symbol1.sym_decimal() == elem_symbol2.sym_decimal() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_string() == elem_symbol2.sym_string() {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::NotEqual => {
                                if elem_symbol1.sym_type() != elem_symbol2.sym_type() {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol1.sym_integer() != elem_symbol2.sym_integer() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_type() == crate::TokenType::Decimal {
                                    if elem_symbol1.sym_decimal() != elem_symbol2.sym_decimal() {
                                        elem_symbol1.set_integer(1);
                                    } else {
                                        elem_symbol1.set_integer(0);
                                    }
                                } else if elem_symbol1.sym_string() == elem_symbol2.sym_string() {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            crate::OperatorType::Plus => {
                                if elem_symbol1.sym_type() != elem_symbol2.sym_type() {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    elem_symbol1.set_integer(
                                        elem_symbol1.sym_integer() + elem_symbol2.sym_integer(),
                                    );
                                } else if elem_symbol1.sym_type() == crate::TokenType::Decimal {
                                    elem_symbol1.set_decimal(
                                        elem_symbol1.sym_decimal() + elem_symbol2.sym_decimal(),
                                    );
                                } else {
                                    elem_symbol1.set_string(
                                        format!(
                                            "{}{}",
                                            elem_symbol1.sym_string(),
                                            elem_symbol2.sym_string()
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                            crate::OperatorType::Minus => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    elem_symbol1.set_integer(
                                        elem_symbol1.sym_integer() - elem_symbol2.sym_integer(),
                                    );
                                } else {
                                    elem_symbol1.set_decimal(
                                        elem_symbol1.sym_decimal() - elem_symbol2.sym_decimal(),
                                    );
                                }
                            }
                            crate::OperatorType::Times => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    elem_symbol1.set_integer(
                                        elem_symbol1.sym_integer() * elem_symbol2.sym_integer(),
                                    );
                                } else {
                                    elem_symbol1.set_decimal(
                                        elem_symbol1.sym_decimal() * elem_symbol2.sym_decimal(),
                                    );
                                }
                            }
                            crate::OperatorType::Divide => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol2.sym_integer() == 0 {
                                        elem_symbol1.set_integer(0); // Do not error on divide by zero
                                    } else {
                                        elem_symbol1.set_integer(
                                            elem_symbol1.sym_integer() / elem_symbol2.sym_integer(),
                                        );
                                    }
                                } else if elem_symbol2.sym_decimal() == dec!(0.0) {
                                    elem_symbol1.set_decimal(dec!(0.0)); // Do not error on divide by zero
                                } else {
                                    elem_symbol1.set_decimal(
                                        elem_symbol1.sym_decimal() / elem_symbol2.sym_decimal(),
                                    );
                                }
                            }
                            crate::OperatorType::Modulus => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    if elem_symbol2.sym_integer() == 0 {
                                        elem_symbol1.set_integer(0); // Do not error on divide by zero
                                    } else {
                                        elem_symbol1.set_integer(
                                            elem_symbol1.sym_integer() % elem_symbol2.sym_integer(),
                                        );
                                    }
                                } else if elem_symbol2.sym_decimal() == dec!(0.0) {
                                    elem_symbol1.set_decimal(dec!(0.0)); // Do not error on divide by zero
                                } else {
                                    elem_symbol1.set_decimal(
                                        elem_symbol1.sym_decimal() % elem_symbol2.sym_decimal(),
                                    );
                                }
                            }
                            crate::OperatorType::Exponent => {
                                if elem_symbol1.sym_type() == crate::TokenType::String
                                    || elem_symbol2.sym_type() == crate::TokenType::String
                                {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol1.sym_type() == crate::TokenType::Integer {
                                    elem_symbol1.set_integer(
                                        elem_symbol1
                                            .sym_integer()
                                            .pow(elem_symbol2.sym_integer() as u32),
                                    );
                                } else {
                                    match elem_symbol2.sym_decimal().to_usize() {
                                        None => {
                                            return Err(crate::ErrorType::InvalidOperand);
                                        }
                                        Some(o) => {
                                            elem_symbol1.set_decimal(CoreUtility::decimal_pow(
                                                elem_symbol1.sym_decimal(),
                                                o,
                                            ));
                                        }
                                    }
                                }
                            }
                            crate::OperatorType::UnaryMinus => {
                                if elem_symbol2.sym_type() == crate::TokenType::String {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol2.sym_type() == crate::TokenType::Integer {
                                    elem_symbol1.set_integeri(-elem_symbol2.sym_integeri());
                                } else {
                                    elem_symbol1.set_decimal(-elem_symbol2.sym_decimal());
                                }
                            }
                            crate::OperatorType::UnaryNot => {
                                if elem_symbol2.sym_type() != crate::TokenType::Integer {
                                    return Err(crate::ErrorType::InvalidOperand);
                                }
                                if elem_symbol2.sym_integer() == 0 {
                                    elem_symbol1.set_integer(1);
                                } else {
                                    elem_symbol1.set_integer(0);
                                }
                            }
                            _ => {}
                        }
                        stack.push(elem_symbol1.copy());
                    }
                    crate::TokenType::Integer
                    | crate::TokenType::Decimal
                    | crate::TokenType::String => {
                        stack.push(elem_symbol.copy());
                    }
                    _ => {}
                }
            }

            if stack.len() != 1 {
                return Err(crate::ErrorType::Incomplete);
            }

            elem_last_symbol = stack.pop();

            if self.scanner().get_type() == crate::TokenType::Unknown {
                break;
            }
        }
        if elem_last_symbol.is_none() {
            return Err(crate::ErrorType::Incomplete);
        }
        let elem: &ElemSymbol;
        match elem_last_symbol.as_ref() {
            None => {
                return Err(crate::ErrorType::Incomplete);
            }
            Some(o) => {
                elem = o;
            }
        }

        let mut elem_result_symbol = ElemSymbol::new();

        match elem.sym_type() {
            crate::TokenType::Integer => {
                elem_result_symbol.set_integer(elem.sym_integer());
            }
            crate::TokenType::Decimal => {
                elem_result_symbol.set_decimal(elem.sym_decimal());
            }
            crate::TokenType::String => {
                elem_result_symbol.set_string(elem.sym_string());
            }
            _ => {}
        }

        Ok(elem_result_symbol)
    }

    /// Parses an unknown type of symbol from the expression.
    /// This method will first look for a sub-expression and,
    /// if found, instanciate this element recursively to
    /// return the result of the sub-expression. However,
    /// if a sub-expression is not found, this method will parse
    /// the symbol, lookup it's value in the symbol table, and
    /// return the symbol's value as the result.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    /// * `symbol_name` - If true return the symbol name itself,
    ///     otherwise lookup the symbol name in the symbol table and
    ///     return it's value.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn get_expr_symbol(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
        symbol_name: bool,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol = ElemSymbol::new();
        if self.scanner_mut().scan_subexpression() {
            let mut elem_symbol_opt = None;
            let token: String;

            {
                let mut sym_table = self.symbol_table_mut();
                token = String::from(self.scanner().get_token());
                match sym_table.get_symbol_mut(token.as_str()) {
                    None => {}
                    Some(o) => {
                        elem_symbol_opt = Option::from(o.copy());
                    }
                }
            }
            if elem_symbol_opt.is_none() {

                let calc_expression = CalcExpression::new_with_symbol_table(
                    &self.calc_manager,
                    &self.symbol_table,
                    self.fiscal_year_start,
                    self.decimal_digits,
                    token.as_str(),
                );

                let result = calc_expression.evaluate(list_am_opt, elem_balance_result_opt);
                match result {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(o) => {
                        elem_symbol_opt = Option::from(o);
                    }
                }
            }

            let elem_symbol: &mut ElemSymbol;
            match elem_symbol_opt.as_mut() {
                None => {
                    return Err(crate::ErrorType::Incomplete);
                }
                Some(o) => {
                    elem_symbol = o;
                }
            }

            match elem_symbol.sym_type() {
                crate::TokenType::Integer => {
                    elem_result_symbol.set_integer(elem_symbol.sym_integer());
                }
                crate::TokenType::Decimal => {
                    elem_result_symbol.set_decimal(elem_symbol.sym_decimal());
                }
                crate::TokenType::String => {
                    elem_result_symbol.set_string(elem_symbol.sym_string());
                }
                _ => {}
            }

            return Ok(elem_result_symbol);
        }
        let scan_type = self.scanner().get_type();

        let mut signed: bool = false;
        if scan_type == crate::TokenType::Punctuation && self.scanner().get_token().starts_with('-')
        {
            self.scanner_mut().scan_token();
            signed = true;
        }

        let mut text: String;
        {
            let scanner = self.scanner();
            text = String::from(scanner.get_token());
        }

        match scan_type {
            crate::TokenType::Integer => {
                let mut result = CoreUtility::parse_integeri(text.as_str());
                if signed {
                    result = -result;
                }
                elem_result_symbol.set_integeri(result);
            }
            crate::TokenType::Decimal => {
                let mut dresult = CoreUtility::parse_decimal(text.as_str());
                if signed {
                    dresult = -dresult;
                }
                elem_result_symbol.set_decimal(dresult);
            }
            crate::TokenType::String => {
                elem_result_symbol.set_string(text.as_str());
            }
            crate::TokenType::Alpha => {
                let token = self.scanner_mut().scan_token();
                if token == crate::TokenType::Punctuation
                    && self.scanner().get_token().starts_with('(')
                {
                    let result = self.function(text.as_str(), list_am_opt, elem_balance_result_opt);
                    match result {
                        Err(e) => {
                            return Err(e);
                        }
                        Ok(o) => {
                            elem_result_symbol = o;
                        }
                    }
                } else {
                    if !(self.scanner().get_type() == crate::TokenType::Punctuation
                        && self.scanner().get_token().starts_with('['))
                    {
                        self.scanner_mut().rescan();
                    } else {
                        self.scanner_mut().scan_token();
                        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
                        match result {
                            Err(e) => {
                                return Err(e);
                            }
                            Ok(o) => {
                                elem_result_symbol = o;
                            }
                        }
                        if !(self.scanner().get_type() == crate::TokenType::Punctuation
                            && self.scanner().get_token().starts_with(']'))
                        {
                            return Err(crate::ErrorType::RightBracket);
                        }
                        text = format!("{}[{}]", text, elem_result_symbol.sym_integer());
                    }
                    if symbol_name {
                        elem_result_symbol.set_string(text.as_str());
                    } else {
                        let sym_table = self.symbol_table();
                        let elem_symbol_opt = sym_table.get_symbol(text.as_str());
                        if elem_symbol_opt.is_none() {
                            return Err(crate::ErrorType::InvalidSymbol);
                        }
                        match elem_symbol_opt.as_ref() {
                            None => {
                                return Err(crate::ErrorType::Incomplete);
                            }
                            Some(o) => match o.sym_type() {
                                crate::TokenType::Integer => {
                                    elem_result_symbol.set_integer(o.sym_integer());
                                }
                                crate::TokenType::Decimal => {
                                    elem_result_symbol.set_decimal(o.sym_decimal());
                                }
                                crate::TokenType::String => {
                                    elem_result_symbol.set_string(o.sym_string());
                                }
                                _ => {}
                            },
                        }
                    }
                }
                if signed {
                    match elem_result_symbol.sym_type() {
                        crate::TokenType::Integer => {
                            elem_result_symbol.set_integeri(-elem_result_symbol.sym_integeri());
                        }
                        crate::TokenType::Decimal => {
                            elem_result_symbol.set_decimal(-elem_result_symbol.sym_decimal());
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                return Err(crate::ErrorType::InvalidToken);
            }
        }

        Ok(elem_result_symbol)
    }

    /// Parses an integer symbol from the expression.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn get_expr_integer(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);

        match result.as_ref() {
            Err(e) => Err(*e),
            Ok(o) => {
                if o.sym_type() != crate::TokenType::Integer {
                    return Err(crate::ErrorType::Integer);
                }

                result
            }
        }
    }

    /// Parses a Decimal symbol from the expression.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn get_expr_decimal(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);

        match result.as_ref() {
            Err(e) => Err(*e),
            Ok(o) => {
                if o.sym_type() != crate::TokenType::Decimal {
                    return Err(crate::ErrorType::Decimal);
                }
                result
            }
        }
    }

    /// Parses a string symbol from the expression.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn get_expr_string(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);

        match result.as_ref() {
            Err(e) => Err(*e),
            Ok(o) => {
                if o.sym_type() != crate::TokenType::String {
                    return Err(crate::ErrorType::String);
                }
                result
            }
        }
    }

    /// Parses and executes a function from the expression.
    /// Based upon a given function name, the appropriate method
    /// is called to parse the parameters, execute the specific
    /// function and return the resulting symbol.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function(
        &self,
        name: &str,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        if !self.calc_mgr().mgr().functions().get_element_by_key(name) {
            return Err(crate::ErrorType::Function);
        }

        self.scanner_mut().scan_token();

        let mut result: Result<ElemSymbol, crate::ErrorType>;

        let function_type = CoreUtility::get_function(self.calc_mgr().mgr().functions().value());
        match function_type {
            crate::FunctionType::Abs => {
                result = self.function_abs(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Am => {
                result = self.function_am(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Cashflow => {
                result = self.function_cashflow(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::DateDiff => {
                result = self.function_date_diff(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::DateFiscal => {
                result = self.function_date_fiscal(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::DateNew => {
                result = self.function_date_new(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::DateNow => {
                result = self.function_date_now();
            }
            crate::FunctionType::Default => {
                result = self.function_default(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Descriptor => {
                result = self.function_descriptor(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Decimal => {
                result = self.function_decimal(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Format => {
                result = self.function_format(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::FormatCurrency => {
                result = self.function_format_currency(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::FormatDate => {
                result = self.function_format_date(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::FormatNumber => {
                result = self.function_format_number(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::If => {
                result = self.function_if(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Integer => {
                result = self.function_integer(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Len => {
                result = self.function_len(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Lowercase => {
                result = self.function_lowercase(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Max => {
                result = self.function_max(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Mid => {
                result = self.function_mid(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Min => {
                result = self.function_min(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Parse => {
                result = self.function_parse(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Pr => {
                result = self.function_pr(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Replace => {
                result = self.function_replace(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Round => {
                result = self.function_round(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::RoundFraction => {
                result = self.function_round_fraction(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Set => {
                result = self.function_set(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Trim => {
                result = self.function_trim(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Type => {
                result = self.function_type(list_am_opt, elem_balance_result_opt);
            }
            crate::FunctionType::Uppercase => {
                result = self.function_uppercase(list_am_opt, elem_balance_result_opt);
            }
            _ => {
                return Err(crate::ErrorType::Function);
            }
        }

        match result.as_ref() {
            Err(_e) => {}
            Ok(_o) => {
                let token = self.scanner_mut().scan_token();
                if !(token == crate::TokenType::Punctuation
                    && self.scanner().get_token().starts_with(')'))
                {
                    result = Err(crate::ErrorType::RightParen);
                }
            }
        }

        result
    }

    /// Parses and executes the absolute value function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_abs(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        let mut elem_result_symbol = ElemSymbol::new();

        match result {
            Err(e) => Err(e),
            Ok(o) => {
                match o.sym_type() {
                    crate::TokenType::Integer => {
                        elem_result_symbol.set_integeri(o.sym_integeri().abs());
                    }
                    crate::TokenType::Decimal => {
                        elem_result_symbol.set_decimal(o.sym_decimal().abs());
                    }
                    _ => {
                        return Err(crate::ErrorType::Integer);
                    }
                }

                Ok(elem_result_symbol)
            }
        }
    }

    /// Parses and executes the "am" function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_am(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let location: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                location = String::from(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let col_name: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                col_name = String::from(o.sym_string());
            }
        }
        let index: usize;
        if self
            .calc_mgr()
            .mgr()
            .map_col_names()
            .get_element_by_key(col_name.as_str())
        {
            index = self.calc_mgr().mgr().map_col_names().value();
        } else if col_name == "StrBal" {
            index = crate::COL_LABEL_STRBAL;
        } else if col_name == "EAR" {
            index = crate::COL_LABEL_EAR;
        } else if col_name == "PR" {
            index = crate::COL_LABEL_PR;
        } else if col_name == "DR" {
            index = crate::COL_LABEL_DR;
        } else {
            return Err(crate::ErrorType::Element);
        }
        
        let mut elem_result_symbol = ElemSymbol::new();

        if list_am_opt.is_none() || elem_balance_result_opt.is_none() {
            match CoreUtility::get_col_name(index) {
                crate::ColumnType::EventType
                | crate::ColumnType::Frequency
                | crate::ColumnType::StrBal => {
                    elem_result_symbol.set_string("");
                    return Ok(elem_result_symbol);
                }
                crate::ColumnType::Value
                | crate::ColumnType::Interest
                | crate::ColumnType::SlInterest
                | crate::ColumnType::IntOnInterest
                | crate::ColumnType::ValueToInterest
                | crate::ColumnType::ValueToPrincipal
                | crate::ColumnType::AccruedBalance
                | crate::ColumnType::Balance => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                    return Ok(elem_result_symbol);
                }
                crate::ColumnType::Date
                | crate::ColumnType::Intervals
                | crate::ColumnType::EndDate => {
                    elem_result_symbol.set_integer(0);
                    return Ok(elem_result_symbol);
                }
                crate::ColumnType::Ear | crate::ColumnType::Pr | crate::ColumnType::Dr => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                    return Ok(elem_result_symbol);
                }
                _ => {
                    return Err(crate::ErrorType::Element);
                }
            }
        }

        let list_am: &ListAmortization;
        match list_am_opt {
            None => {
                return Err(crate::ErrorType::Index);
            }
            Some(o) => {
                list_am = o;
            }
        }

        let elem_balance_result: &ElemBalanceResult;
        match elem_balance_result_opt {
            None => {
                return Err(crate::ErrorType::Index);
            }
            Some(o) => {
                elem_balance_result = o;
            }
        }

        let orig_index = list_am.index();
        let mut location_index = usize::MAX;
        let mut interest_days_in_year: usize = 360;

        if location == "Current" {
            location_index = orig_index;
        } else if location.starts_with("FirstStart=") {
            let text = CoreUtility::crop_letters(location.as_str(), 11);
            for index in 0..list_am.count() {
                if !list_am.get_element(index) {
                    break;
                }
                if list_am.event_type().starts_with(text) {
                    location_index = index;
                    break;
                }
            }
            list_am.get_element(orig_index);
        } else if location.starts_with("FirstMid=") {
            let text = CoreUtility::crop_letters(location.as_str(), 9);
            for index in 0..list_am.count() {
                if !list_am.get_element(index) {
                    break;
                }
                if list_am.event_type().contains(text) {
                    location_index = index;
                    break;
                }
            }
            list_am.get_element(orig_index);
        } else if location.starts_with("LastStart=") {
            let text = CoreUtility::crop_letters(location.as_str(), 10);
            for index in (0..list_am.count()).rev() {
                if !list_am.get_element(index) {
                    break;
                }
                if list_am.event_type().starts_with(text) {
                    location_index = index;
                    break;
                }
            }
            list_am.get_element(orig_index);
        } else if location.starts_with("LastMid=") {
            let text = CoreUtility::crop_letters(location.as_str(), 8);
            for index in (0..list_am.count()).rev() {
                if !list_am.get_element(index) {
                    break;
                }
                if list_am.event_type().contains(text) {
                    location_index = index;
                    break;
                }
            }
            list_am.get_element(orig_index);
        } else if location == "PrinPrev" {
            let mut index: i32 = (orig_index as i32) - 1;
            while index >= 0 {
                if !list_am.get_element(index as usize) {
                    break;
                }
                if list_am.elem_type() == crate::ExtensionType::PrincipalChange {
                    location_index = index as usize;
                    break;
                }
                index -= 1;
            }
            list_am.get_element(orig_index);
        } else if location == "PrinPrevStat" {
            let mut index: i32 = (orig_index as i32) - 1;
            while index >= 0 {
                if !list_am.get_element(index as usize) {
                    break;
                }
                if list_am.elem_type() == crate::ExtensionType::PrincipalChange
                    && list_am.elem_extension().pc_balance_statistics()
                {
                    location_index = index as usize;
                    break;
                }
                index -= 1;
            }
            list_am.get_element(orig_index);
        } else if location == "PrinNext" {
            let mut index = orig_index + 1;
            while index < list_am.count() {
                if !list_am.get_element(index) {
                    break;
                }
                if list_am.elem_type() == crate::ExtensionType::PrincipalChange {
                    location_index = index;
                    break;
                }
                index += 1;
            }
            list_am.get_element(orig_index);
        } else if location == "PrinNextStat" {
            let mut index = orig_index + 1;
            while index < list_am.count() {
                if !list_am.get_element(index) {
                    break;
                }
                if list_am.elem_type() == crate::ExtensionType::PrincipalChange
                    && list_am.elem_extension().pc_balance_statistics()
                {
                    location_index = index;
                    break;
                }
                index += 1;
            }
            list_am.get_element(orig_index);
        } else if location == "PrinFirst" {
            location_index = elem_balance_result.prin_first_index();
        } else if location == "PrinFirstStat" {
            location_index = elem_balance_result.prin_first_stat_index();
        } else if location == "PrinFirstPV" {
            location_index = elem_balance_result.prin_first_pv_index();
        } else if location == "PrinFirstStatPV" {
            location_index = elem_balance_result.prin_first_stat_pv_index();
        } else if location == "PrinLast" {
            location_index = elem_balance_result.prin_last_index();
        } else if location == "PrinLastStat" {
            location_index = elem_balance_result.prin_last_stat_index();
        } else if location == "CurFirstPV" {
            location_index = elem_balance_result.cur_first_pv_index();
        } else if location == "IntPrev" {
            let mut index: i32 = (orig_index as i32) - 1;
            while index >= 0 {
                if !list_am.get_element(index as usize) {
                    break;
                }
                if list_am.elem_type() == crate::ExtensionType::InterestChange {
                    location_index = index as usize;
                    break;
                }
                index -= 1;
            }
            if location_index == usize::MAX {
                location_index = orig_index;
            }
            list_am.get_element(orig_index);
        } else if location == "IntNext" {
            let mut index = orig_index + 1;
            while index < list_am.count() {
                if !list_am.get_element(index) {
                    break;
                }
                if list_am.elem_type() == crate::ExtensionType::InterestChange {
                    location_index = index;
                    break;
                }
                index += 1;
            }
            if location_index >= list_am.count() {
                location_index = orig_index;
            }
            list_am.get_element(orig_index);
        } else if location == "IntFirst" {
            location_index = elem_balance_result.int_first_index();
        } else if location == "IntLast" {
            location_index = elem_balance_result.int_last_index();
        } else {
            return Err(crate::ErrorType::Element);
        }

        list_am.get_element(location_index);

        match CoreUtility::get_col_name(index) {
            crate::ColumnType::EventType
            | crate::ColumnType::Frequency
            | crate::ColumnType::StrBal => {
                if list_am.index() == usize::MAX {
                    elem_result_symbol.set_string("");
                    return Ok(elem_result_symbol);
                }
            }
            crate::ColumnType::Value
            | crate::ColumnType::Interest
            | crate::ColumnType::SlInterest
            | crate::ColumnType::IntOnInterest
            | crate::ColumnType::ValueToInterest
            | crate::ColumnType::ValueToPrincipal
            | crate::ColumnType::AccruedBalance
            | crate::ColumnType::Balance => {
                if list_am.index() == usize::MAX {
                    elem_result_symbol.set_decimal(dec!(0.0));
                    return Ok(elem_result_symbol);
                }
            }
            crate::ColumnType::Date | crate::ColumnType::Intervals | crate::ColumnType::EndDate => {
                if list_am.index() == usize::MAX {
                    elem_result_symbol.set_integer(0);
                    return Ok(elem_result_symbol);
                }
            }
            crate::ColumnType::Ear | crate::ColumnType::Pr | crate::ColumnType::Dr => {
                if list_am.index() == usize::MAX
                    || list_am.elem_type() != crate::ExtensionType::InterestChange
                {
                    elem_result_symbol.set_decimal(dec!(0.0));
                    return Ok(elem_result_symbol);
                }
                interest_days_in_year = list_am.elem_extension().ic_days_in_year();
            }
            _ => {
                return Err(crate::ErrorType::Element);
            }
        }
        
        match CoreUtility::get_col_name(index) {
            crate::ColumnType::EventType => {
                elem_result_symbol.set_string(list_am.event_type());
            }
            crate::ColumnType::Date => {
                elem_result_symbol.set_integer(list_am.event_date());
            }
            crate::ColumnType::Value => {
                elem_result_symbol.set_decimal(list_am.value());
            }
            crate::ColumnType::Frequency => {
                let freq = CoreUtility::get_frequency_mnemonic(list_am.frequency());
                elem_result_symbol.set_string(freq.as_str());
            }
            crate::ColumnType::Intervals => {
                elem_result_symbol.set_integer(list_am.intervals());
            }
            crate::ColumnType::EndDate => {
                elem_result_symbol.set_integer(CoreUtility::date_new(
                    list_am.orig_date(),
                    list_am.event_date(),
                    list_am.frequency(),
                    list_am.intervals(),
                    list_am.elem_extension().extension_eom(),
                ));
            }
            crate::ColumnType::Interest => {
                elem_result_symbol.set_decimal(list_am.interest());
            }
            crate::ColumnType::SlInterest => {
                elem_result_symbol.set_decimal(list_am.sl_interest());
            }
            crate::ColumnType::IntOnInterest => {
                elem_result_symbol.set_decimal(list_am.interest() - list_am.sl_interest());
            }
            crate::ColumnType::ValueToInterest => {
                elem_result_symbol.set_decimal(list_am.value_to_interest());
            }
            crate::ColumnType::ValueToPrincipal => {
                elem_result_symbol.set_decimal(list_am.value_to_principal());
            }
            crate::ColumnType::AccruedBalance => {
                elem_result_symbol.set_decimal(list_am.acc_balance());
            }
            crate::ColumnType::Balance => {
                elem_result_symbol.set_decimal(list_am.balance());
            }
            crate::ColumnType::StrBal => {
                // StrBal
                let balance = self.calc_mgr().util_round(list_am.balance());
                if elem_balance_result.polarity() < 0 {
                    if balance > dec!(0.0) {
                        elem_result_symbol.set_string(
                            format!(
                                "+{}",
                                self.calc_mgr()
                                    .mgr()
                                    .list_locale()
                                    .format_currency(balance, self.decimal_digits)
                            )
                            .as_str(),
                        );
                    } else {
                        elem_result_symbol.set_string(
                            self.calc_mgr()
                                .mgr()
                                .list_locale()
                                .format_currency(balance.abs(), self.decimal_digits)
                                .as_str(),
                        );
                    }
                } else {
                    elem_result_symbol.set_string(
                        self.calc_mgr()
                            .mgr()
                            .list_locale()
                            .format_currency(balance, self.decimal_digits)
                            .as_str(),
                    );
                }
            }
            crate::ColumnType::Ear => {
                elem_result_symbol.set_decimal(
                    CoreUtility::rate_nar_to_ear(
                        list_am.value() / dec!(100.0),
                        list_am.frequency(),
                        interest_days_in_year,
                    ) * dec!(100.0),
                );
            }
            crate::ColumnType::Pr => {
                elem_result_symbol.set_decimal(
                    CoreUtility::rate_nar_to_pr(
                        list_am.value() / dec!(100.0),
                        list_am.frequency(),
                        interest_days_in_year,
                    ) * dec!(100.0),
                );
            }
            crate::ColumnType::Dr => {
                elem_result_symbol.set_decimal(
                    CoreUtility::rate_nar_to_dr(
                        list_am.value() / dec!(100.0),
                        interest_days_in_year,
                    ) * dec!(100.0),
                );
            }
            _ => {}
        }

        list_am.get_element(orig_index);
        Ok(elem_result_symbol)
    }

    /// Parses and executes the cashflow function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_cashflow(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let text: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                text = String::from(o.sym_string());
            }
        }

        let calc_mgr = self.calc_mgr();
        let cashflow = calc_mgr.list_cashflow();
        let elem_preferences_opt = cashflow.preferences();
        let mut elem_result_symbol = ElemSymbol::new();
        match text.as_str() {
            "Name" => {
                elem_result_symbol.set_string(cashflow.name());
            }
            "Locale" => match elem_preferences_opt {
                None => {
                    elem_result_symbol.set_string("");
                }
                Some(o) => {
                    elem_result_symbol.set_string(o.locale_str());
                }
            },
            "CrossCode" => match elem_preferences_opt {
                None => {
                    elem_result_symbol.set_string("");
                }
                Some(o) => {
                    elem_result_symbol.set_string(o.cross_rate_code());
                }
            },
            "Encoding" => match elem_preferences_opt {
                None => {
                    elem_result_symbol.set_string("");
                }
                Some(o) => {
                    elem_result_symbol.set_string(o.default_encoding());
                }
            },
            "Group" => match elem_preferences_opt {
                None => {
                    elem_result_symbol.set_string("");
                }
                Some(o) => {
                    elem_result_symbol.set_string(o.group());
                }
            },
            "YearStart" => match elem_preferences_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(o.fiscal_year_start());
                }
            },
            "DecDigits" => match elem_preferences_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(o.decimal_digits());
                }
            },
            "PrinTotal" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(o.prin_total());
                }
            },
            "PrinBefore" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(o.prin_total() - o.prin_present());
                }
            },
            "PrinAfter" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(o.prin_present());
                }
            },
            "IntTotal" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.interest_total());
                }
            },
            "IntBefore" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.interest_total() - o.interest_present());
                }
            },
            "IntAfter" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.interest_present());
                }
            },
            "SLIntTotal" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.sl_interest_total());
                }
            },
            "SLIntBefore" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.sl_interest_total() - o.sl_interest_present());
                }
            },
            "SLIntAfter" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.sl_interest_present());
                }
            },
            "AccBal" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.acc_balance());
                }
            },
            "Balance" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.balance());
                }
            },
            "BalDate" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(o.balance_date());
                }
            },
            "StrBal" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_string("");
                }
                Some(o) => {
                    let balance = self.calc_mgr().util_round(o.balance());
                    if o.polarity() < 0 {
                        if balance > dec!(0.0) {
                            elem_result_symbol.set_string(
                                format!(
                                    "+{}",
                                    self.calc_mgr()
                                        .mgr()
                                        .list_locale()
                                        .format_currency(balance, self.decimal_digits)
                                )
                                .as_str(),
                            );
                        } else {
                            elem_result_symbol.set_string(
                                self.calc_mgr()
                                    .mgr()
                                    .list_locale()
                                    .format_currency(balance.abs(), self.decimal_digits)
                                    .as_str(),
                            );
                        }
                    } else {
                        elem_result_symbol.set_string(
                            self.calc_mgr()
                                .mgr()
                                .list_locale()
                                .format_currency(balance, self.decimal_digits)
                                .as_str(),
                        );
                    }
                }
            },
            "PrinTotalDecr" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.prin_decrease());
                }
            },
            "PrinTotalIncr" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.prin_increase());
                }
            },
            "AuxActiveDecr" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.aux_active_decrease());
                }
            },
            "AuxActiveIncr" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.aux_active_increase());
                }
            },
            "AuxPassiveDecr" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.aux_passive_decrease());
                }
            },
            "AuxPassiveIncr" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_decimal(dec!(0.0));
                }
                Some(o) => {
                    elem_result_symbol.set_decimal(o.aux_passive_increase());
                }
            },
            "Polarity" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integeri(o.polarity());
                }
            },
            "AccBalSeen" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(if o.acc_balance_seen() { 1 } else { 0 });
                }
            },
            "RuleOf78Seen" => match elem_balance_result_opt {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integer(if o.rule_of_78_seen() { 1 } else { 0 });
                }
            },
            _ => {
                return Err(crate::ErrorType::Element);
            }
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the date difference function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_date_diff(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let date1: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                date1 = o.sym_integer();
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let date2: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                date2 = o.sym_integer();
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let frequency: crate::FrequencyType;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                frequency = CoreUtility::get_frequency(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let mut intervals: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                intervals = o.sym_integer();
                if intervals == 0 {
                    intervals = 1;
                }
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let eom: bool;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                eom = o.sym_integer() != 0;
            }
        }

        let mut elem_result_symbol = ElemSymbol::new();
        elem_result_symbol.set_integeri(CoreUtility::date_diff(
            date1, date2, frequency, intervals, eom,
        ));
        Ok(elem_result_symbol)
    }

    /// Return the most recent date that is not greater
    /// than the given date and is relative to the start
    /// of the fiscal year (month and day).
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_date_fiscal(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let orig_date: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                orig_date = o.sym_integer();
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let frequency: crate::FrequencyType;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                frequency = CoreUtility::get_frequency(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let mut intervals: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                intervals = o.sym_integer();
                if intervals == 0 {
                    intervals = 1;
                }
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let adjust: bool;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                adjust = o.sym_integer() != 0;
            }
        }
        let mut year = orig_date / 10000;
        if crate::SERIAL_BASE_YEAR * 10000 + (orig_date % 10000)
            < crate::SERIAL_BASE_YEAR * 10000 + self.fiscal_year_start
        {
            year -= 1;
        }
        let mut date = year * 10000 + self.fiscal_year_start;
        let mut prev_date = date;
        while date <= orig_date {
            prev_date = date;
            date = CoreUtility::date_new(date, date, frequency, intervals, true);
        }

        if adjust {
            prev_date = CoreUtility::date_newi(
                prev_date,
                prev_date,
                crate::FrequencyType::OneDay,
                -1,
                false,
            );
        }

        let mut elem_result_symbol = ElemSymbol::new();
        elem_result_symbol.set_integer(prev_date);

        Ok(elem_result_symbol)
    }

    /// Parses and executes the new date function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_date_new(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut date: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                date = o.sym_integer();
            }
        }
        let orig_date = date;
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let mut periods: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                periods = o.sym_integer();
                if periods == 0 {
                    periods = 1;
                }
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let frequency: crate::FrequencyType;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                frequency = CoreUtility::get_frequency(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let mut intervals: usize;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                intervals = o.sym_integer();
                if intervals == 0 {
                    intervals = 1;
                }
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let eom: bool;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                eom = o.sym_integer() != 0;
            }
        }
        while periods > 0 {
            date = CoreUtility::date_new(orig_date, date, frequency, intervals, eom);
            periods -= 1;
        }

        let mut elem_result_symbol = ElemSymbol::new();
        elem_result_symbol.set_integer(date);
        Ok(elem_result_symbol)
    }

    /// Parses and executes the current date function.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_date_now(&self) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol = ElemSymbol::new();
        elem_result_symbol.set_integer(CoreUtility::date_now());
        self.scanner_mut().rescan();
        Ok(elem_result_symbol)
    }

    /// Parses and executes the Decimal function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_decimal(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                elem_result_symbol.set_decimal(dec!(elem_result_symbol.sym_integer()));
            }
            crate::TokenType::String => {
                elem_result_symbol
                    .set_decimal(CoreUtility::parse_decimal(elem_result_symbol.sym_string()));
            }
            _ => {}
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the default function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_default(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, true);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        if elem_result_symbol.sym_type() != crate::TokenType::String {
            return Err(crate::ErrorType::String);
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let sym_table = self.symbol_table();
        let mut elem_symbol_opt = sym_table.get_symbol(elem_result_symbol.sym_string());
        if elem_symbol_opt.is_none() {
            return self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        }

        match elem_symbol_opt.as_mut() {
            None => {
                return Err(crate::ErrorType::Incomplete);
            }
            Some(o) => match o.sym_type() {
                crate::TokenType::Integer => {
                    elem_result_symbol.set_integer(o.sym_integer());
                }
                crate::TokenType::Decimal => {
                    elem_result_symbol.set_decimal(o.sym_decimal());
                }
                _ => {
                    elem_result_symbol.set_string(o.sym_string());
                }
            },
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the descriptor function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_descriptor(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let group: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                group = String::from(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let name: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                name = String::from(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let elem_type: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_type = String::from(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let code: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                code = String::from(o.sym_string());
            }
        }
        let mut value = CoreUtility::get_descriptor_value(
            Option::from(&self.list_descriptor_user),
            self.list_descriptor_cashflow.as_ref(),
            self.list_descriptor_event.as_ref(),
            group.as_str(),
            name.as_str(),
            elem_type.as_str(),
            code.as_str(),
        );

        if value.is_empty() {
            // Look up a generic (non-locale descriptor)
            value = CoreUtility::get_descriptor_value(
                Option::from(&self.list_descriptor_user),
                self.list_descriptor_cashflow.as_ref(),
                self.list_descriptor_event.as_ref(),
                group.as_str(),
                name.as_str(),
                "",
                "",
            );
        }

        let mut elem_result_symbol = ElemSymbol::new();
        elem_result_symbol.set_string(value.as_str());
        Ok(elem_result_symbol)
    }

    /// Parses and executes the format function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_format(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                elem_result_symbol
                    .set_string(format!("{}", elem_result_symbol.sym_integer()).as_str());
            }
            crate::TokenType::Decimal => {
                elem_result_symbol.set_string(
                    format!(
                        "{}",
                        CoreUtility::round(
                            elem_result_symbol.sym_decimal(),
                            self.decimal_digits,
                            crate::RoundType::Bankers
                        )
                    )
                    .as_str(),
                );
            }
            _ => {}
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the format currency function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_format_currency(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                elem_result_symbol.set_string(
                    self.calc_mgr()
                        .mgr()
                        .list_locale()
                        .format_currency(
                            dec!(elem_result_symbol.sym_integer()),
                            self.decimal_digits,
                        )
                        .as_str(),
                );
            }
            crate::TokenType::Decimal => {
                elem_result_symbol.set_string(
                    self.calc_mgr()
                        .mgr()
                        .list_locale()
                        .format_currency(elem_result_symbol.sym_decimal(), self.decimal_digits)
                        .as_str(),
                );
            }
            _ => {}
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the format date function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_format_date(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        elem_result_symbol.set_string(
            self.calc_mgr()
                .mgr()
                .list_locale()
                .format_date(elem_result_symbol.sym_integer())
                .as_str(),
        );
        Ok(elem_result_symbol)
    }

    /// Parses and executes the format number function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_format_number(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                elem_result_symbol.set_string(
                    self.calc_mgr()
                        .mgr()
                        .list_locale()
                        .format_integeri(elem_result_symbol.sym_integeri())
                        .as_str(),
                );
            }
            crate::TokenType::Decimal => {
                elem_result_symbol.set_string(
                    self.calc_mgr()
                        .mgr()
                        .list_locale()
                        .format_decimal(elem_result_symbol.sym_decimal())
                        .as_str(),
                );
            }
            _ => {}
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the if function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_if(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let elem_symbol: ElemSymbol;
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_symbol = o;
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        if elem_symbol.sym_integer() == 0 {
            let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_result_symbol = o;
                }
            }
        } else {
            let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false); // Scan past the symbol
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(_o) => {}
            }
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the integer function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_integer(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        match elem_result_symbol.sym_type() {
            crate::TokenType::Decimal => match elem_result_symbol.sym_decimal().to_i32() {
                None => {
                    elem_result_symbol.set_integer(0);
                }
                Some(o) => {
                    elem_result_symbol.set_integeri(o);
                }
            },
            crate::TokenType::String => {
                elem_result_symbol
                    .set_integeri(CoreUtility::parse_integeri(elem_result_symbol.sym_string()));
            }
            _ => {}
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the length function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_len(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        elem_result_symbol.set_integer(elem_result_symbol.sym_string().len());
        Ok(elem_result_symbol)
    }

    /// Parses and executes the lowercase function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_lowercase(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        elem_result_symbol.set_string(elem_result_symbol.sym_string().to_lowercase().as_str());
        Ok(elem_result_symbol)
    }

    /// Parses and executes the max function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_max(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let result1: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                result1 = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                result1 = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let result2: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                result2 = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                result2 = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }
        elem_result_symbol.set_decimal(if result1 >= result2 { result1 } else { result2 });
        Ok(elem_result_symbol)
    }

    /// Parses and executes the mid function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_mid(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let text = String::from(elem_result_symbol.sym_string());
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        let mut start: usize = if elem_result_symbol.sym_integer() > 0 {
            elem_result_symbol.sym_integer() - 1
        } else {
            0
        };
        if start == usize::MAX || start >= text.len() {
            start = 0;
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }
        let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        let mut length: usize = elem_result_symbol.sym_integer();
        if length == 0 || start + length >= text.len() {
            length = text.len() - start;
        }

        let rs = &text[(start)..(start + length + 1)];
        elem_result_symbol.set_string(rs);
        Ok(elem_result_symbol)
    }

    /// Parses and executes the min function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_min(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let result1: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                result1 = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                result1 = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let result2: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                result2 = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                result2 = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }
        elem_result_symbol.set_decimal(if result1 <= result2 { result1 } else { result2 });
        Ok(elem_result_symbol)
    }

    /// Parses and executes the parse function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_parse(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let text = String::from(elem_result_symbol.sym_string());
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let delims = String::from(elem_result_symbol.sym_string());
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        if self.scanner().get_type() != crate::TokenType::Alpha {
            return Err(crate::ErrorType::Alpha);
        }

        let tokens: Vec<&str>;
        let name: String;
        {
            let scanner = self.scanner();
            name = String::from(scanner.get_token());
            tokens = text.split(delims.as_str()).collect();
        }

        let token_count = tokens.len();
        elem_result_symbol.set_integer(token_count);
        for (index, token) in tokens.iter().enumerate() {
            let mut elem_symbol = ElemSymbol::new();
            elem_symbol.set_string(token);

            let text_fmt = format!("{}[{}]", name, index);
            self.symbol_table_mut()
                .add_symbol(text_fmt.as_str(), elem_symbol);
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the pr function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_pr(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let dresult: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                dresult = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                dresult = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }

        let mut dpr: Decimal = dec!(0.0);
        match list_am_opt {
            None => {}
            Some(o) => {
                dpr = CoreUtility::rate_nar_to_pr(
                    dresult / dec!(100.0),
                    o.frequency(),
                    crate::DEFAULT_DAYS_IN_YEAR,
                ) * dec!(100.0);
            }
        }

        elem_result_symbol.set_decimal(dpr);
        Ok(elem_result_symbol)
    }

    /// Parses and executes the replace function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_replace(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let text: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                text = String::from(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let cfrom: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                cfrom = String::from(o.sym_string());
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let cto: String;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                cto = String::from(o.sym_string());
            }
        }

        let rs = text.replace(cfrom.as_str(), cto.as_str());

        let mut elem_result_symbol = ElemSymbol::new();
        elem_result_symbol.set_string(rs.as_str());
        Ok(elem_result_symbol)
    }

    /// Parses and executes the round function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_round(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let dvalue: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                dvalue = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                dvalue = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }
        let mut digits: usize = self.decimal_digits;
        let mut round_ctrl = crate::RoundType::Bankers;

        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
            let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    elem_result_symbol = o;
                }
            }

            digits = elem_result_symbol.sym_integer();

            let token = self.scanner_mut().scan_token();
            if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',')
            {
                self.scanner_mut().scan_token();
                round_ctrl = if self.scanner().get_token() == "bias-up" {
                    crate::RoundType::BiasUp
                } else if self.scanner().get_token() == "bias-down" {
                    crate::RoundType::BiasDown
                } else if self.scanner().get_token() == "up" {
                    crate::RoundType::Up
                } else if self.scanner().get_token() == "truncate" {
                    crate::RoundType::Truncate
                } else {
                    crate::RoundType::Bankers
                };
            } else {
                self.scanner_mut().rescan();
            }
        } else {
            self.scanner_mut().rescan();
        }

        elem_result_symbol.set_decimal(CoreUtility::round(dvalue, digits, round_ctrl));
        Ok(elem_result_symbol)
    }

    /// Parses and executes the roundfraction function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_round_fraction(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let dvalue: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                dvalue = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                dvalue = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, false);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let dfraction: Decimal;
        match elem_result_symbol.sym_type() {
            crate::TokenType::Integer => {
                dfraction = dec!(elem_result_symbol.sym_integer());
            }
            crate::TokenType::Decimal => {
                dfraction = elem_result_symbol.sym_decimal();
            }
            _ => {
                return Err(crate::ErrorType::Decimal);
            }
        }
        let mut round_ctrl = crate::RoundType::Bankers;
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
            round_ctrl = if self.scanner().get_token() == "bias-up" {
                crate::RoundType::BiasUp
            } else if self.scanner().get_token() == "bias-down" {
                crate::RoundType::BiasDown
            } else if self.scanner().get_token() == "up" {
                crate::RoundType::Up
            } else if self.scanner().get_token() == "truncate" {
                crate::RoundType::Truncate
            } else {
                crate::RoundType::Bankers
            };
        } else {
            self.scanner_mut().rescan();
        }

        elem_result_symbol.set_decimal(CoreUtility::round_fraction(dvalue, dfraction, round_ctrl));
        Ok(elem_result_symbol)
    }

    /// Parses and executes the set function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_set(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        if self.scanner().get_type() != crate::TokenType::Alpha {
            return Err(crate::ErrorType::Alpha);
        }
        let mut text = String::from(self.scanner().get_token());

        let token = self.scanner_mut().scan_token();
        if !(token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with('['))
        {
            self.scanner_mut().rescan();
        } else {
            self.scanner_mut().scan_token();
            let result = self.get_expr_integer(list_am_opt, elem_balance_result_opt);
            match result {
                Err(e) => {
                    return Err(e);
                }
                Ok(o) => {
                    if !(self.scanner().get_type() == crate::TokenType::Punctuation
                        && self.scanner().get_token().starts_with(']'))
                    {
                        return Err(crate::ErrorType::RightBracket);
                    }
                    text = format!("{}[{}]", text, o.sym_integer());
                }
            }
        }
        let token = self.scanner_mut().scan_token();
        if token == crate::TokenType::Punctuation && self.scanner().get_token().starts_with(',') {
            self.scanner_mut().scan_token();
        }

        let elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, true);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        let mut sym_table = self.symbol_table_mut();
        let mut elem_symbol_opt = sym_table.get_symbol_mut(text.as_str());
        let mut symbol_new = ElemSymbol::new();
        let mut new_symbol: bool = false;

        if elem_symbol_opt.is_none() {
            elem_symbol_opt = Option::from(&mut symbol_new);
            new_symbol = true;
        }

        match elem_symbol_opt.as_mut() {
            None => {
                return Err(crate::ErrorType::Incomplete);
            }
            Some(o) => match elem_result_symbol.sym_type() {
                crate::TokenType::Integer => {
                    o.set_integer(elem_result_symbol.sym_integer());
                    symbol_new.set_integer(elem_result_symbol.sym_integer());
                }
                crate::TokenType::Decimal => {
                    o.set_decimal(elem_result_symbol.sym_decimal());
                    symbol_new.set_decimal(elem_result_symbol.sym_decimal());
                }
                crate::TokenType::String => {
                    o.set_string(elem_result_symbol.sym_string());
                    symbol_new.set_string(elem_result_symbol.sym_string());
                }
                _ => {}
            },
        }
        if new_symbol {
            sym_table.add_symbol(text.as_str(), symbol_new);
        }

        Ok(elem_result_symbol)
    }

    /// Parses and executes the trim function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_trim(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        let s = String::from(elem_result_symbol.sym_string().trim());
        elem_result_symbol.set_string(s.as_str());
        Ok(elem_result_symbol)
    }

    /// Parses and executes the type function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_type(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_symbol(list_am_opt, elem_balance_result_opt, true);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }
        if elem_result_symbol.sym_type() != crate::TokenType::String {
            return Err(crate::ErrorType::String);
        }
        let sym_table = self.symbol_table();
        let elem_symbol = sym_table.get_symbol(elem_result_symbol.sym_string());
        match elem_symbol.as_ref() {
            None => {
                elem_result_symbol.set_integer(crate::TokenType::Unknown as usize);
            }
            Some(o) => {
                elem_result_symbol.set_integer(o.sym_type() as usize);
            }
        }
        Ok(elem_result_symbol)
    }

    /// Parses and executes the uppercase function.
    ///
    /// # Arguments
    ///
    /// * `list_am_opt` - Amortization list.
    /// * `elem_balance_result_opt` - Balance results.
    ///
    /// # Return
    ///
    /// * Result symbol if successful, otherwise error code.

    pub fn function_uppercase(
        &self,
        list_am_opt: Option<&ListAmortization>,
        elem_balance_result_opt: Option<&ElemBalanceResult>,
    ) -> Result<ElemSymbol, crate::ErrorType> {
        let mut elem_result_symbol: ElemSymbol;
        let result = self.get_expr_string(list_am_opt, elem_balance_result_opt);
        match result {
            Err(e) => {
                return Err(e);
            }
            Ok(o) => {
                elem_result_symbol = o;
            }
        }

        elem_result_symbol.set_string(elem_result_symbol.sym_string().to_uppercase().as_str());
        Ok(elem_result_symbol)
    }

    /// Normalize the currently initialized expression.
    ///
    /// # Arguments
    ///
    /// * `new_line` - If true, format with a newline character between expressions.
    ///
    /// # Return
    ///
    /// * The normalized expression string.

    pub fn normalize_expression(&self, new_line: bool) -> String {
        let mut buf = String::from("");
        let mut level: usize = 0;

        loop {
            while self.scanner_mut().scan_token() != crate::TokenType::Unknown {
                let scan_type = self.scanner().get_type();

                if level == 0
                    && scan_type == crate::TokenType::Punctuation
                    && self.scanner().get_token().starts_with(',')
                {
                    break;
                }

                let mut text = String::from(self.scanner().get_token());
                let ch1: char = if text.is_empty() {
                    '\x00'
                } else {
                    text.chars().next().unwrap_or(' ')
                };

                match scan_type {
                    crate::TokenType::Punctuation => match ch1 {
                        '(' => {
                            buf.push(ch1);
                            level += 1;
                        }
                        ')' => {
                            buf.push(ch1);
                            if level > 0 {
                                level -= 1;
                            }
                        }
                        '[' | ']' => {
                            buf.push(ch1);
                        }
                        ',' => {
                            buf.push(ch1);
                            buf.push(' ');
                        }
                        _ => {
                            if (ch1 == '<' || ch1 == '>')
                                && self.scanner_mut().scan_token() != crate::TokenType::Unknown
                            {
                                let ch2: char =
                                    self.scanner().get_token().chars().next().unwrap_or(' ');
                                if ch2 == '=' || (ch1 == '<' && ch2 == '>') {
                                    text.push_str(self.scanner().get_token());
                                } else {
                                    self.scanner_mut().rescan();
                                }
                            }
                            if self
                                .calc_mgr()
                                .mgr()
                                .operators()
                                .get_element_by_key(text.as_str())
                            {
                                if !buf.is_empty() {
                                    buf.push(' ');
                                }
                                buf.push_str(text.as_str());
                                buf.push(' ');
                            } else {
                                buf.push_str(text.as_str());
                            }
                        }
                    },
                    crate::TokenType::Integer | crate::TokenType::Decimal => {
                        buf.push_str(text.as_str());
                    }
                    crate::TokenType::String => {
                        buf.push('"');
                        buf.push_str(text.as_str());
                        buf.push('"');
                    }
                    crate::TokenType::Alpha => {
                        if self
                            .calc_mgr()
                            .mgr()
                            .operators()
                            .get_element_by_key(text.as_str())
                        {
                            if !buf.is_empty() {
                                buf.push(' ');
                            }
                            buf.push_str(text.as_str());
                            buf.push(' ');
                        } else {
                            buf.push_str(text.as_str());
                        }
                    }
                    _ => {}
                }
            }

            if self.scanner().get_type() == crate::TokenType::Punctuation
                && self.scanner().get_token().starts_with(',')
            {
                buf.push(',');
                if new_line {
                    buf.push('\n');
                } else {
                    buf.push(' ');
                }
            }

            if self.scanner().get_type() == crate::TokenType::Unknown {
                break;
            }
        }
        buf
    }
}
