// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#include "value.hpp"

namespace uxf {

bool MapValue::empty() const {
    return true; // TODO
}

void MapValue::push(Value *value) {
    throw Error("MapValue::push() not implemented"); // TODO
}

size_t MapValue::size() const {
    return 0; // TODO
}

}
