# save env vars
$OLD_RUSTC_BOOTSTRAP=$env:RUSTC_BOOTSTRAP
$OLD_CARGO_INCREMENTAL=$env:CARGO_INCREMENTAL
$OLD_RUSTFLAGS=$env:RUSTFLAGS
$OLD_LLVM_PROFILE_FILE=$env:LLVM_PROFILE_FILE
$OLD_RUSTDOCFLAGS=$env:RUSTDOCFLAGS


$env:RUSTFLAGS="-Cinstrument-coverage"
$env:LLVM_PROFILE_FILE="./target/debug/grcov/grvov-%p-%m.profraw"

# $env:RUSTC_BOOTSTRAP=1
# $env:CARGO_INCREMENTAL=0
# $env:RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
# $env:RUSTDOCFLAGS="-Cpanic=abort"
cargo clean
cargo build

cargo test

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing --ignore "/*" -o ./target/debug/coverage/ --llvm


# restore env vars
$env:RUSTC_BOOTSTRAP=$OLD_RUSTC_BOOTSTRAP
$env:CARGO_INCREMENTAL=$OLD_CARGO_INCREMENTAL
$env:RUSTFLAGS=$OLD_RUSTFLAGS
$env:LLVM_PROFILE_FILE=$OLD_LLVM_PROFILE_FILE
$env:RUSTDOCFLAGS=$OLD_RUSTDOCFLAGS
