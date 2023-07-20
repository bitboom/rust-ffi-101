#include <iostream>

struct MyCallback {
    void (*cb)(void);
};

void hello(void) {
	std::cout << "Something wrong!" << std::endl;
}

extern "C" void call(MyCallback it);

int main() {
	MyCallback wrong = {
		hello,
	};

    call(wrong);
    return 0;
}
