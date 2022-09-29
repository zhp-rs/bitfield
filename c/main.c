#include <stdio.h>

struct Pa {
    unsigned char a1: 1;
    unsigned char a2: 2;
    unsigned char a5: 5;
    unsigned char b1: 1;
    unsigned char b2: 1;
    unsigned char b6: 6;
    short s;
};

int main(int argc, char const *argv[])
{
	struct Pa da = {1, 3, 9, 1, 0, 0x2d, 0x1122};
	unsigned int *pint = (unsigned int *)&da;

    printf("{ a1: %u, a2: %u, a5: %u, b1: %u, b2: %u, b6: %u, s: %d }\n", 
    da.a1, da.a2, da.a5, da.b1, da.b2, da.b6, da.s
    );
	printf("int: %#x\n", *pint);
	return 0;
}

