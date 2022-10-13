// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#ifndef SCALARVALUE_HPP
#define SCALARVALUE_HPP

#include "value.hpp"

namespace uxf {

class ScalarValue : public Value {
public:
};

class BoolValue : public ScalarValue {
public:
    const string_view uxf_typename() const { return VTYPE_NAME_BOOL; }
};

class RealValue : public ScalarValue {
public:
    const string_view uxf_typename() const { return VTYPE_NAME_REAL; }
};

class KeyValue : public ScalarValue {
public:
};

class BytesValue : public KeyValue {
public:
    const string_view uxf_typename() const { return VTYPE_NAME_BYTES; }
};

class DateValue : public KeyValue {
public:
    const string_view uxf_typename() const { return VTYPE_NAME_DATE; }
};

class DateTimeValue : public KeyValue {
public:
    const string_view uxf_typename() const { return VTYPE_NAME_DATETIME; }
};

class IntValue : public KeyValue {
public:
    const string_view uxf_typename() const { return VTYPE_NAME_INT; }
};

class StrValue : public KeyValue {
public:
    const string_view uxf_typename() const { return VTYPE_NAME_STR; }
};

}

#endif // SCALARVALUE_HPP
