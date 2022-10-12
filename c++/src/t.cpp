// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#include <iostream>
#include "uxf.hpp"

using namespace std;

int main() {
    auto v = new uxf::IntValue();
    cout << "int = " << v << endl;
    auto lst = new uxf::ListValue();
    cout << "list size = " << lst->size() << ' ' << lst << endl;
}
