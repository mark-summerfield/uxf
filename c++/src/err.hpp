// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

#ifndef ERR_HPP
#define ERR_HPP

#include <exception>

using namespace std;

namespace uxf {

class Error : public std::exception {
public:
    Error(const char* message) : message_(message) {}
    const char* what() const throw() { return message_; }
private:
    const char* message_;
};

}
#endif // ERR_HPP
