#include "myclass.hpp"


MyClass::MyClass(int i) {
        val = i;
}

void MyClass::set(int i) {
        val = i;
}

int MyClass::get() {
        return val;
}