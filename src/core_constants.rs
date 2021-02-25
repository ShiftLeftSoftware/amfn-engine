/// The core constants.
// Copyright (c) 2021 ShiftLeft Software
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Serialization UID. 
pub const SERIAL_UID: usize = 1; 
/// The application name. 
pub const APP_NAME: &str = "Amortization Functions (AmFn)";
/// Version major string. 
pub const ID_VERSION_MAJOR: &str = "1";
/// Version minor string. 
pub const ID_VERSION_MINOR: &str = "0";
/// Version message. 
pub const ID_VERSION: &str = "Version: 1.0";
/// Copyright message. 
pub const ID_COPYRIGHT: &str = 
    "Copyright \u{00a9} 2021 ShiftLeft Software.\nAll rights reserved.";
/// Empty text. 
pub const EMPTY_TEXT: &str = "<<>>";
/// Null text. 
pub const NULL_TEXT: &str = "<>";
/// Empty field display. 
pub const EMPTY_DISPLAY: &str = "";
/// Error prefix. 
pub const ERROR_PREFIX: &str = ">> ";

/// Default encoding. 
pub const DEFAULT_ENCODING: &str = "utf-8";
/// Default days in year. 
pub const DEFAULT_DAYS_IN_YEAR: usize = 360;
/// Default physical year start. 
pub const DEFAULT_FISCAL_YEAR_START: usize = 101; // MMDD
/// Base year for date/serial conversions. 
pub const SERIAL_BASE_YEAR: usize = 1900;
/// Base year for current century validation. 
pub const SERIAL_BASE_CENTURY: usize = 2000;
/// Default decimal digits. 
pub const DEFAULT_DECIMAL_DIGITS: usize = 2;
/// Default round balance. 
pub const DEFAULT_ROUND_BALANCE: bool = false;
/// Default combine principal. 
pub const DEFAULT_COMBINE_PRINCIPAL: bool = true;
/// Default compress descriptor. 
pub const DEFAULT_COMPRESS_DESCRIPTOR: bool = false;
/// Default omit statistic events. 
pub const DEFAULT_OMIT_STATISTIC_EVENTS: bool = true;
/// Calculate the overall APR for a cashflow during balancing. 
pub const DEFAULT_CALCULATE_BALANCE: bool = false;
/// Default - Template - Column order. 
pub const DEFAULT_TEMPLATE_COLUMNS: &str = 
    "Type~65|Date~50|Date-expr~30|Sort~30|Value~70|Value-expr~30|Periods~50|Periods-expr~30|Frequency~60|Intervals~50|End-date~50|Skip-periods~50|Parameter-list~40|Descriptor-list~40|Event-name~60|Next-name~60";
/// Default - Event - Column order. 
pub const DEFAULT_EVENT_COLUMNS: &str = 
    "Type~65|Date~50|Value~70|Periods~50|Frequency~60|Intervals~50|End-date~50|Skip-periods~50|Parameter-list~40|Descriptor-list~40";
/// Default - Amortization - Column order. 
pub const DEFAULT_AM_COLUMNS: &str = 
    "Sequence~30|Type~65|Date~50|Decrease~70|Increase~70|Interest~70|Value-to-interest~70|Value-to-principal~70|Balance~70|Accrued-balance~70|Frequency~60|Intervals~50|Parameter-list~40|Descriptor-list~40";
/// Default sort - Principal change. 
pub const DEFAULT_SORT_PRINCIPAL_CHANGE: usize = 10;
/// Default sort - Current value. 
pub const DEFAULT_SORT_CURRENT_VALUE_CHANGE: usize = 20;
/// Default sort - Statistic value. 
pub const DEFAULT_SORT_STATISTIC_VALUE_CHANGE: usize = 30;
/// Default sort - Interest change. 
pub const DEFAULT_SORT_INTEREST_CHANGE: usize = 40;
/// Default - Maximum display decimal digits. 
pub const MAXIMUM_DISPLAY_DECIMAL_DIGITS: usize = 6;
/// Default - Minimum column width. 
pub const MINIMUM_COLUMN_WIDTH: usize = 20;
/// Maximum iterations for calculate interest. 
pub const MAXIMUM_ITERATIONS_CALCULATE_INTEREST: usize = 30;
/// Maximum iterations for calculate periods. 
pub const MAXIMUM_ITERATIONS_CALCULATE_PERIODS: usize = 30;
/// Maximum iterations for calculate principal. 
pub const MAXIMUM_ITERATIONS_CALCULATE_PRINCIPAL: usize = 30;
/// Maximum iterations for calculate yield. 
pub const MAXIMUM_ITERATIONS_CALCULATE_YIELD: usize = 30;

/// Maximum calculated principal (can be increased).
pub const MAX_CALC_PRINCIPAL: &str = "1000000000000.0";
/// Maximum calculated interest (can be increased).
pub const MAX_CALC_INTEREST: &str = "2000.0";
/// Low round value. 
pub const LOW_ROUND: &str = "0.49999999";
/// Mid round value. 
pub const MID_ROUND: &str = "0.5";
/// High round value. 
pub const HIGH_ROUND: &str = "0.50000001";

/// Column labels - Sequence. 
pub const COL_LABEL_SEQUENCE: &str = "Col_Label_Sequence";
/// Column labels - Event type. 
pub const COL_LABEL_EVENT_TYPE: &str = "Col_Label_Event_Type";
/// Column labels - Date. 
pub const COL_LABEL_DATE: &str = "Col_Label_Date";
/// Column labels - Date expression. 
pub const COL_LABEL_DATE_EXPR: &str = "Col_Label_Date_Expr";
/// Column labels - Sort. 
pub const COL_LABEL_SORT: &str = "Col_Label_Sort";
/// Column labels - Value. 
pub const COL_LABEL_VALUE: &str = "Col_Label_Value";
/// Column labels - Value expression. 
pub const COL_LABEL_VALUE_EXPR: &str = "Col_Label_Value_Expr";
/// Column labels - Decrease. 
pub const COL_LABEL_DECREASE: &str = "Col_Label_Decrease";
/// Column labels - Increase. 
pub const COL_LABEL_INCREASE: &str = "Col_Label_Increase";
/// Column labels - Periods. 
pub const COL_LABEL_PERIODS: &str = "Col_Label_Periods";
/// Column labels - Periods expression. 
pub const COL_LABEL_PERIODS_EXPR: &str = "Col_Label_Periods_Expr";
/// Column labels - Skip periods. 
pub const COL_LABEL_SKIP_PERIODS: &str = "Col_Label_Skip_Periods";
/// Column labels - Intervals. 
pub const COL_LABEL_INTERVALS: &str = "Col_Label_Intervals";
/// Column labels - Frequency. 
pub const COL_LABEL_FREQUENCY: &str = "Col_Label_Frequency";
/// Column labels - End date. 
pub const COL_LABEL_END_DATE: &str = "Col_Label_End_Date";
/// Column labels - Parameter list. 
pub const COL_LABEL_PARAMETER_LIST: &str = "Col_Label_Parameter_List";
/// Column labels - Descriptor list. 
pub const COL_LABEL_DESCRIPTOR_LIST: &str = "Col_Label_Descriptor_List";
/// Column labels - Compounded interest. 
pub const COL_LABEL_INTEREST: &str = "Col_Label_Interest";
/// Column labels - Straight-line interest. 
pub const COL_LABEL_SL_INTEREST: &str = "Col_Label_SL_Interest";
/// Column labels - Interest on interest. 
pub const COL_LABEL_INT_ON_INTEREST: &str = "Col_Label_Int_On_Interest";
/// Column labels - Value-to-interest. 
pub const COL_LABEL_VALUE_TO_INTEREST: &str = "Col_Label_Value_To_Interest";
/// Column labels - Value-to-principal. 
pub const COL_LABEL_VALUE_TO_PRINCIPAL: &str = "Col_Label_Value_To_Principal";
/// Column labels - Accrued-balance. 
pub const COL_LABEL_ACCRUED_BALANCE: &str = "Col_Label_Accrued_Balance";
/// Column labels - Balance. 
pub const COL_LABEL_BALANCE: &str = "Col_Label_Balance";
/// Column labels - Event-name. 
pub const COL_LABEL_EVENT_NAME: &str = "Col_Label_Event_Name";
/// Column labels - Next-name. 
pub const COL_LABEL_NEXT_NAME: &str = "Col_Label_Next_Name";

/// Group - General. 
pub const GROUP_GENERAL: &str = "General";
/// Group - Font. 
pub const GROUP_FONT: &str = "Font";
/// Group - Column header. 
pub const GROUP_COLHEADER: &str = "ColHeader";
/// Group - Column value. 
pub const GROUP_COLVALUE: &str = "ColValue";
/// Group - Template. 
pub const GROUP_TEMPLATE: &str = "Template";
/// Group - Event. 
pub const GROUP_EVENT: &str = "Event";
/// Group - Amortization. 
pub const GROUP_AM: &str = "Amortization";
/// Group - Summary. 
pub const GROUP_SUMMARY: &str = "Summary";
/// Group - Principal change. 
pub const GROUP_PRINCIPAL_CHANGE: &str = "PrinChange";
/// Group - Current value. 
pub const GROUP_CURRENT_VALUE: &str = "CurValue";
/// Group - Statistic value. 
pub const GROUP_STATISTIC_VALUE: &str = "StatValue";
/// Group - Interest change. 
pub const GROUP_INTEREST_CHANGE: &str = "IntChange";
/// Group - Color. 
pub const GROUP_COLOR: &str = "Color";
/// Name - Font - Primary. 
pub const NAME_FONT_PRIMARY: &str = "Primary";
/// Name - Font - Alternate 2. 
pub const NAME_FONT_ALTERNATE2: &str = "Alternate2";
/// Name - Font - Alternate 3. 
pub const NAME_FONT_ALTERNATE3: &str = "Alternate3";
/// Name - Font - Caption. 
pub const NAME_FONT_CAPTION: &str = "Caption";
/// Name - Font - Statistics. 
pub const NAME_FONT_STATISTICS: &str = "Statistics";
/// Name - Font - Summary. 
pub const NAME_FONT_SUMMARY: &str = "Summary";
/// Name - Column order. 
pub const NAME_COLUMNS: &str = "Columns";
/// Name - Grid. 
pub const NAME_GRID: &str = "Grid";
/// Name - Event type. 
pub const NAME_EVENT_TYPE: &str = "EventType";
/// Name - Status. 
pub const NAME_STATUS: &str = "Status";
/// Name - Summary. 
pub const NAME_SUMMARY: &str = "Summary";
/// Name - Sequence. 
pub const NAME_SEQUENCE: &str = "Sequence";
/// Name - Margin. 
pub const NAME_MARGIN: &str = "Margin";
/// Name - Header - First. 
pub const NAME_HEADER_FIRST: &str = "HeaderFirst";
/// Name - Header - Next. 
pub const NAME_HEADER_NEXT: &str = "HeaderNext";
/// Name - Footer - First. 
pub const NAME_FOOTER_FIRST: &str = "FooterFirst";
/// Name - Footer - Next. 
pub const NAME_FOOTER_NEXT: &str = "FooterNext";
/// Type - Locale. 
pub const TYPE_LOCALE: &str = "locale";
  
/// Resource - User event type - Current value (default). 
pub const USER_EVENT_TYPE_CURRENT_VALUE: &str = "User_Event_Type_Current_Value";
/// Resource - User event type - Interest change (default). 
pub const USER_EVENT_TYPE_INTEREST_CHANGE: &str = "User_Event_Type_Interest_Change";
/// Resource - User event type - Principal change (default). 
pub const USER_EVENT_TYPE_PRINCIPAL_CHANGE: &str = "User_Event_Type_Principal_Change";
/// Resource - User event type - Statistic value (default). 
pub const USER_EVENT_TYPE_STATISTIC_VALUE: &str = "User_Event_Type_Statistic_Value";
/// Resource - New name. 
pub const USER_NEW: &str = "User_New";
/// Resource - Stat name. 
pub const USER_STAT: &str = "User_Stat";
/// Resource - User status (default). 
pub const USER_STATUS: &str = "User_Status";

/// Parameter - Description 
pub const PARAM_DESCRIPTION: &str = "strDescription";

/// Mapping for column names - editable. 
pub const MAPCOLNAMES_EDITABLE: usize = 1;
/// Mapping for column names - empty column. 
pub const MAPCOLNAMES_EMPTY: usize = 2;
/// Mapping for column names - exclude column. 
pub const MAPCOLNAMES_EXCLUDE: usize = 4;

/// Serialize user preferences.
pub const JSON_SERIALIZE_PREFERENCES: usize = 1;
/// Serialize templates.
pub const JSON_SERIALIZE_TEMPLATES: usize = 2;
/// Serialize exchange rates.
pub const JSON_SERIALIZE_EXCHANGE_RATES: usize = 4;
/// Serialize cashflow preferences.
pub const JSON_SERIALIZE_CASHFLOW_PREFERENCES: usize = 8;
/// Serialize cashflows with event list.
pub const JSON_SERIALIZE_EVENT_LIST: usize = 16;
/// Serialize cashflows with amortization list and balance results
pub const JSON_SERIALIZE_AMORTIZATION_LIST: usize = 32;
/// Serialize cashflows with amortization list (with rollup elements)
pub const JSON_SERIALIZE_AMORTIZATION_LIST_ROLLUPS: usize = 64;
/// Serialize cashflows with amortization list (with rollup and detail elements)
pub const JSON_SERIALIZE_AMORTIZATION_LIST_DETAILS: usize = 128;

/// Json line ending. 
pub const LINE_ENDING: &str = "\n";
/// Visible delimiter. 
pub const DELIMITER_VISIBLE: &str = "|";
/// Spaces per tab. 
pub const TAB_SPACES: usize = 4;