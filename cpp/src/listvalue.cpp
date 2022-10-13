// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#include "value.hpp"

namespace uxf {

bool ListValue::empty() const {
    return true; // TODO
}

void ListValue::push(Value *value) {
    throw Error("ListValue::push() not implemented"); // TODO
}

size_t ListValue::size() const {
    return 0; // TODO
}

}
