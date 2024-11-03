# save env vars
$OLD_RUSTC_BOOTSTRAP=$env:RUSTC_BOOTSTRAP
$OLD_CARGO_INCREMENTAL=$env:CARGO_INCREMENTAL
$OLD_RUSTFLAGS=$env:RUSTFLAGS
$OLD_LLVM_PROFILE_FILE=$env:LLVM_PROFILE_FILE
$OLD_RUSTDOCFLAGS=$env:RUSTDOCFLAGS


$env:RUSTFLAGS="-Cinstrument-coverage"

$PROFILE_DIR="${pwd}/target/cov/grcov"

$env:LLVM_PROFILE_FILE="$PROFILE_DIR/grcov-%p-%m.profraw"

rm -Recurse $PROFILE_DIR
# $env:RUSTC_BOOTSTRAP=1
# $env:CARGO_INCREMENTAL=0
# $env:RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
# $env:RUSTDOCFLAGS="-Cpanic=abort"
# cargo clean
cargo build --profile cov

cargo test --profile cov --workspace

grcov $PROFILE_DIR -s . --binary-path ./target/cov/ -t html --branch  -o ./target/cov/coverage/ --llvm


# restore env vars
$env:RUSTC_BOOTSTRAP=$OLD_RUSTC_BOOTSTRAP
$env:CARGO_INCREMENTAL=$OLD_CARGO_INCREMENTAL
$env:RUSTFLAGS=$OLD_RUSTFLAGS
$env:LLVM_PROFILE_FILE=$OLD_LLVM_PROFILE_FILE
$env:RUSTDOCFLAGS=$OLD_RUSTDOCFLAGS
