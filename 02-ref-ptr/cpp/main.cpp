#include <iostream>

extern "C" void pass_ref(int&);
extern "C" void pass_ptr(int*);

int main(void) {                    
    {                               
        int a = 0;                  
        pass_ref(a);                
        std::cout << a << std::endl;
    }                               

    {                               
        int a = 0;                  
        pass_ptr(&a);               
        std::cout << a << std::endl;
    }
    
    /* 
       Segmentation fault (null dereference)
       int* pa = 0;
       pass_ptr(pa);
       std::cout << *pa << std::endl;
    */
}
