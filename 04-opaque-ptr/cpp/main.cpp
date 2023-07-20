extern "C" {
struct Coordinate;

void create(Coordinate**);
void print(Coordinate*);
void destroy(Coordinate*);
}

int main(void) {     
    Coordinate* coo = nullptr;
    create(&coo);   
    print(coo);     
    destroy(coo);   
}
