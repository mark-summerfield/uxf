#ifndef VALUE_HPP
#define VALUE_HPP

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
class IntValue;
class StrValue;

class Value {
};

class CollectionValue : Value {
};

class MapValue : CollectionValue {
};

class SerialValue : CollectionValue {
};

class ListValue: SerialValue {
};

class TableValue: SerialValue {
};

class ScalarValue : Value {
};

class NullValue : Value {
};

class KeyValue : ScalarValue {
};

class IntValue : KeyValue {
};

class StrValue : KeyValue {
};

}
#endif // VALUE_HPP
