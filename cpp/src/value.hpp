// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#ifndef VALUE_HPP
#define VALUE_HPP

#include <string_view>
#include "consts.hpp"
#include "error.hpp"

using namespace std;

namespace uxf {

class Value;
class NullValue;
class CollectionValue;
class MapValue;
class SerialValue;
class ListValue;
class TableValue;
class ScalarValue;
class BoolValue;
class RealValue;
class KeyValue;
class BytesValue;
class DateValue;
class DateTimeValue;
class IntValue;
class StrValue;

Value* naturalize(const char* s);

class Value {
public:
    virtual ~Value() {}
    virtual const string_view uxf_typename() const = 0;
};

class NullValue : public Value {
public:
    const string_view uxf_typename() const {
        return VALUE_NAME_NULL;
    }
};

class CollectionValue : public Value {
    virtual bool empty() const = 0;
    virtual size_t size() const = 0;
    virtual void push(Value *value) = 0;
    // TODO push_many()
};

class MapValue : public CollectionValue {
public:
    bool empty() const;
    void push(Value *value);
    size_t size() const;
    const string_view uxf_typename() const {
        return VTYPE_NAME_MAP;
    }
};

class SerialValue : public CollectionValue {
};

class ListValue: public SerialValue {
public:
    bool empty() const;
    void push(Value *value);
    size_t size() const;
    const string_view uxf_typename() const {
        return VTYPE_NAME_LIST;
    }
};

class TableValue: public SerialValue {
public:
    bool empty() const;
    void push(Value *value);
    size_t size() const;
    const string_view uxf_typename() const {
        return VTYPE_NAME_TABLE;
    }
};

}

#endif // VALUE_HPP
