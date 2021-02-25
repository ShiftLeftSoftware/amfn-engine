/// The core enums.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Extension type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ExtensionType {
    /// Current value event type.
    CurrentValue = 1,
    /// Interest change event type.
    InterestChange = 2,
    /// Principal change event type.
    PrincipalChange = 3,
    /// Statistic value event type.
    StatisticValue = 4,
}

/// Principal type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PrincipalType {
    /// Current value event type.
    Increase = 1,
    /// Interest change event type.
    Decrease = 2,
    /// Principal change event type.
    Positive = 3,
    /// Statistic value event type.
    Negative = 4,
}

/// Method type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum MethodType {
    /// Actuarial method (Normal Rule).
    Actuarial = 0,
    /// Simple interest method (U.S. Rule).
    SimpleInterest = 1,
}

/// Round type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum RoundType {
    /// Round None.
    None = 0,
    /// Banker's rounding.
    Bankers = 1,
    /// Round Bias Up.
    BiasUp = 2,
    /// Round Bias Down.
    BiasDown = 3,
    /// Round Up.
    Up = 4,
    /// Truncate.
    Truncate = 5,
}

/// Day count type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum DayCountType {
    /// Periodic rate.
    Periodic = 1,
    /// Rule-of-78 (periodic rate).
    RuleOf78 = 2,
    /// Actual/<DaysInYear>.
    Actual = 3,
    /// Actual/Actual (ISMA).
    ActualActualISMA = 4,
    /// Actual/Actual (AFB).
    ActualActualAFB = 5,
    /// Actual/365L (ISDA).
    Actual365L = 6,
    /// 30/<DaysInYear>.
    Dc30 = 7,
    /// 30E/<DaysInYear>.
    Dc30E = 8,
    /// 30E+/<DaysInYear>.
    Dc30EP = 9,
}

/// Frequency type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FrequencyType {
    /// Not specified.
    None = 0,
    /// 1 year.
    OneYear = 1,
    /// 6 months.
    SixMonths = 2,
    /// 4 months.
    FourMonths = 3,
    /// 3 months.
    ThreeMonths = 4,
    /// 2 months.
    TwoMonths = 5,
    /// 1 month.
    OneMonth = 6,
    /// Half month (semi-monthly).
    HalfMonth = 7,
    /// Start of fixed frequencies
    StartFixed = 100,
    /// 4 weeks.
    FourWeeks = 101,
    /// 2 weeks (bi-weekly).
    TwoWeeks = 102,
    /// 1 week.
    OneWeek = 103,
    /// 1 day.
    OneDay = 104,
    /// Continuous.
    Continuous = 105,
}

/// Merge type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum MergeType {
    /// Do not merge interest events.
    IntNone = 0,
    /// Interest events from cashflow 1.
    Int1 = 1,
    /// Interest events from cashflow 2.
    Int2 = 2,
    /// Keep all interest events.
    IntAll = 3,
}

/// Token type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TokenType {
    /// Unknown token type.
    Unknown = 0,
    /// Integer token type.
    Integer = 1, // Symbol table type
    /// Decimal token type.
    Decimal = 2, // Symbol table type
    /// &str token type.
    String = 3, // Symbol table type
    /// Alpha token type.
    Alpha = 21,
    /// Punctuation token type.
    Punctuation = 22,
    /// Operator token type.
    Operator = 23,
    /// Left parenthesis token type.
    LeftParen = 24,
}

/// Format type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FormatType {
    /// Format - String.
    String = 0,
    /// Format - Date.
    Date = 1,
    /// Format - Integer.
    Integer = 2,
    /// Format - Decimal.
    Decimal = 3,
    /// Format - Currency.
    Currency = 4,
}

/// Operator type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum OperatorType {
    /// No operator.
    None = 0,
    /// And operator.
    And = 1,
    /// Or operator.
    Or = 2,
    /// Greater than operator.
    Greater = 3,
    /// Less than operator.
    Less = 4,
    /// Greater than or equal operator.
    GreaterEqual = 5,
    /// Less than or equal operator.
    LessEqual = 6,
    /// Equal operator.
    Equal = 7,
    /// Not equal operator.
    NotEqual = 8,
    /// Addition operator.
    Plus = 9,
    /// Subtraction operator.
    Minus = 10,
    /// Multiplication operator.
    Times = 11,
    /// Division operator.
    Divide = 12,
    /// Modulus operator.
    Modulus = 13,
    /// Exponent operator.
    Exponent = 14,
    /// Minus unary operator.
    UnaryMinus = 15,
    /// Not unary operator.
    UnaryNot = 16,
}

/// Function type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum FunctionType {
    /// No function.
    None = 0,
    /// Absolute value function.
    Abs = 1,
    /// Am function.
    Am = 2,
    /// Cashflow function.
    Cashflow = 3,
    /// Date difference function.
    DateDiff = 4,
    /// Fiscal date function.
    DateFiscal = 5,
    /// New date function.
    DateNew = 6,
    /// Current date function.
    DateNow = 7,
    /// Default function.
    Default = 8,
    /// Descriptor function.
    Descriptor = 9,
    /// Decimal function.
    Decimal = 10,
    /// Format function.
    Format = 11,
    /// Format currency function.
    FormatCurrency = 12,
    /// Format date function.
    FormatDate = 13,
    /// Format number function.
    FormatNumber = 14,
    /// If function.
    If = 15,
    /// Integer function.
    Integer = 16,
    /// Length function.
    Len = 17,
    /// Lowercase function.
    Lowercase = 18,
    /// Max function.
    Max = 19,
    /// Mid function.
    Mid = 20,
    /// Min function.
    Min = 21,
    /// Parse function.
    Parse = 22,
    /// Periodic rate (PR) function.
    Pr = 23,
    /// Replace function.
    Replace = 24,
    /// Round function.
    Round = 25,
    /// RoundFraction function.
    RoundFraction = 26,
    /// Set function.
    Set = 27,
    /// Trim function.
    Trim = 28,
    /// Type function.
    Type = 29,
    /// Uppercase function.
    Uppercase = 30,
}

/// Error type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ErrorType {
    /// No error.
    None = 0,
    /// Missing left parenthesis.
    LeftParen = 1,
    /// Missing right parenthesis.
    RightParen = 2,
    /// Operator expected.
    Operator = 3,
    /// Operand expected.
    Operand = 4,
    /// Invalid operator.
    InvalidOperator = 5,
    /// Invalid operand.
    InvalidOperand = 6,
    /// Invalid token.
    InvalidToken = 7,
    /// Missing operand.
    MissingOperand = 8,
    /// Missing right bracket.
    RightBracket = 9,
    /// Invalid symbol.
    InvalidSymbol = 10,
    /// Divide by zero.
    DivideByZero = 11,
    /// Integer expected.
    Integer = 12,
    /// Decimal expected.
    Decimal = 13,
    /// &str expected.
    String = 14,
    /// Date expected.
    Date = 15,
    /// Alpha expected.
    Alpha = 16,
    /// Invalid function.
    Function = 17,
    /// Invalid index value.
    Index = 18,
    /// Incomplete expression.
    Incomplete = 19,
    /// Cannot calculate interest.
    CalcInterest = 20,
    /// Cannot calculate periods.
    CalcPeriods = 21,
    /// Cannot calculate principal.
    CalcPrincipal = 22,
    /// Cashflow not selected.
    Cashflow = 23,
    /// Invalid cashflow name.
    CfName = 24,
    /// Element not selected.
    Element = 25,
    /// Invalid Json.
    Json = 26,
}

/// Element level enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ElemUpdateType {
    /// Descriptor update.
    Descriptor = 0,
    /// Parameter update.
    Parameter = 1,
    /// Preferences update.
    Preferences = 2,
    /// Cashflow update
    Cashflow = 3,
    /// Event update.
    Event = 4,
    /// Template update.
    Template = 5,
    /// Exchange rate update.
    ExchangeRate = 6,
}

/// Element level enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ElemLevelType {
    /// Engine level.
    Engine = 0,
    /// Cashflow level.
    Cashflow = 1,
    /// Event level.
    Event = 2,
}

/// Table type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TableType {
    /// Event table.
    Event = 1,
    /// Amortization table.
    Amortization = 2,
}

/// Column type enumeration.

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ColumnType {
    /// Column names - None.
    None = 0,
    /// Column names - Sequence.
    Sequence = 1,
    /// Column names - Event type.
    EventType = 2,
    /// Column names - Date.
    Date = 3,
    /// Column names - Date expression.
    DateExpr = 4,
    /// Column names - Sort.
    Sort = 5,
    /// Column names - Value.
    Value = 6,
    /// Column names - Value expression.
    ValueExpr = 7,
    /// Column names - Decrease.
    Decrease = 8,
    /// Column names - Increase.
    Increase = 9,
    /// Column names - Periods.
    Periods = 10,
    /// Column names - Periods expression.
    PeriodsExpr = 11,
    /// Column names - Skip periods.
    SkipPeriods = 12,
    /// Column names - Intervals.
    Intervals = 13,
    /// Column names - Frequency.
    Frequency = 14,
    /// Column names - End date.
    EndDate = 15,
    /// Column names - Descriptor list.
    DescriptorList = 16,
    /// Column names - Parameter list.
    ParameterList = 17,
    /// Column names - Componded interest.
    Interest = 18,
    /// Column names - Straight-line interest.
    SlInterest = 19,
    /// Column names - Interest on interest.
    IntOnInterest = 20,
    /// Column names - Value-to-interest.
    ValueToInterest = 21,
    /// Column names - Value-to-principal.
    ValueToPrincipal = 22,
    /// Column names - Accrued-balance.
    AccruedBalance = 23,
    /// Column names - Balance.
    Balance = 24,
    /// Column names - Event-name.
    EventName = 25,
    /// Column names - Next-name.
    NextName = 26,
    /// Column names - StrBal
    StrBal = 1000,
    /// Column names - EAR
    Ear = 1001,
    /// Column names - PR
    Pr = 1002,
    /// Column names - DR
    Dr = 1003,
}
