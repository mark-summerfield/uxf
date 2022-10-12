#include <iostream>
#include "uxf.hpp"

using namespace std;

int main() {
    auto v = new uxf::Value();
    try {
        cout << "t " << v->size() << endl;
    } catch (uxf::Error& err) {
        cout << "error: " << err.what() << endl;
    }
    auto lst = new uxf::ListValue();
    cout << "list size = " << lst->size() << endl;
}
