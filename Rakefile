task :exe => 'main'

file 'main' => ['farewells.o', 'greetings.o', 'main.o'] do
  sh "gcc farewells.o greetings.o main.o -o main -I ./include/"
end

file "greetings.o" => ["greetings.c", "./include/greetings.h"] do
    sh "gcc -fPIC -c greetings.c -I ./include/"
end

file "farewells.o" => ["farewells.c", "./include/farewells.h"] do
  sh "gcc -fPIC -c farewells.c -I ./include/"
end

file "main.o" => "main.c" do
  sh "gcc -fPIC -c main.c -I ./include/"
end
