#ifndef ERR_HPP
#define ERR_HPP

#include <exception>

using namespace std;

namespace uxf {

class Error : public std::exception {
public:
    Error(const char* message) : m_message(message) {}
    const char* what() const throw() { return m_message; }
private:
    const char* m_message;
};

}
#endif // ERR_HPP
