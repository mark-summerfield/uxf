// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#ifndef ERROR_HPP
#define ERROR_HPP

#include <stdexcept>

using namespace std;

namespace uxf {

class Error : public std::runtime_error {
public:
    Error(const char* message) : std::runtime_error(message) {}
};

}

#endif // ERROR_HPP
