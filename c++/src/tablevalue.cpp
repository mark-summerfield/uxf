// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#include "value.hpp"

namespace uxf {

bool TableValue::empty() const {
    return true; // TODO
}

void TableValue::push(Value *value) {
    throw Error("TableValue::push() not implemented"); // TODO
}

size_t TableValue::size() const {
    return 0; // TODO
}

}
