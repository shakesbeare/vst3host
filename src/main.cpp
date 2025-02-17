#include "iostream"
#include "pluginterfaces/vst/vsttypes.h"

typedef struct MyStruct {
    int a;
    int b;
};

int main() {
    MyStruct foo;
    foo.a = 32;
    foo.b = 34;

    std::cout << "Hello, World!" << std::endl;
    std::cout << "Value of foo.a is " << foo.a << std::endl;
    std::cout << "Value of foo.b is " << foo.b << std::endl;
}
