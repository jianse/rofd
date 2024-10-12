$RUSTFLAGS="-Cinstrument-coverage"

cargo build

$LLVM_PROFILE_FILE="your_name-%p-%m.profraw"

cargo test

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
