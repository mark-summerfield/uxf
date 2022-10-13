// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#ifndef CONSTS_HPP
#define CONSTS_HPP

#include <string_view>
using namespace ::std::literals::string_view_literals;

namespace uxf {

static const int UXF_VERSION = 1;

constexpr inline auto VERSION = "0.1.0";
constexpr inline auto VALUE_NAME_NULL = "null"sv;
constexpr inline auto VTYPE_NAME_BOOL = "bool"sv;
constexpr inline auto VTYPE_NAME_BYTES = "bytes"sv;
constexpr inline auto VTYPE_NAME_DATE = "date"sv;
constexpr inline auto VTYPE_NAME_DATETIME = "datetime"sv;
constexpr inline auto VTYPE_NAME_INT = "int"sv;
constexpr inline auto VTYPE_NAME_LIST = "list"sv;
constexpr inline auto VTYPE_NAME_MAP = "map"sv;
constexpr inline auto VTYPE_NAME_REAL = "real"sv;
constexpr inline auto VTYPE_NAME_STR = "str"sv;
constexpr inline auto VTYPE_NAME_TABLE = "table"sv;

};

#endif // CONSTS_HPP
