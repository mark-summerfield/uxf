#ifndef VALUE_HPP
#define VALUE_HPP

#include "err.hpp"

using namespace std;

namespace uxf {

class Value;
class CollectionValue;
class MapValue;
class SerialValue;
class ListValue;
class TableValue;
class ScalarValue;
class NullValue;
class KeyValue;
class DateValue;
class DateTimeValue;
class IntValue;
class StrValue;

class Value {
public:
    virtual ~Value() {}

    virtual size_t size() const {
        throw Error("scalars don't have a size");
    }
};

class CollectionValue : public Value {
};

class MapValue : public CollectionValue {
};

class SerialValue : public CollectionValue {
};

class ListValue: public SerialValue {
public:
    size_t size() const {
        return 0; // TODO
    }
};

class TableValue: public SerialValue {
};

class ScalarValue : public Value {
};

class NullValue : public Value {
};

class KeyValue : public ScalarValue {
};

class DateValue : public KeyValue {
};

class DateTimeValue : public KeyValue {
};

class IntValue : public KeyValue {
};

class StrValue : public KeyValue {
};

}
#endif // VALUE_HPP
