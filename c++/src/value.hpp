// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#ifndef VALUE_HPP
#define VALUE_HPP

#include "consts.hpp"
#include "err.hpp"

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
class RealValue;
class KeyValue;
class DateValue;
class DateTimeValue;
class IntValue;
class StrValue;

Value* naturalize(const char* s);

class Value {
public:
    virtual ~Value() {}
    virtual const char* uxf_typename() const;
};

class NullValue : public Value {
public:
    const char* uxf_typename() const { return VALUE_NAME_NULL; }
};

class CollectionValue : public Value {
    virtual size_t size() const = 0;
    virtual void push(Value *value) = 0;
    // TODO push_many()
};

class MapValue : public CollectionValue {
public:
    bool empty() const;
    void push(Value *value);
    size_t size() const;
    const char* uxf_typename() const { return VTYPE_NAME_MAP; }
};

class SerialValue : public CollectionValue {
};

class ListValue: public SerialValue {
public:
    bool empty() const;
    void push(Value *value);
    size_t size() const;
    const char* uxf_typename() const { return VTYPE_NAME_LIST; }
};

class TableValue: public SerialValue {
public:
    bool empty() const;
    void push(Value *value);
    size_t size() const;
    const char* uxf_typename() const { return VTYPE_NAME_TABLE; }
};

class ScalarValue : public Value {
public:
};

class RealValue : public ScalarValue {
public:
    const char* uxf_typename() const { return VTYPE_NAME_REAL; }
};

class KeyValue : public ScalarValue {
public:
};

class DateValue : public KeyValue {
public:
    const char* uxf_typename() const { return VTYPE_NAME_DATE; }
};

class DateTimeValue : public KeyValue {
public:
    const char* uxf_typename() const { return VTYPE_NAME_DATETIME; }
};

class IntValue : public KeyValue {
public:
    const char* uxf_typename() const { return VTYPE_NAME_INT; }
};

class StrValue : public KeyValue {
public:
    const char* uxf_typename() const { return VTYPE_NAME_STR; }
};

}
#endif // VALUE_HPP
