// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#ifndef CONSTS_HPP
#define CONSTS_HPP

#include <string_view>

using namespace std::literals;

namespace uxf {

const string_view VALUE_NAME_NULL = "null"sv;
const string_view VTYPE_NAME_BOOL = "bool"sv;
const string_view VTYPE_NAME_BYTES = "bytes"sv;
const string_view VTYPE_NAME_DATE = "date"sv;
const string_view VTYPE_NAME_DATETIME = "datetime"sv;
const string_view VTYPE_NAME_INT = "int"sv;
const string_view VTYPE_NAME_LIST = "list"sv;
const string_view VTYPE_NAME_MAP = "map"sv;
const string_view VTYPE_NAME_REAL = "real"sv;
const string_view VTYPE_NAME_STR = "str"sv;
const string_view VTYPE_NAME_TABLE = "table"sv;

};

#endif // CONSTS_HPP
