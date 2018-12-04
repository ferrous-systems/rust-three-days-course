# Run cargo rustc -- --print native-static-libs to get this list

cargo build --release

gcc test/test.c target/release/libpoint.a -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread -lutil -I include -o point

./point
