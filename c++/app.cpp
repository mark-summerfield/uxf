#include <cstdio>
#include "base.hpp"

int main() {
    auto lst = new ListValue();
    printf("app v%d %p\n", version(), lst);
    return 0;
}
