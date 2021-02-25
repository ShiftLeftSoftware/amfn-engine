//! The AmFn token scanner.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub struct CalcScan {
    /// The expression to scan.
    expr: Vec<char>,
    /// The index past the last token scanned.
    expr_index: usize,
    /// The index of the start of the last token (for rescan).
    last_index: usize,

    /// The last token scanned.
    token: String,
    /// The type of the last token scanned.
    token_type: crate::TokenType,

    /// The accumulation buffer for the current token being scanned.
    accum_token: String,
}

/// The AmFn token scanner implementation.

impl CalcScan {
    /// Create and return a new scanner.
    ///
    /// # Arguments
    ///
    /// * `expr_param` - The expression to scan.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn new(expr_param: &str) -> CalcScan {
        return CalcScan {
            expr: expr_param.chars().collect(),
            expr_index: 0,
            last_index: 0,
            token: String::from(""),
            token_type: crate::TokenType::Unknown,

            accum_token: String::from(""),
        };
    }

    /// Get the type of the last token scanned.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_type(&self) -> crate::TokenType {
        self.token_type
    }

    /// Get the text of the last token scanned.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn get_token(&self) -> &str {
        self.token.as_str()
    }

    /// Initialize the instance variables with a new expression.
    ///
    /// # Arguments
    ///
    /// * `expr_param` - The expression to scan.

    pub fn init_scan(&mut self, expr_param: &str) {
        self.expr = expr_param.chars().collect();
        self.expr_index = 0;
        self.last_index = 0;

        self.token = String::from("");
        self.token_type = crate::TokenType::Unknown;

        self.accum_token = String::from("");
    }

    /// Sets the current expression index to the starting
    /// index of the last token scanned.

    pub fn rescan(&mut self) {
        self.expr_index = self.last_index;

        self.token = String::from("");
        self.token_type = crate::TokenType::Unknown;

        self.accum_token = String::from("");
    }

    /// Scan the next token in the expression updating
    /// all applicable instance variables.
    ///
    /// # Return
    ///
    /// * The type of the token just scanned.

    pub fn scan_token(&mut self) -> crate::TokenType {
        let mut ch: char;

        self.token_type = crate::TokenType::Unknown;

        while self.expr_index < self.expr.len() {
            ch = self.expr[self.expr_index];

            if !char::is_whitespace(ch) {
                break;
            }

            self.expr_index += 1;
        }

        self.last_index = self.expr_index;

        if self.expr_index >= self.expr.len() {
            self.token = String::from("");
            return self.token_type;
        }

        let mut ch = self.expr[self.expr_index];
        if ch == '"' {
            self.token_type = crate::TokenType::String;
            self.expr_index += 1;

            while self.expr_index < self.expr.len() {
                ch = self.expr[self.expr_index];

                if ch == '"' {
                    self.expr_index += 1;
                    break;
                }

                self.accum_token.push(ch);
                self.expr_index += 1;
            }
        } else if char::is_alphabetic(ch) {
            self.token_type = crate::TokenType::Alpha;

            while self.expr_index < self.expr.len() {
                ch = self.expr[self.expr_index];

                if !(char::is_alphanumeric(ch) || ch == '_') {
                    break;
                }

                self.accum_token.push(ch);
                self.expr_index += 1;
            }
        } else if char::is_numeric(ch) || ch == '.' {
            self.token_type = crate::TokenType::Integer;
            ch = '\0';

            while self.expr_index < self.expr.len() {
                ch = self.expr[self.expr_index];
                if !char::is_numeric(ch) {
                    break;
                }
                self.accum_token.push(ch);
                self.expr_index += 1;
            }

            if self.expr_index < self.expr.len() && ch == '.' {
                self.token_type = crate::TokenType::Decimal;
                self.accum_token.push(ch);
                self.expr_index += 1;

                while self.expr_index < self.expr.len() {
                    ch = self.expr[self.expr_index];
                    if !char::is_numeric(ch) {
                        break;
                    }
                    self.accum_token.push(ch);
                    self.expr_index += 1;
                }
            }
        } else {
            self.token_type = crate::TokenType::Punctuation;
            self.accum_token.push(ch);
            self.expr_index += 1;
        }

        self.token = String::from(self.accum_token.as_str());
        let length = self.accum_token.len();

        if length > 0 {
            self.accum_token.drain(..length);
        }

        self.token_type
    }

    /// Determine if the next item in the expression is a sub-expression.
    /// If the next item in the expression is a sub-expression, the
    /// next token will be the complete sub-expression with a type of string.
    ///
    /// # Return
    ///
    /// * True if a subexpression is detected, otherwise false.

    pub fn scan_subexpression(&mut self) -> bool {
        let mut end_index: usize = self.last_index;
        let mut paren_level: usize = 0;
        let mut bracket_level: usize = 0;
        let mut length;
        let mut exit_loop: bool = false;
        let mut is_expression: bool = false;
        let mut ignore_chars: bool = false;

        while !exit_loop && end_index < self.expr.len() {
            let ch = self.expr[end_index];
            let is_letter_or_digit: bool = char::is_alphanumeric(ch);
            length = self.accum_token.len();
            if !is_letter_or_digit && length > 0 {
                let stemp: String = self.accum_token.to_lowercase();

                if stemp == "and"
                    || stemp == "or"
                    || stemp == "mod"
                    || stemp == "exp"
                    || stemp == "not"
                {
                    is_expression = true;
                }

                self.accum_token.drain(..length);
            }
            if ignore_chars {
                if ch == '"' {
                    ignore_chars = false;
                }
                end_index += 1;
                continue;
            }
            match ch {
                '(' => {
                    if !is_expression && end_index == self.last_index {
                        is_expression = true;
                    }
                    paren_level += 1;
                }
                ')' => {
                    if paren_level == 0 && bracket_level == 0 {
                        exit_loop = true;
                    } else {
                        paren_level -= 1;
                    }
                }
                '[' => {
                    bracket_level += 1;
                }
                ']' => {
                    if paren_level == 0 && bracket_level == 0 {
                        exit_loop = true;
                    } else {
                        bracket_level -= 1;
                    }
                }
                ',' => {
                    if paren_level == 0 && bracket_level == 0 {
                        exit_loop = true;
                    }
                }
                '\"' => {
                    ignore_chars = true;
                }
                '-' => {
                    if !is_expression && end_index > self.last_index {
                        is_expression = true;
                    }
                }
                '.' | '_' => {}
                _ => {
                    if !is_expression {
                        if !is_letter_or_digit && paren_level == 0 && bracket_level == 0 {
                            is_expression = true;
                        } else if is_letter_or_digit {
                            self.accum_token.push(ch);
                        }
                    }
                }
            }
            if !exit_loop {
                end_index += 1;
            }
        }

        length = self.accum_token.len();
        if length > 0 {
            self.accum_token.drain(..length);
        }

        if !is_expression {
            return false;
        }

        self.expr_index = end_index;
        self.token_type = crate::TokenType::String;
        self.token.clear();
        for index in self.last_index..self.expr_index {
            self.token.push(self.expr[index]);
        }

        true
    }
}
