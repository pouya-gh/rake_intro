system("gcc -fPIC -c greetings.c -I ./include/")
system("gcc -fPIC -c farewells.c -I ./include/")
system("gcc -fPIC -c main.c -I ./include/")
system("gcc *.o -o main -I ./include/")
