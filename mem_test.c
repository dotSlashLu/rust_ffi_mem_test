#include <stdio.h>

// a CStr from rust, what about the memory?
char *a_string();
char *a_string2();

int main(int argc, char **argv) 
{
    char *ret = a_string();
    printf("str from rust, ptr: %p content: %s\n", ret, ret);

    ret = a_string2();
    printf("str2 from rust, ptr: %p content: %s\n", ret, ret);
}


