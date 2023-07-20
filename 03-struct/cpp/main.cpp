#include <iostream>

struct Coordinate {
    int x;
    int y;
};

extern "C" Coordinate flip(Coordinate);

int main(void) {
    auto flipped = flip(Coordinate { .x = 3, .y = 5 });
    std::cout << flipped.x << ", " << flipped.y << std::endl;
}
