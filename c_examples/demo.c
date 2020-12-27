#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "structs.c"

int main()
{
    printf("\n..starting..\n");

    int a = 10;
    int *ref_a = &a;
    printf("%x\n", ref_a);
    printf("%d\n", *ref_a);

    struct User User1;
    strcpy(User1.name, "Jon");
    User1.age = 30;

    struct User User2;
    strcpy(User2.name, "Doe");
    User2.age = 35;

    
    printf("\n..done3\n");
}