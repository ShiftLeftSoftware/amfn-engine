//! The core manager element.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::{Ref, RefCell, RefMut};

use super::{ListKey, ListLocale};

pub struct CoreManager {
    /// Table of event type mappings.
    map_event_type: ListKey,
    /// Table of principal type mappings.
    map_principal_type: ListKey,
    /// Table of interest method mappings.
    map_interest_method: ListKey,
    /// Table of interest rounding mappings.
    map_interest_rounding: ListKey,
    /// Table of day count basis mappings.
    map_day_count_basis: ListKey,
    /// Table of frequency mappings.
    map_frequency: ListKey,
    /// Table of parameter types.
    map_parameter_type: ListKey,
    /// Table of operators.
    operators: ListKey,
    /// Table of functions.
    functions: ListKey,
    /// Table of error mappings.
    map_error: ListKey,
    /// Table of column names.
    map_col_names: ListKey,

    /// List of locales.
    list_locale: RefCell<ListLocale>,
}

/// The core manager default implementation.

impl Default for CoreManager {
    /// Create and return a new core manager.
    ///
    /// # Return
    ///
    /// * See description.

    fn default() -> Self {
        CoreManager::new()
    }
}

/// The core manager implementation.

impl CoreManager {
    /// Create and return a new core manager.
    ///
    /// # Return
    ///
    /// * See description.    

    pub fn new() -> CoreManager {
        let mut mgr = CoreManager {
            map_event_type: ListKey::new(),
            map_principal_type: ListKey::new(),
            map_interest_method: ListKey::new(),
            map_interest_rounding: ListKey::new(),
            map_day_count_basis: ListKey::new(),
            map_frequency: ListKey::new(),
            map_parameter_type: ListKey::new(),
            operators: ListKey::new(),
            functions: ListKey::new(),
            map_error: ListKey::new(),
            map_col_names: ListKey::new(),
            list_locale: RefCell::new(ListLocale::new()),
        };
        // Initialize engine lists

        mgr.map_event_type.add_key(
            "Event_Type_Current_Value",
            crate::ExtensionType::CurrentValue as usize,
            0,
        );

        mgr.map_event_type.add_key(
            "Event_Type_Interest_Change",
            crate::ExtensionType::InterestChange as usize,
            0,
        );

        mgr.map_event_type.add_key(
            "Event_Type_Principal_Change",
            crate::ExtensionType::PrincipalChange as usize,
            0,
        );

        mgr.map_event_type.add_key(
            "Event_Type_Statistic_Value",
            crate::ExtensionType::StatisticValue as usize,
            0,
        );

        mgr.map_principal_type.add_key(
            "Principal_Type_Increase",
            crate::PrincipalType::Increase as usize,
            0,
        );

        mgr.map_principal_type.add_key(
            "Principal_Type_Decrease",
            crate::PrincipalType::Decrease as usize,
            0,
        );

        mgr.map_principal_type.add_key(
            "Principal_Type_Positive",
            crate::PrincipalType::Positive as usize,
            0,
        );

        mgr.map_principal_type.add_key(
            "Principal_Type_Negitive",
            crate::PrincipalType::Negative as usize,
            0,
        );

        mgr.map_interest_method.add_key(
            "Method_Actuarial",
            crate::MethodType::Actuarial as usize,
            0,
        );

        mgr.map_interest_method.add_key(
            "Method_Simple_Interest",
            crate::MethodType::SimpleInterest as usize,
            0,
        );

        mgr.map_interest_rounding
            .add_key("Rounding_None", crate::RoundType::None as usize, 0);

        mgr.map_interest_rounding.add_key(
            "Rounding_Bankers",
            crate::RoundType::Bankers as usize,
            0,
        );

        mgr.map_interest_rounding
            .add_key("Rounding_Bias_Up", crate::RoundType::BiasUp as usize, 0);

        mgr.map_interest_rounding.add_key(
            "Rounding_Bias_Down",
            crate::RoundType::BiasDown as usize,
            0,
        );

        mgr.map_interest_rounding
            .add_key("Rounding_Up", crate::RoundType::Up as usize, 0);

        mgr.map_interest_rounding.add_key(
            "Rounding_Truncate",
            crate::RoundType::Truncate as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_Periodic",
            crate::DayCountType::Periodic as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_Rule_Of_78",
            crate::DayCountType::RuleOf78 as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_Actual",
            crate::DayCountType::Actual as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_Actual_Actual_ISMA",
            crate::DayCountType::ActualActualISMA as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_Actual_Actual_AFB",
            crate::DayCountType::ActualActualAFB as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_Actual_365L",
            crate::DayCountType::Actual365L as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_30",
            crate::DayCountType::Dc30 as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_30E",
            crate::DayCountType::Dc30E as usize,
            0,
        );

        mgr.map_day_count_basis.add_key(
            "Day_Count_Basis_30EP",
            crate::DayCountType::Dc30EP as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_1_Year",
            crate::FrequencyType::OneYear as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_6_Months",
            crate::FrequencyType::SixMonths as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_4_Months",
            crate::FrequencyType::FourMonths as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_3_Months",
            crate::FrequencyType::ThreeMonths as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_2_Months",
            crate::FrequencyType::TwoMonths as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_1_Month",
            crate::FrequencyType::OneMonth as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_Half_Month",
            crate::FrequencyType::HalfMonth as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_4_Weeks",
            crate::FrequencyType::FourWeeks as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_2_Weeks",
            crate::FrequencyType::TwoWeeks as usize,
            0,
        );

        mgr.map_frequency.add_key(
            "Frequency_1_Week",
            crate::FrequencyType::OneWeek as usize,
            0,
        );

        mgr.map_frequency
            .add_key("Frequency_1_Day", crate::FrequencyType::OneDay as usize, 0);

        mgr.map_frequency.add_key(
            "Frequency_Continuous",
            crate::FrequencyType::Continuous as usize,
            0,
        );

        mgr.map_parameter_type.add_key(
            "Dialog_Type_Integer",
            crate::TokenType::Integer as usize,
            0,
        );

        mgr.map_parameter_type
            .add_key("Dialog_Type_Float", crate::TokenType::Decimal as usize, 0);

        mgr.map_parameter_type
            .add_key("Dialog_Type_String", crate::TokenType::String as usize, 0);

        mgr.operators
            .add_key("and", crate::OperatorType::And as usize, 0);

        mgr.operators
            .add_key("or", crate::OperatorType::Or as usize, 0);

        mgr.operators
            .add_key(">", crate::OperatorType::Greater as usize, 1);

        mgr.operators
            .add_key("<", crate::OperatorType::Less as usize, 1);

        mgr.operators
            .add_key(">=", crate::OperatorType::GreaterEqual as usize, 1);

        mgr.operators
            .add_key("<=", crate::OperatorType::LessEqual as usize, 1);

        mgr.operators
            .add_key("=", crate::OperatorType::Equal as usize, 1);

        mgr.operators
            .add_key("<>", crate::OperatorType::NotEqual as usize, 1);

        mgr.operators
            .add_key("+", crate::OperatorType::Plus as usize, 2);

        mgr.operators
            .add_key("-", crate::OperatorType::Minus as usize, 2);

        mgr.operators
            .add_key("*", crate::OperatorType::Times as usize, 3);

        mgr.operators
            .add_key("/", crate::OperatorType::Divide as usize, 3);

        mgr.operators
            .add_key("mod", crate::OperatorType::Modulus as usize, 3);

        mgr.operators
            .add_key("exp", crate::OperatorType::Exponent as usize, 4);

        mgr.operators
            .add_key("~", crate::OperatorType::UnaryMinus as usize, 5);

        mgr.operators
            .add_key("not", crate::OperatorType::UnaryNot as usize, 5);
        mgr.functions
            .add_key("abs(number)", crate::FunctionType::Abs as usize, 0);

        mgr.functions.add_key(
            "am(\"location\", crate::\"option\")",
            crate::FunctionType::Am as usize,
            0,
        );

        mgr.functions.add_key(
            "cashflow(\"option\")",
            crate::FunctionType::Cashflow as usize,
            0,
        );

        mgr.functions.add_key(
            "datediff(date1, date2, \"frequency\", crate::intervals, eom)",
            crate::FunctionType::DateDiff as usize,
            0,
        );

        mgr.functions.add_key(
            "datefiscal(date, \"frequency\", crate::intervals, adjust)",
            crate::FunctionType::DateFiscal as usize,
            0,
        );

        mgr.functions.add_key(
            "datenew(date, periods, \"frequency\", crate::intervals, eom)",
            crate::FunctionType::DateNew as usize,
            0,
        );

        mgr.functions
            .add_key("datenow()", crate::FunctionType::DateNow as usize, 0);

        mgr.functions.add_key(
            "default(variable, value)",
            crate::FunctionType::Default as usize,
            0,
        );

        mgr.functions.add_key(
            "descriptor(\"group\", crate::\"name\", crate::\"type\", crate::\"code\")",
            crate::FunctionType::Descriptor as usize,
            0,
        );

        mgr.functions
            .add_key("decimal(value)", crate::FunctionType::Decimal as usize, 0);

        mgr.functions
            .add_key("format(value)", crate::FunctionType::Format as usize, 0);

        mgr.functions.add_key(
            "formatcurrency(value)",
            crate::FunctionType::FormatCurrency as usize,
            0,
        );

        mgr.functions.add_key(
            "formatdate(date)",
            crate::FunctionType::FormatDate as usize,
            0,
        );

        mgr.functions.add_key(
            "formatnumber(value)",
            crate::FunctionType::FormatNumber as usize,
            0,
        );

        mgr.functions.add_key(
            "if(condition, result-if-true, result-if-false)",
            crate::FunctionType::If as usize,
            0,
        );

        mgr.functions
            .add_key("integer(value)", crate::FunctionType::Integer as usize, 0);

        mgr.functions
            .add_key("len(\"string\")", crate::FunctionType::Len as usize, 0);

        mgr.functions.add_key(
            "lowercase(\"string\")",
            crate::FunctionType::Lowercase as usize,
            0,
        );

        mgr.functions.add_key(
            "max(number1, number2)",
            crate::FunctionType::Max as usize,
            0,
        );

        mgr.functions.add_key(
            "mid(\"string\", crate::start, end)",
            crate::FunctionType::Mid as usize,
            0,
        );

        mgr.functions.add_key(
            "min(number1, number2)",
            crate::FunctionType::Min as usize,
            0,
        );

        mgr.functions.add_key(
            "parse(\"string\", crate::\"delimiters\", crate::name)",
            crate::FunctionType::Parse as usize,
            0,
        );

        mgr.functions
            .add_key("pr(number)", crate::FunctionType::Pr as usize, 0);

        mgr.functions.add_key(
            "replace(\"string\", crate::\"character-from\", crate::\"character-to\")",
            crate::FunctionType::Replace as usize,
            0,
        );

        mgr.functions
            .add_key("round(number)", crate::FunctionType::Round as usize, 0);

        mgr.functions.add_key(
            "roundfraction(number1, number2)",
            crate::FunctionType::RoundFraction as usize,
            0,
        );

        mgr.functions
            .add_key("set(variable, value)", crate::FunctionType::Set as usize, 0);

        mgr.functions
            .add_key("trim(\"string\")", crate::FunctionType::Trim as usize, 0);

        mgr.functions
            .add_key("type(variable)", crate::FunctionType::Type as usize, 0);

        mgr.functions.add_key(
            "uppercase(\"string\")",
            crate::FunctionType::Uppercase as usize,
            0,
        );

        mgr.map_error
            .add_key("Error_Left_Paren", crate::ErrorType::LeftParen as usize, 0);

        mgr.map_error.add_key(
            "Error_Right_Paren",
            crate::ErrorType::RightParen as usize,
            0,
        );

        mgr.map_error
            .add_key("Error_Operator", crate::ErrorType::Operator as usize, 0);

        mgr.map_error
            .add_key("Error_Operand", crate::ErrorType::Operand as usize, 0);

        mgr.map_error.add_key(
            "Error_Invalid_Operator",
            crate::ErrorType::InvalidOperator as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Invalid_Operand",
            crate::ErrorType::InvalidOperand as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Invalid_Token",
            crate::ErrorType::InvalidToken as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Missing_Operand",
            crate::ErrorType::MissingOperand as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Right_Bracket",
            crate::ErrorType::RightBracket as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Invalid_Symbol",
            crate::ErrorType::InvalidSymbol as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Divide_By_Zero",
            crate::ErrorType::DivideByZero as usize,
            0,
        );

        mgr.map_error
            .add_key("Error_Integer", crate::ErrorType::Integer as usize, 0);

        mgr.map_error
            .add_key("Error_Float", crate::ErrorType::Decimal as usize, 0);

        mgr.map_error
            .add_key("Error_String", crate::ErrorType::String as usize, 0);

        mgr.map_error
            .add_key("Error_Date", crate::ErrorType::Date as usize, 0);

        mgr.map_error
            .add_key("Error_Alpha", crate::ErrorType::Alpha as usize, 0);

        mgr.map_error
            .add_key("Error_Function", crate::ErrorType::Function as usize, 0);

        mgr.map_error
            .add_key("Error_Index", crate::ErrorType::Index as usize, 0);

        mgr.map_error
            .add_key("Error_Incomplete", crate::ErrorType::Incomplete as usize, 0);

        mgr.map_error.add_key(
            "Error_Calculate_Interest",
            crate::ErrorType::CalcInterest as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Calculate_Periods",
            crate::ErrorType::CalcPeriods as usize,
            0,
        );

        mgr.map_error.add_key(
            "Error_Calculate_Principal",
            crate::ErrorType::CalcPrincipal as usize,
            0,
        );

        mgr.map_error
            .add_key("Error_Cashflow", crate::ErrorType::Cashflow as usize, 0);

        mgr.map_error
            .add_key("Error_CFName", crate::ErrorType::CfName as usize, 0);

        mgr.map_error
            .add_key("Error_Element", crate::ErrorType::Element as usize, 0);

        mgr.map_error
            .add_key("Error_Json", crate::ErrorType::Json as usize, 0);

        mgr.map_col_names
            .add_key("Sequence", crate::ColumnType::Sequence as usize, 0);

        mgr.map_col_names.add_key(
            "Type",
            crate::ColumnType::EventType as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "Date",
            crate::ColumnType::Date as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "Date-expr",
            crate::ColumnType::DateExpr as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );

        mgr.map_col_names.add_key(
            "Sort",
            crate::ColumnType::Sort as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );

        mgr.map_col_names.add_key(
            "Value",
            crate::ColumnType::Value as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "Value-expr",
            crate::ColumnType::ValueExpr as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );

        mgr.map_col_names
            .add_key("Decrease", crate::ColumnType::Decrease as usize, 0);

        mgr.map_col_names
            .add_key("Increase", crate::ColumnType::Increase as usize, 0);

        mgr.map_col_names.add_key(
            "Periods",
            crate::ColumnType::Periods as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "Periods-expr",
            crate::ColumnType::PeriodsExpr as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );

        mgr.map_col_names.add_key(
            "Skip-periods",
            crate::ColumnType::SkipPeriods as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "Intervals",
            crate::ColumnType::Intervals as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "Frequency",
            crate::ColumnType::Frequency as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "End-date",
            crate::ColumnType::EndDate as usize,
            crate::MAPCOLNAMES_EDITABLE,
        );

        mgr.map_col_names.add_key(
            "Descriptor-list",
            crate::ColumnType::DescriptorList as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );

        mgr.map_col_names.add_key(
            "Parameter-list",
            crate::ColumnType::ParameterList as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );

        mgr.map_col_names
            .add_key("Interest", crate::ColumnType::Interest as usize, 0);

        mgr.map_col_names
            .add_key("SLInterest", crate::ColumnType::SlInterest as usize, 0);

        mgr.map_col_names.add_key(
            "IntOnInterest",
            crate::ColumnType::IntOnInterest as usize,
            0,
        );

        mgr.map_col_names.add_key(
            "Value-to-interest",
            crate::ColumnType::ValueToInterest as usize,
            0,
        );

        mgr.map_col_names.add_key(
            "Value-to-principal",
            crate::ColumnType::ValueToPrincipal as usize,
            0,
        );

        mgr.map_col_names.add_key(
            "Accrued-balance",
            crate::ColumnType::AccruedBalance as usize,
            crate::MAPCOLNAMES_EMPTY,
        );

        mgr.map_col_names
            .add_key("Balance", crate::ColumnType::Balance as usize, 0);

        mgr.map_col_names.add_key(
            "Event-name",
            crate::ColumnType::EventName as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );
        mgr.map_col_names.add_key(
            "Next-name",
            crate::ColumnType::NextName as usize,
            crate::MAPCOLNAMES_EDITABLE | crate::MAPCOLNAMES_EXCLUDE,
        );

        mgr
    }

    /// Get the map event type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_event_type(&self) -> &ListKey {
        &self.map_event_type
    }

    /// Get the map principal type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_principal_type(&self) -> &ListKey {
        &self.map_principal_type
    }

    /// Get the map interest method.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_interest_method(&self) -> &ListKey {
        &self.map_interest_method
    }

    /// Get the map interest rounding.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_interest_rounding(&self) -> &ListKey {
        &self.map_interest_rounding
    }

    /// Get the map day count basis.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_day_count_basis(&self) -> &ListKey {
        &self.map_day_count_basis
    }

    /// Get the map frequency.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_frequency(&self) -> &ListKey {
        &self.map_frequency
    }

    /// Get the map parameter type.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_parameter_type(&self) -> &ListKey {
        &self.map_parameter_type
    }

    /// Get the operators.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn operators(&self) -> &ListKey {
        &self.operators
    }

    /// Get the functions.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn functions(&self) -> &ListKey {
        &self.functions
    }

    /// Get the map error.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_error(&self) -> &ListKey {
        &self.map_error
    }

    /// Get the map col names.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn map_col_names(&self) -> &ListKey {
        &self.map_col_names
    }

    /// Get the locale list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_locale(&self) -> Ref<ListLocale> {
        self.list_locale.borrow()
    }

    /// Get the locale list.
    ///
    /// # Return
    ///
    /// * See description.

    pub fn list_locale_mut(&self) -> RefMut<ListLocale> {
        self.list_locale.borrow_mut()
    }

    /// Append to the list locale.
    ///
    /// # Arguments
    ///
    /// * `list_locale` - See description.

    pub fn append_list_locale(&mut self, mut list_locale: ListLocale) {
        let ll = list_locale.list_mut();

        loop {
            match ll.pop() {
                None => { break; }
                Some(o) => { self.list_locale.borrow_mut().list_mut().push(o); }
            }
        }
    }

    /// Set the list locale.
    ///
    /// # Arguments
    ///
    /// * `list_locale` - See description.

    pub fn set_list_locale(&mut self, list_locale: ListLocale) {
        self.list_locale = RefCell::new(list_locale);
    }
}