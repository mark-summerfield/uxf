#include <cstdio>
#include "uxf.hpp"

int main() {
    auto lst = new uxf::ListValue();
    printf("uxf v%s (uxf %d) %p\n", uxf::VERSION, uxf::UXF_VERSION, lst);
    return 0;
}
