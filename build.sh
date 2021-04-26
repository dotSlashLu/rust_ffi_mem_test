pushd rust && cargo build && popd
gcc -g -Wall -ldl -lpthread -o mem_test mem_test.c ./rust/target/debug/librust.a

