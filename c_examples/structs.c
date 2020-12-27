
struct User
{
    char name[50];
    int age;
};

void struct_demo()
{
    struct User User1;
    strcpy(User1.name, "Jon");
    User1.age = 30;

    struct User User2;
    strcpy(User2.name, "Doe");
    User2.age = 35;

    printf("\nUser1: %s", User1.name);
    printf("\nUser2: %s", User2.name);
}