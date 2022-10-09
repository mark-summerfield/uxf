#ifndef LIB_HPP
#define LIB_HPP

int version(); // TODO delete: use a const instead

class BaseValue;
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

class BaseValue {
    // TODO operator< operator==
};

class CollectionValue : BaseValue {
};

class MapValue : CollectionValue {
};

class SerialValue : CollectionValue {
};

class ListValue: SerialValue {
};

class TableValue: SerialValue {
};

class ScalarValue : BaseValue {
};

class NullValue : ScalarValue {
};

class KeyValue : ScalarValue {
};

class IntValue : KeyValue {
};

class StrValue : KeyValue {
};
#endif // LIB_HPP
